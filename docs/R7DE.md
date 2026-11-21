# Retro7 Desktop (R7DE)

R7DE is intended to be Retro7's own Wayland desktop, built around a Smithay compositor and GTK applications. It is not a re-skin of KDE or XFCE.

## Current implementation

- `r7de/shell` contains the first GTK4 panel prototype.
- The panel uses GTK layer-shell to attach to the top edge of a Wayland compositor.
- It includes a Retro7 launcher menu, terminal launch actions, a clock, and an Aero-inspired blue glass style.
- The ISO build compiles the panel from source and places `r7-shell` in `/usr/local/bin`.
- The R7DE workflow checks Rust compilation independently.

## Current limitations

This is the beginning of the shell, not a complete desktop environment. It requires a Wayland compositor and does not yet provide the Smithay compositor, window management, a login/session manager, panel configuration, notifications, settings, file manager, theme manager, or installer integration. Until those pieces land, the ISO must not be described as a stable desktop release.

The launcher currently offers applications that will be integrated in later milestones. A menu entry can report that its program is unavailable until that application is packaged.

## Architecture and build

The GTK shell and compositor are separate processes. They will communicate through Wayland protocols and narrowly scoped IPC where the standard protocols do not cover Retro7-specific settings. The compositor must own window focus, input routing, outputs, and protocol security; GTK layer-shell is for shell surfaces such as panels and notifications.

Build the shell inside an Arch Linux environment:

```sh
cargo check --manifest-path r7de/shell/Cargo.toml
cargo build --release --manifest-path r7de/shell/Cargo.toml
```

The ISO workflow supplies GTK4 and GTK4 layer-shell development packages, builds the binary, then includes its runtime packages in the image.
