#!/bin/sh
set -eu

if [ "$#" -ne 3 ]; then
  printf '%s\n' "Usage: publish-stable.sh <product.json> <stable.json> <readback.json>" >&2
  exit 2
fi

descriptor=$1
stable=$2
readback=$3

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

aws --version 2>&1 | grep -F "aws-cli/$AWS_CLI_EXPECTED_VERSION" >/dev/null \
  || { printf '%s\n' "unexpected AWS CLI version" >&2; exit 1; }
key=$(jq -er '.distribution.stablePointer' "$descriptor")
expected_sha=$(openssl dgst -sha256 -r "$stable" | awk '{print $1}')

aws --endpoint-url "$R2_ENDPOINT_URL" s3api put-object \
  --bucket "$R2_BUCKET" \
  --key "$key" \
  --body "$stable" \
  --content-type 'application/json' \
  --cache-control 'public,max-age=60,must-revalidate' >/dev/null

curl --fail --silent --show-error --max-redirs 0 \
  --proto '=https' --proto-redir '=https' --tlsv1.2 \
  --output "$readback" \
  "${LIKNON_DOWNLOAD_BASE_URL%/}/$key?readback=${GITHUB_RUN_ID:-local}"
actual_sha=$(openssl dgst -sha256 -r "$readback" | awk '{print $1}')
[ "$actual_sha" = "$expected_sha" ] || { printf '%s\n' "stable pointer readback mismatch" >&2; exit 1; }
