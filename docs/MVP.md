# MVP scope

This is the working MVP definition, based on the project brief and its priority to produce a bootable ISO first. It is a staged target: the first alpha image proves the build and boot path; the first user-facing MVP adds the Retro7 desktop and installer.

## Stage A — bootable alpha

- x86_64 Arch Linux live ISO built from the repository.
- BIOS and UEFI boot support inherited from Archiso.
- Reachable live shell with package management and basic network tools.
- Build artifact produced by GitHub Actions.
- Clearly identified as an experimental developer image.

This stage does not claim to be a usable desktop distribution. It exists to make the image pipeline and system base concrete.

## Stage B — user-facing MVP

- Boot to a working R7DE Wayland session by default.
- Basic window handling, panel, application launcher, settings, and terminal.
- Networking available from the live session.
- Aero-inspired default theme with system settings that remain usable when effects are disabled.
- Calamares installer supports a straightforward x86_64 installation using ext4 and UEFI.
- No default telemetry; no theme or package code is executed without appropriate installation and trust controls.
- Build and installation instructions are published with the image.

## Deferred beyond MVP

- ARM64 images, Secure Boot signing, and BIOS installation if they need separate development paths.
- Btrfs rollback and ext4 rollback.
- Custom app store, theme store, Theme Creator, community package publishing, and ratings.
- The full set of eleven Retro7 applications.
- Hardware minimum claims, stable release guarantees, and a 1.0 date.

## Acceptance criteria

Stage A is complete when the image builds from a clean checkout in CI, and the resulting ISO has been booted to the live shell in both UEFI and legacy BIOS virtual machines.

Stage B is complete when a new user can boot the live desktop, use the core session features, install to a supported UEFI machine or VM, reboot into the installed system, and update it using documented steps. Evidence must cover at least one supported VM configuration.

These criteria are targets, not claims that the current image meets them.