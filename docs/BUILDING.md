# Building Retro7

The first build target is an x86_64 Arch Linux live ISO. The build script stages the installed Archiso releng profile, overlays Retro7's package additions and live-root files, builds the R7DE applications, then invokes mkarchiso.

## Prerequisites

- An up-to-date Arch Linux x86_64 host
- archiso installed from the official Arch repositories
- Rust/Cargo
- sudo when not running the build as root
- Internet access to download packages

## Build

From the repository root:

    sudo pacman -Syu --needed archiso rust
    bash ./scripts/build-iso.sh

The script writes the ISO under .build/out/ and uses .build/work/ for temporary build data. You can override these paths with OUTPUT_DIR and WORK_DIR.

## Live desktop

The ISO includes the GTK R7DE applications and a Wayland session. labwc is currently used as a temporary bootstrap compositor; the planned native Smithay compositor is not yet implemented.

A regular non-root user logging in on tty1 is automatically placed into R7DE. From another local TTY, start it manually with:

    r7-session

To keep a text console on tty1, set RETRO7_NO_AUTO_START=1 in the login environment before the profile hook runs. The live session requires a working systemd-logind seat and a graphics device supported by wlroots.

## GitHub Actions

The Build and boot-test ISO workflow builds the profile in a privileged Arch Linux container, smoke-tests the GTK panel in a headless Wayland session, checks that the resulting ISO reaches a responsive live shell under QEMU in BIOS and UEFI modes, then uploads the image as a short-lived artifact.

## Local VM boot checks

The CI workflow uses QEMU and OVMF. It passes a serial console option into the image and verifies that the live shell accepts a command. You can repeat these checks locally with:

    python3 scripts/boot-test-iso.py --iso .build/out/retro7-YYYY.MM.DD-x86_64.iso --mode bios
    python3 scripts/boot-test-iso.py --iso .build/out/retro7-YYYY.MM.DD-x86_64.iso --mode uefi

Install qemu-system-x86 and OVMF before running the UEFI check.

## Notes

- This is an experimental desktop integration build, not a stable release.
- The base Archiso releng package set is inherited at build time; the repository contains only Retro7 additions.
- Arch Linux is a rolling distribution. Rebuilds may use newer packages than earlier artifacts.
