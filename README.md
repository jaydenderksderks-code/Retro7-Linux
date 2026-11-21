# Retro7 Linux

**Computing, Reimagined.**

Retro7 Linux is an independent Linux distribution project built on Arch Linux. Its long-term goal is a complete desktop experience with Retro7 Desktop (R7DE), a Wayland desktop, an Aero-inspired default appearance, and a modular set of system tools.

The project is building its first desktop milestone. An early GTK4 Wayland panel (R7DE shell prototype) is being compiled into the x86_64 live ISO. The image remains a development system: the Smithay compositor, complete desktop session, installer, and stable 1.0 are not finished.

## Current scope

- Arch Linux base and systemd
- x86_64 live ISO, with BIOS and UEFI boot modes provided by Archiso
- R7DE GTK4 panel prototype and Foot terminal
- GitHub Actions ISO build artifact
- No default telemetry

The complete R7DE desktop, Calamares installer, custom package repository, ARM64 image, Secure Boot signing, and release policy remain later milestones.

## Build locally

See Building Retro7 in docs/BUILDING.md for prerequisites and commands.

## Project direction

- MVP scope: docs/MVP.md
- Architecture: docs/ARCHITECTURE.md
- R7DE status: docs/R7DE.md
- Roadmap: docs/ROADMAP.md
- Contributing: CONTRIBUTING.md

## License

The repository currently contains an LGPL-2.1 license file. The distribution will combine components with different upstream licenses; each component and redistributed package must retain its applicable license notices. A project-wide licensing policy remains to be documented.
