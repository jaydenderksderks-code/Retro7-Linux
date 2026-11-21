#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
archiso_releng_dir="${ARCHISO_RELENG_DIR:-/usr/share/archiso/configs/releng}"
work_dir="${WORK_DIR:-"$repo_root/.build/work"}"
output_dir="${OUTPUT_DIR:-"$repo_root/.build/out"}"

if [[ "$(uname -m)" != "x86_64" ]]; then
  printf 'This initial profile targets x86_64; found %s.\n' "$(uname -m)" >&2
  exit 1
fi

if [[ ! -d "$archiso_releng_dir" ]]; then
  printf 'Archiso releng profile not found at %s. Install archiso or set ARCHISO_RELENG_DIR.\n' "$archiso_releng_dir" >&2
  exit 1
fi

for tool in mkarchiso cargo; do
  if ! command -v "$tool" >/dev/null 2>&1; then
    printf '%s was not found. Install the required Retro7 build packages.\n' "$tool" >&2
    exit 1
  fi
done

if [[ "$(id -u)" -ne 0 ]]; then
  if ! command -v sudo >/dev/null 2>&1; then
    printf 'Run as root or install sudo.\n' >&2
    exit 1
  fi
  exec sudo -- bash "$0" "$@"
fi

export ARCHISO_RELENG_DIR="$archiso_releng_dir"
mkdir -p -- "$work_dir" "$output_dir"
profile_dir="$(mktemp -d "${TMPDIR:-/tmp}/retro7-profile.XXXXXX")"
cleanup() {
  rm -rf -- "$profile_dir"
}
trap cleanup EXIT

cp -a -- "$archiso_releng_dir/." "$profile_dir/"
cat -- "$repo_root/iso/packages.x86_64" >> "$profile_dir/packages.x86_64"
install -m 0644 -- "$repo_root/iso/profiledef.sh" "$profile_dir/profiledef.sh"
cp -a -- "$repo_root/iso/airootfs/." "$profile_dir/airootfs/"

export CARGO_TARGET_DIR="$work_dir/cargo-target"
cargo build --release --bins --manifest-path "$repo_root/r7de/shell/Cargo.toml"
install -Dm 0755 -- "$CARGO_TARGET_DIR/release/r7-shell" "$profile_dir/airootfs/usr/local/bin/r7-shell"
install -Dm 0755 -- "$CARGO_TARGET_DIR/release/r7-explorer" "$profile_dir/airootfs/usr/local/bin/r7-explorer"

mkarchiso -v -w "$work_dir" -o "$output_dir" "$profile_dir"
