#!/bin/sh
set -eu

if [ "$#" -ne 2 ]; then
  printf '%s\n' "Usage: sign-direct.sh <direct-root> <private-key.pem>" >&2
  exit 2
fi

root=$1
private_key=$2

[ -f "$root/install.sh" ] || { printf '%s\n' "missing install.sh" >&2; exit 1; }
[ -f "$root/install.ps1" ] || { printf '%s\n' "missing install.ps1" >&2; exit 1; }
[ -f "$root/manifest.json" ] || { printf '%s\n' "missing manifest.json" >&2; exit 1; }
[ -f "$private_key" ] || { printf '%s\n' "missing detached-signature private key" >&2; exit 1; }

umask 077
openssl dgst -sha256 -sign "$private_key" -out "$root/install.sh.sig" "$root/install.sh"
openssl dgst -sha256 -sign "$private_key" -out "$root/install.ps1.sig" "$root/install.ps1"
openssl dgst -sha256 -sign "$private_key" -out "$root/manifest.sig" "$root/manifest.json"
