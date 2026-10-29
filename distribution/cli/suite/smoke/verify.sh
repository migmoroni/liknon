#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
  printf '%s\n' "Usage: verify.sh <suite-bin-directory>" >&2
  exit 2
fi

bin_dir=$1
for command_name in liknon liknon-validator liknon-manager liknon-tool; do
  command_path="$bin_dir/$command_name"
  [ -x "$command_path" ] || {
    printf '%s\n' "suite command is not executable: $command_path" >&2
    exit 1
  }
  "$command_path" --version > /dev/null
  "$command_path" -h > /dev/null
done

for rejected in likx likv likm likt; do
  [ ! -e "$bin_dir/$rejected" ] || {
    printf '%s\n' "compact executable alias must not exist: $rejected" >&2
    exit 1
  }
done

"$bin_dir/liknon" validator --version > /dev/null
"$bin_dir/liknon" v --version > /dev/null
"$bin_dir/liknon" manager --version > /dev/null
"$bin_dir/liknon" m --version > /dev/null
"$bin_dir/liknon" tool --version > /dev/null
"$bin_dir/liknon" t --version > /dev/null
"$bin_dir/liknon-validator" schema config > /dev/null
