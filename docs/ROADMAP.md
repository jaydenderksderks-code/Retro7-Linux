# Roadmap

This roadmap follows the staged MVP in [MVP scope](MVP.md). It is a sequence of milestones, not a release promise.

## 1. Reproducible live-image foundation

- Build an x86_64 Archiso image from this repository.
- Produce an ISO artifact in GitHub Actions.
- Boot it in UEFI and BIOS virtual machines.
- Record known limitations and keep the image minimal.

## 2. First graphical session

- Prototype the R7DE compositor and shell in a separate, testable module.
- Start with basic session launch, input, windows, panel, launcher, and settings.
- Keep the live image usable while the desktop is developed.
- Use the MVP acceptance criteria to determine when the first graphical release is usable.

## 3. System integration

- Session management, networking, audio, notifications, and settings.
- Define package boundaries and a signed package repository.
- Establish update and rollback behavior.

## 4. Theme ecosystem

- Define the theme format and a validator.
- Add the Aero-inspired default style and Frutiger Aero as a separate theme.
- Build the Theme Manager before the visual Theme Creator.
- Require explicit trust and permissions for executable content.

## 5. Installer and applications

- Integrate and configure Calamares after the live image and package set are stable.
- Build Retro7 applications incrementally, preserving the ability to select alternatives.
- Add first-boot setup and installation choices.

## 6. Stabilization and release

- Test supported hardware and virtual machines.
- Validate boot modes, installation, updates, security, and recovery procedures.
- Publish a release only after documented acceptance criteria are met.

## Release status

There is no 1.0 date. Minimum hardware, ARM64 support, and stable-release policy are not yet proven or finalized.
