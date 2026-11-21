# Building Retro7

The first build target is an x86_64 Arch Linux live ISO. The build script stages the installed Archiso releng profile, overlays Retro7's package additions and live-root files, then invokes mkarchiso. This keeps boot support and required profile files aligned with the Archiso package version on the builder.

## Prerequisites

- An up-to-date Arch Linux x86_64 host
- archiso installed from the official Arch repositories
- sudo when not running the build as root
- Internet access to download packages

## Build

From the repository root:

    sudo pacman -Syu --needed archiso
    bash ./scripts/build-iso.sh

The script writes the ISO under .build/out/ and uses .build/work/ for temporary build data. You can override these paths with OUTPUT_DIR and WORK_DIR.

## GitHub Actions

The Build ISO workflow builds the same profile on pushes, pull requests, and manual dispatches. Successful runs upload the ISO as a short-lived workflow artifact. The first run is the first real build validation; package repositories and upstream Archiso can change over time.

## Boot checks

Use QEMU/KVM or VirtualBox to boot the resulting ISO. Check UEFI and legacy BIOS separately where available, confirm the live system reaches a login shell, and confirm networking is available. An ISO build alone does not demonstrate that it boots correctly.

## Notes

- This initial image does not include R7DE or Calamares.
- The base Archiso releng package set is inherited at build time; the repository contains only Retro7 additions.
- Arch Linux is a rolling distribution. Rebuilds may use newer packages than earlier artifacts.
