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

script_dir=$(CDPATH= cd "$(dirname "$0")" && pwd -P)
tmp_dir=$(mktemp -d "${TMPDIR:-/tmp}/liknon-tool-smoke.XXXXXX")
cleanup() { rm -rf -- "$tmp_dir"; }
trap cleanup EXIT HUP INT TERM

"$canonical" -h > "$tmp_dir/canonical-help.txt"
"$alias" -h > "$tmp_dir/alias-help.txt"
sed 's/likt/liknon-tool/g' "$tmp_dir/alias-help.txt" > "$tmp_dir/normalized-alias-help.txt"
cmp "$tmp_dir/canonical-help.txt" "$tmp_dir/normalized-alias-help.txt"

mkdir "$tmp_dir/workspace"
cp -R "$script_dir/fixture/.liknon" "$tmp_dir/workspace/"
cp -R "$tmp_dir/workspace/.liknon" "$tmp_dir/before"

"$canonical" catalog --workspace "$tmp_dir/workspace" --format=json > "$tmp_dir/catalog.json"
"$alias" search smoke --workspace "$tmp_dir/workspace" --format=json > "$tmp_dir/search.json"
"$canonical" show knowledge:fixture/smoke --workspace "$tmp_dir/workspace" --format=json > "$tmp_dir/show.json"
"$canonical" resolve knowledge:fixture/smoke --workspace "$tmp_dir/workspace" --format=json > "$tmp_dir/resolve.json"
"$canonical" context knowledge:fixture/smoke --workspace "$tmp_dir/workspace" --format=json > "$tmp_dir/context.json"

grep -F 'Liknon Tool smoke payload.' "$tmp_dir/show.json" > /dev/null
grep -F 'untrusted-resource-content' "$tmp_dir/context.json" > /dev/null
diff -r "$tmp_dir/before" "$tmp_dir/workspace/.liknon" > /dev/null
