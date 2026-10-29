#!/bin/sh
set -eu

if [ "$#" -ne 2 ]; then
  printf '%s\n' "Usage: verify.sh <canonical-command> <alias-command>" >&2
  exit 2
fi

canonical=$1
alias=$2

[ -x "$canonical" ] || { printf '%s\n' "canonical command is not executable: $canonical" >&2; exit 1; }
[ -x "$alias" ] || { printf '%s\n' "alias command is not executable: $alias" >&2; exit 1; }
canonical=$(CDPATH= cd "$(dirname "$canonical")" && printf '%s/%s\n' "$(pwd -P)" "$(basename "$canonical")")
alias=$(CDPATH= cd "$(dirname "$alias")" && printf '%s/%s\n' "$(pwd -P)" "$(basename "$alias")")

[ "$("$canonical" --version)" = "$("$alias" --version)" ] \
  || { printf '%s\n' "installed commands report different versions" >&2; exit 1; }

tmp_dir=$(mktemp -d "${TMPDIR:-/tmp}/liknon-manager-smoke.XXXXXX")
cleanup() { rm -rf -- "$tmp_dir"; }
trap cleanup EXIT HUP INT TERM

"$canonical" -h > "$tmp_dir/canonical-help.txt"
"$alias" -h > "$tmp_dir/alias-help.txt"
sed 's/likm/liknon-manager/g' "$tmp_dir/alias-help.txt" > "$tmp_dir/normalized-alias-help.txt"
cmp "$tmp_dir/canonical-help.txt" "$tmp_dir/normalized-alias-help.txt"
"$canonical" schema resource > /dev/null
mkdir "$tmp_dir/workspace"
"$canonical" init --workspace "$tmp_dir/workspace" --format=json > /dev/null
"$alias" list --workspace "$tmp_dir/workspace" --format=json > /dev/null
"$canonical" verify --offline --workspace "$tmp_dir/workspace" --format=json > /dev/null
