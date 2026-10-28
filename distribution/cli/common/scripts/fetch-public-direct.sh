#!/bin/sh
set -eu

if [ "$#" -ne 5 ]; then
  printf '%s\n' "Usage: fetch-public-direct.sh <base-url> <release-prefix> <version> <output-dir> <public-key>" >&2
  exit 2
fi

base_url=${1%/}
release_prefix=$2
version=$3
output=$4
public_key=$5
release_url="$base_url/$release_prefix/v$version"

case "$base_url" in
  https://*) download_protocol='=https' ;;
  http://127.0.0.1:*|http://localhost:*) download_protocol='=http' ;;
  *) printf '%s\n' "public download origin must use HTTPS" >&2; exit 1 ;;
esac

[ -f "$public_key" ] || { printf '%s\n' "public key not found: $public_key" >&2; exit 1; }
for program in curl jq openssl; do
  command -v "$program" >/dev/null 2>&1 \
    || { printf '%s\n' "missing required program: $program" >&2; exit 1; }
done

mkdir -p "$output"
download() {
  curl --fail --silent --show-error --max-redirs 0 \
    --proto "$download_protocol" --proto-redir "$download_protocol" \
    --tlsv1.2 --output "$2" "$1"
}
download "$release_url/manifest.json" "$output/manifest.json"
download "$release_url/manifest.sig" "$output/manifest.sig"
openssl dgst -sha256 -verify "$public_key" \
  -signature "$output/manifest.sig" "$output/manifest.json" >/dev/null

for name in $(jq -r '.artifacts[].archive.path, .checksums.path, .installers[].file.path, .installers[].signaturePath' "$output/manifest.json"); do
  case "$name" in
    ""|*/*|*..*) printf '%s\n' "unsafe public object path: $name" >&2; exit 1 ;;
  esac
  download "$release_url/$name" "$output/$name"
done
