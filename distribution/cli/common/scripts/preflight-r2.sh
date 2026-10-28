#!/bin/sh
set -eu

if [ "$#" -ne 3 ]; then
  printf '%s\n' "Usage: preflight-r2.sh <product.json> <identity.json> <direct-root>" >&2
  exit 2
fi

descriptor=$1
identity=$2
root=$3

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
  command -v "$program" >/dev/null 2>&1 \
    || { printf '%s\n' "missing required program: $program" >&2; exit 1; }
done
aws --version 2>&1 | grep -F "aws-cli/$AWS_CLI_EXPECTED_VERSION" >/dev/null \
  || { printf '%s\n' "unexpected AWS CLI version" >&2; exit 1; }

package=$(jq -er '.product.cargoPackage' "$descriptor")
release_prefix=$(jq -er '.distribution.releasePrefix' "$descriptor")
version=$(jq -er '.version' "$identity")
[ "$(jq -er '.package' "$identity")" = "$package" ] \
  || { printf '%s\n' "identity package mismatch" >&2; exit 1; }
version_prefix="$release_prefix/v$version"

tmp_dir=$(mktemp -d "${TMPDIR:-/tmp}/liknon-r2-preflight.XXXXXX")
cleanup() { rm -rf -- "$tmp_dir"; }
trap cleanup EXIT HUP INT TERM

aws --endpoint-url "$R2_ENDPOINT_URL" s3api head-bucket \
  --bucket "$R2_BUCKET" >/dev/null
curl --silent --show-error --head --max-redirs 0 \
  --proto '=https' --proto-redir '=https' --tlsv1.2 --output /dev/null \
  "${LIKNON_DOWNLOAD_BASE_URL%/}/?preflight=${GITHUB_RUN_ID:-local}"

expected="$tmp_dir/expected"
{
  printf '%s\n' manifest.json manifest.sig SHA256SUMS install.sh install.sh.sig install.ps1 install.ps1.sig
  jq -r '.artifacts[].archive.path' "$root/manifest.json"
} | sort -u > "$expected"

while IFS= read -r name; do
  file="$root/$name"
  [ -f "$file" ] || { printf '%s\n' "missing direct object: $name" >&2; exit 1; }
  key="$version_prefix/$name"
  error="$tmp_dir/head-error"
  if aws --endpoint-url "$R2_ENDPOINT_URL" s3api head-object \
    --bucket "$R2_BUCKET" --key "$key" > /dev/null 2> "$error"; then
    stored="$tmp_dir/stored"
    public="$tmp_dir/public"
    aws --endpoint-url "$R2_ENDPOINT_URL" s3api get-object \
      --bucket "$R2_BUCKET" --key "$key" "$stored" >/dev/null
    cmp "$file" "$stored" \
      || { printf '%s\n' "immutable R2 conflict: $key" >&2; exit 1; }
    curl --fail --silent --show-error --max-redirs 0 \
      --proto '=https' --proto-redir '=https' --tlsv1.2 \
      --output "$public" \
      "${LIKNON_DOWNLOAD_BASE_URL%/}/$key?preflight=${GITHUB_RUN_ID:-local}"
    cmp "$file" "$public" \
      || { printf '%s\n' "public R2 readback conflict: $key" >&2; exit 1; }
  elif ! grep -Eq '(404|Not Found|NoSuchKey)' "$error"; then
    cat "$error" >&2
    printf '%s\n' "cannot determine R2 object state: $key" >&2
    exit 1
  fi
done < "$expected"
