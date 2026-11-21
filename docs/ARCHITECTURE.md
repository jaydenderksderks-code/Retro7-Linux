# Architecture

## Principles

- Retro7 is a distribution, not a theme applied to another distribution.
- Arch Linux, pacman, and systemd are the initial system foundation.
- Keep the system modular so the desktop and applications can be developed independently.
- No telemetry is enabled by default. Crash reports remain local unless a user explicitly chooses to share them.
- Untrusted theme content must not gain unrestricted shell access.
- Keep upstream components replaceable where practical.

## Current implementation

The repository currently starts with an Archiso-based x86_64 live-image pipeline. The first image reuses Archiso's upstream releng profile and adds Retro7 metadata and a small package set. It provides a bootable foundation, not the final desktop or installer.

Archiso supplies the bootloader and live-root configuration. Retro7 keeps only its own package additions and root-filesystem overlays in this repository, so upstream boot configuration is not copied and silently frozen here.

## Planned system shape

The long-term desktop is Retro7 Desktop (R7DE), a Wayland session. Smithay with Rust and GTK is the current proposal for the compositor and shell work, but module boundaries and IPC are not finalized.

Likely components include a compositor, shell/panel, start menu, settings, notifications, session manager, and theme engine. These are planning names, not implemented packages.

## Decisions still open

- R7DE module boundaries and compositor-to-shell IPC
- Initial desktop/session choice for the first graphical image
- Calamares configuration and exact supported install layouts
- ARM64 build and distribution strategy
- Secure Boot signing and key ownership
- Rollback behavior for Btrfs and ext4
- Official package repository and signing
- Theme format, validation, and sandboxing
- License policy for original code, assets, and the aggregate distribution
- Stable and rolling release policy

The staged MVP scope and acceptance criteria are documented in MVP.md. Resolve implementation details in focused design notes before they become hard-to-change interfaces.
