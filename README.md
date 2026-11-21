# Retro7 Linux

**Computing, Reimagined.**

Retro7 Linux is an independent Linux distribution project built on Arch Linux. Its goal is a complete desktop experience with Retro7 Desktop (R7DE), an original Wayland desktop, an Aero-inspired default appearance, and a modular set of system tools.

## Current development milestone

The x86_64 live ISO now includes the R7DE GTK4 panel and applications, plus a graphical Wayland session using labwc as a temporary bootstrap compositor. The planned native Smithay compositor is still under development; this is an experimental integration image, not a finished desktop distribution or stable 1.0.

A regular non-root user logging in on tty1 is automatically started in R7DE. From another local TTY, run `r7-session` manually.

## Current scope

- Arch Linux base and systemd
- x86_64 live ISO with BIOS and UEFI boot support inherited from Archiso
- R7DE GTK4 panel, Explorer, Terminal, Editor, and Settings
- labwc bootstrap Wayland compositor and XWayland support
- GitHub Actions ISO build, BIOS/UEFI boot checks, and headless Wayland smoke test
- No default telemetry

The native Smithay compositor, Calamares installer, custom package repository, ARM64 image, Secure Boot signing, full Theme Creator/Store, and stable release remain future milestones.

## Build locally

See [Building Retro7](docs/BUILDING.md) for prerequisites and commands.

## Project direction

- MVP scope: [docs/MVP.md](docs/MVP.md)
- Architecture: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- R7DE status: [docs/R7DE.md](docs/R7DE.md)
- Roadmap: [docs/ROADMAP.md](docs/ROADMAP.md)
- Contributing: [CONTRIBUTING.md](CONTRIBUTING.md)

## License

The repository currently contains an LGPL-2.1 license file. The distribution will combine components with different upstream licenses; each component and redistributed package must retain its applicable license notices. A project-wide licensing policy remains to be documented.
