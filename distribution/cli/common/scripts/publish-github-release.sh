#!/bin/sh
set -eu

if [ "$#" -ne 3 ]; then
  printf '%s\n' "Usage: publish-github-release.sh <owner/repository> <tag> <notes.md>" >&2
  exit 2
fi

repository=$1
tag=$2
notes=$3

: "${GH_TOKEN:?GH_TOKEN is required}"
command -v gh >/dev/null 2>&1 || { printf '%s\n' "gh is required" >&2; exit 1; }

tmp_dir=$(mktemp -d "${TMPDIR:-/tmp}/liknon-github-release.XXXXXX")
cleanup() { rm -rf -- "$tmp_dir"; }
trap cleanup EXIT HUP INT TERM
release_json="$tmp_dir/release.json"

if gh api "repos/$repository/releases/tags/$tag" > "$release_json" 2>/dev/null; then
  [ "$(jq -r '.tag_name' "$release_json")" = "$tag" ] \
    || { printf '%s\n' "GitHub Release tag mismatch" >&2; exit 1; }
  [ "$(jq '.assets | length' "$release_json")" = 0 ] \
    || { printf '%s\n' "GitHub Release must not contain binary assets" >&2; exit 1; }
else
  gh release create "$tag" \
    --repo "$repository" \
    --verify-tag \
    --title "$tag" \
    --notes-file "$notes"
fi

gh api "repos/$repository/releases/tags/$tag" > "$release_json"
[ "$(jq '.assets | length' "$release_json")" = 0 ] \
  || { printf '%s\n' "GitHub Release contains unexpected assets" >&2; exit 1; }
