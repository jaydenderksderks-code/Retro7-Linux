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

The Build and boot-test ISO workflow builds the profile in a privileged Arch Linux container, checks that the resulting ISO reaches a responsive live shell under QEMU in BIOS and UEFI modes, then uploads the image as a short-lived artifact. The first green run will be the first CI evidence for those VM boot paths. Package repositories and upstream Archiso can change over time.

## Local VM boot checks

The CI workflow uses QEMU and OVMF. It passes a serial console option into the image and verifies that the live shell accepts a command. You can repeat these checks locally with:

    python3 scripts/boot-test-iso.py --iso .build/out/retro7-YYYY.MM.DD-x86_64.iso --mode bios
    python3 scripts/boot-test-iso.py --iso .build/out/retro7-YYYY.MM.DD-x86_64.iso --mode uefi

Install qemu-system-x86 and OVMF before running the UEFI check.

## Notes

- This initial image does not include R7DE or Calamares.
- The base Archiso releng package set is inherited at build time; the repository contains only Retro7 additions.
- Arch Linux is a rolling distribution. Rebuilds may use newer packages than earlier artifacts.
