# Retro7 Linux

**Computing, Reimagined.**

Retro7 Linux is an independent Linux distribution project built on Arch Linux. Its long-term goal is a complete desktop experience with Retro7 Desktop (R7DE), a Wayland desktop, an Aero-inspired default appearance, and a modular set of system tools.

The project is at the foundation stage. The first milestone is a reproducible x86_64 live ISO that boots in a virtual machine and on real hardware. This first image is a live system and development platform; it is not yet an installable desktop distribution.

## Current scope

- Arch Linux base and systemd
- x86_64 live ISO, with BIOS and UEFI boot modes provided by Archiso
- A small set of useful live-system tools
- GitHub Actions ISO build artifact
- No default telemetry

The desktop, installer, custom package repository, ARM64 image, Secure Boot signing, and release policy are later milestones. They are not represented as finished features.

## Build locally

See [Building Retro7](docs/BUILDING.md) for prerequisites and commands.

## Project direction

- [Architecture](docs/ARCHITECTURE.md)
- [Roadmap](docs/ROADMAP.md)
- [Contributing](CONTRIBUTING.md)

## License

The repository currently contains an LGPL-2.1 license file. The distribution will combine components with different upstream licenses; each component and redistributed package must retain its applicable license notices. A project-wide licensing policy remains to be documented.
