#!/bin/sh
set -eu

if [ "$#" -ne 4 ]; then
  printf '%s\n' "Usage: publish-r2.sh <product.json> <identity.json> <direct-root> <state.json>" >&2
  exit 2
fi

descriptor=$1
identity=$2
root=$3
state=$4

: "${R2_BUCKET:?R2_BUCKET is required}"
: "${R2_ENDPOINT_URL:?R2_ENDPOINT_URL is required}"
: "${LIKNON_DOWNLOAD_BASE_URL:?LIKNON_DOWNLOAD_BASE_URL is required}"
: "${AWS_CLI_EXPECTED_VERSION:?AWS_CLI_EXPECTED_VERSION is required}"

case "$LIKNON_DOWNLOAD_BASE_URL" in
  https://*) ;;
  *) printf '%s\n' "public R2 origin must use HTTPS" >&2; exit 1 ;;
esac
case "$R2_ENDPOINT_URL" in
  https://*) ;;
  *) printf '%s\n' "private R2 endpoint must use HTTPS" >&2; exit 1 ;;
esac

for program in aws curl jq openssl; do
  command -v "$program" >/dev/null 2>&1 || { printf '%s\n' "missing required program: $program" >&2; exit 1; }
done
aws --version 2>&1 | grep -F "aws-cli/$AWS_CLI_EXPECTED_VERSION" >/dev/null \
  || { printf '%s\n' "unexpected AWS CLI version" >&2; exit 1; }

package=$(jq -er '.product.cargoPackage' "$descriptor")
release_prefix=$(jq -er '.distribution.releasePrefix' "$descriptor")
version=$(jq -er '.version' "$identity")
[ "$(jq -er '.package' "$identity")" = "$package" ] || { printf '%s\n' "identity package mismatch" >&2; exit 1; }
version_prefix="$release_prefix/v$version"

tmp_dir=$(mktemp -d "${TMPDIR:-/tmp}/liknon-r2-publish.XXXXXX")
current_destination=
current_sha=
update_state() {
  destination=$1
  status=$2
  sha=$3
  detail=${4:-}
  next="$tmp_dir/state.next"
  jq \
    --arg destination "$destination" \
    --arg status "$status" \
    --arg sha "$sha" \
    --arg detail "$detail" \
    '.destinations[$destination] = ({status: $status, sha256: $sha} + if $detail == "" then {} else {detail: $detail} end)' \
    "$state" > "$next"
  mv "$next" "$state"
}
cleanup() {
  status=$?
  trap - EXIT HUP INT TERM
  if [ "$status" -ne 0 ] && [ -n "$current_destination" ]; then
    update_state "$current_destination" failed "$current_sha" "publication or public readback failed" || true
  fi
  rm -rf -- "$tmp_dir"
  exit "$status"
}
trap cleanup EXIT HUP INT TERM

mkdir -p "$(dirname -- "$state")"
jq -n --slurpfile identity "$identity" \
  '{schemaVersion: 1, identity: $identity[0], destinations: {}}' > "$state"

expected="$tmp_dir/expected"
{
  printf '%s\n' manifest.sig SHA256SUMS install.sh install.sh.sig install.ps1 install.ps1.sig
  jq -r '.artifacts[].archive.path' "$root/manifest.json"
} | sort -u > "$expected"

while IFS= read -r name; do
  [ -f "$root/$name" ] || { printf '%s\n' "missing direct object: $name" >&2; exit 1; }
done < "$expected"

content_type() {
  case "$1" in
    *.json) printf '%s' 'application/json' ;;
    *.sig) printf '%s' 'application/octet-stream' ;;
    *.sh) printf '%s' 'text/x-shellscript; charset=utf-8' ;;
    *.ps1) printf '%s' 'text/plain; charset=utf-8' ;;
    *.zip) printf '%s' 'application/zip' ;;
    *.tar.gz) printf '%s' 'application/gzip' ;;
    *) printf '%s' 'text/plain; charset=utf-8' ;;
  esac
}

publish_one() {
  name=$1
  file=$root/$name
  key=$version_prefix/$name
  expected_sha=$(openssl dgst -sha256 -r "$file" | awk '{print $1}')
  current_destination="cloudflare-r2:$key"
  current_sha=$expected_sha
  update_state "$current_destination" pending "$expected_sha"
  existing="$tmp_dir/existing"
  if aws --endpoint-url "$R2_ENDPOINT_URL" s3api head-object --bucket "$R2_BUCKET" --key "$key" >/dev/null 2>&1; then
    aws --endpoint-url "$R2_ENDPOINT_URL" s3api get-object --bucket "$R2_BUCKET" --key "$key" "$existing" >/dev/null
    existing_sha=$(openssl dgst -sha256 -r "$existing" | awk '{print $1}')
    [ "$existing_sha" = "$expected_sha" ] \
      || { printf '%s\n' "immutable R2 conflict: $key" >&2; exit 1; }
  else
    aws --endpoint-url "$R2_ENDPOINT_URL" s3api put-object \
      --bucket "$R2_BUCKET" \
      --key "$key" \
      --body "$file" \
      --content-type "$(content_type "$name")" \
      --cache-control 'public,max-age=31536000,immutable' \
      --if-none-match '*' >/dev/null
  fi
  update_state "$current_destination" published "$expected_sha"
  public="$tmp_dir/public"
  curl --fail --silent --show-error --max-redirs 0 \
    --proto '=https' --proto-redir '=https' --tlsv1.2 \
    --output "$public" \
    "${LIKNON_DOWNLOAD_BASE_URL%/}/$key?readback=${GITHUB_RUN_ID:-local}"
  public_sha=$(openssl dgst -sha256 -r "$public" | awk '{print $1}')
  [ "$public_sha" = "$expected_sha" ] \
    || { printf '%s\n' "public R2 readback mismatch: $key" >&2; exit 1; }
  update_state "$current_destination" verified "$expected_sha"
  current_destination=
  current_sha=
}

while IFS= read -r name; do
  publish_one "$name"
done < "$expected"

# The signed manifest is the immutable release completion marker.
publish_one manifest.json
