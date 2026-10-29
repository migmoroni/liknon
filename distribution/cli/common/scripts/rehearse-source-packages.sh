#!/usr/bin/env bash
set -euo pipefail

usage() {
  printf '%s\n' \
    "usage: $0 <support-package>... -- <dependent-package>..." >&2
  exit 2
}

supports=()
dependents=()
after_separator=false
for argument in "$@"; do
  if [ "$argument" = "--" ]; then
    [ "$after_separator" = false ] || usage
    after_separator=true
  elif [ "$after_separator" = true ]; then
    dependents+=("$argument")
  else
    supports+=("$argument")
  fi
done

[ "$after_separator" = true ] || usage
[ "${#supports[@]}" -gt 0 ] || usage
[ "${#dependents[@]}" -gt 0 ] || usage

for command in cargo jq sha256sum tar find sort cut; do
  command -v "$command" >/dev/null || {
    printf 'required command is unavailable: %s\n' "$command" >&2
    exit 1
  }
done

workspace=$(pwd -P)
if [ -n "${LIKNON_SOURCE_REHEARSAL_ROOT:-}" ]; then
  temporary=$LIKNON_SOURCE_REHEARSAL_ROOT
  if [ -e "$temporary" ]; then
    [ -d "$temporary" ] || {
      printf 'rehearsal root is not a directory: %s\n' "$temporary" >&2
      exit 1
    }
    [ -z "$(find "$temporary" -mindepth 1 -print -quit)" ] || {
      printf 'rehearsal root is not empty: %s\n' "$temporary" >&2
      exit 1
    }
  else
    mkdir -p "$temporary"
  fi
else
  temporary=$(mktemp -d "${TMPDIR:-/tmp}/liknon-source-rehearsal.XXXXXX")
  trap 'rm -rf "$temporary"' EXIT
fi
vendor="$temporary/vendor"
package_arguments=(--locked)
if [ "${ALLOW_DIRTY:-0}" = 1 ]; then
  package_arguments+=(--allow-dirty)
fi

cargo vendor --quiet --locked --versioned-dirs "$vendor" \
  > "$temporary/vendor-config.txt"

for package in "${supports[@]}"; do
  version=$(cargo metadata --no-deps --format-version 1 | jq -r \
    --arg package "$package" \
    '.packages[] | select(.name == $package) | .version')
  [ -n "$version" ] && [ "$version" != null ] || {
    printf 'workspace package is unavailable: %s\n' "$package" >&2
    exit 1
  }

  cargo package -p "$package" "${package_arguments[@]}"
  archive="$workspace/target/package/$package-$version.crate"
  package_directory="$vendor/$package-$version"
  [ -f "$archive" ] || {
    printf 'package archive is unavailable: %s\n' "$archive" >&2
    exit 1
  }
  tar -xf "$archive" -C "$vendor"

  checksums="$temporary/$package-$version.checksums"
  : > "$checksums"
  while IFS= read -r -d '' file; do
    relative=${file#"$package_directory"/}
    hash=$(sha256sum "$file" | cut -d ' ' -f1)
    printf '%s\t%s\n' "$relative" "$hash" >> "$checksums"
  done < <(
    find "$package_directory" -type f ! -name .cargo-checksum.json -print0 \
      | sort -z
  )
  package_hash=$(sha256sum "$archive" | cut -d ' ' -f1)
  jq -Rn --arg package "$package_hash" \
    '[inputs | split("\t") | {(.[0]): .[1]}] | add | {files: ., package: $package}' \
    < "$checksums" > "$package_directory/.cargo-checksum.json"
done

for package in "${dependents[@]}"; do
  cargo \
    --config 'source.crates-io.replace-with="liknon-rehearsal"' \
    --config "source.liknon-rehearsal.directory='$vendor'" \
    package -p "$package" "${package_arguments[@]}"
done
