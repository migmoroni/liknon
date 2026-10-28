#!/bin/sh
set -eu

if [ "$#" -ne 2 ]; then
  printf '%s\n' "Usage: verify-release-tag.sh <tag> <trust-directory>" >&2
  exit 2
fi

tag=$1
trust_directory=$2
gpg_key="$trust_directory/tag-signing-public-key.asc"
ssh_signers="$trust_directory/tag-signing-allowed-signers"

if [ -f "$gpg_key" ] && [ -f "$ssh_signers" ]; then
  printf '%s\n' "configure exactly one release-tag trust mechanism" >&2
  exit 1
elif [ -f "$gpg_key" ]; then
  gpg --batch --import "$gpg_key" >/dev/null
elif [ -f "$ssh_signers" ]; then
  git config --local gpg.ssh.allowedSignersFile "$ssh_signers"
else
  printf '%s\n' "AP-1 incomplete: release-tag trust root is missing" >&2
  exit 1
fi

[ "$(git cat-file -t "refs/tags/$tag")" = tag ] \
  || { printf '%s\n' "release tag must be annotated" >&2; exit 1; }
git verify-tag "$tag"
