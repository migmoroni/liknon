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

canonical_version=$("$canonical" --version)
alias_version=$("$alias" --version)
[ "$canonical_version" = "$alias_version" ] \
  || { printf '%s\n' "installed commands report different versions" >&2; exit 1; }

tmp_dir=$(mktemp -d "${TMPDIR:-/tmp}/liknon-validation-smoke.XXXXXX")
cleanup() { rm -rf -- "$tmp_dir"; }
trap cleanup EXIT HUP INT TERM

"$canonical" -h > "$tmp_dir/canonical-help.txt"
"$alias" -h > "$tmp_dir/alias-help.txt"
cmp "$tmp_dir/canonical-help.txt" "$tmp_dir/alias-help.txt"
"$canonical" schema config > /dev/null

jq -n --arg program "$canonical" '{
  schemaVersion: 6,
  workspaceRoot: ".",
  defaultGroup: "release.smoke",
  outputLimitBytes: 4096,
  tools: [{
    id: "installed-validator",
    program: $program,
    requiresTools: [],
    versionArgs: ["--version"],
    versionParser: "firstSemver"
  }],
  checks: [{
    id: "installed-validator.version",
    label: "Installed validator version",
    description: "Executes the installed validation CLI.",
    toolId: "installed-validator",
    args: ["--version"],
    requiresTools: [],
    timeoutSeconds: 30
  }],
  suites: [{
    id: "release.smoke.suite",
    label: "Release smoke",
    description: "Validates the installed command.",
    checks: [{checkId: "installed-validator.version", dependsOn: []}]
  }],
  groups: [{
    id: "release.smoke",
    label: "Release smoke",
    description: "Runs the release smoke suite.",
    members: [{kind: "suite", id: "release.smoke.suite"}]
  }]
}' > "$tmp_dir/config.json"

"$canonical" validate --config "$tmp_dir/config.json" > /dev/null
