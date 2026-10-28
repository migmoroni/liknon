#!/bin/sh
set -eu

if [ "$#" -ne 3 ]; then
  printf '%s\n' "Usage: install-aws-cli.sh <version> <zip-sha256> <prefix>" >&2
  exit 2
fi

version=$1
expected_sha=$2
prefix=$3

case "$version" in
  *[!0-9.]*|'') printf '%s\n' "invalid AWS CLI version" >&2; exit 1 ;;
esac
case "$expected_sha" in
  *[!0-9a-fA-F]*|'') printf '%s\n' "invalid AWS CLI ZIP SHA-256" >&2; exit 1 ;;
esac
[ "${#expected_sha}" -eq 64 ] || { printf '%s\n' "AWS CLI ZIP SHA-256 must contain 64 hexadecimal characters" >&2; exit 1; }

tmp_dir=$(mktemp -d "${TMPDIR:-/tmp}/aws-cli-install.XXXXXX")
cleanup() { rm -rf -- "$tmp_dir"; }
trap cleanup EXIT HUP INT TERM

archive="$tmp_dir/awscliv2.zip"
curl --fail --silent --show-error --location \
  --proto '=https' --proto-redir '=https' --tlsv1.2 \
  --output "$archive" \
  "https://awscli.amazonaws.com/awscli-exe-linux-x86_64-$version.zip"
actual_sha=$(openssl dgst -sha256 -r "$archive" | awk '{print $1}')
[ "$actual_sha" = "$(printf '%s' "$expected_sha" | tr 'A-F' 'a-f')" ] \
  || { printf '%s\n' "AWS CLI archive hash mismatch" >&2; exit 1; }
unzip -q "$archive" -d "$tmp_dir"
mkdir -p "$prefix/bin"
"$tmp_dir/aws/install" --install-dir "$prefix/aws-cli" --bin-dir "$prefix/bin"
"$prefix/bin/aws" --version 2>&1 | grep -F "aws-cli/$version" >/dev/null \
  || { printf '%s\n' "installed AWS CLI version mismatch" >&2; exit 1; }
