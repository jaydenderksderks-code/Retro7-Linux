# Retro7 Desktop (R7DE)

R7DE is intended to be Retro7's own Wayland desktop, built around a Smithay compositor and GTK applications. It is not a re-skin of KDE or XFCE.

## Current implementation

- `r7de/shell` contains the GTK4 panel, R7 Explorer file browser, and R7 Settings theme selector.
- The panel uses GTK layer-shell to attach to the top edge of a Wayland compositor.
- The panel includes a Retro7 launcher menu, terminal/files/settings actions, a clock, and Aero-inspired styling.
- R7 Explorer browses folders and opens files through the default application.\n- R7 Settings offers six panel styles and saves the choice under `~/.config/retro7/theme.conf`; the running panel watches that file and updates live.
- The ISO build compiles the GTK binaries from source and places `r7-shell`, `r7-explorer`, and `r7-settings` in `/usr/local/bin`.
- The R7DE workflow checks Rust compilation independently.

## Current limitations

This is the beginning of the shell, not a complete desktop environment. It requires a Wayland compositor and does not yet provide the Smithay compositor, window management, a login/session manager, notifications, full theme manager/creator, or installer integration. The current file browser and theme selector are early standalone applications. Until those pieces land, the ISO must not be described as a stable desktop release.

The shell and apps still require a Wayland session. They are built into the image but are not yet started by a live graphical session.

## Architecture and build

The GTK shell and compositor are separate processes. They will communicate through Wayland protocols and narrowly scoped IPC where the standard protocols do not cover Retro7-specific settings. The compositor must own window focus, input routing, outputs, and protocol security; GTK layer-shell is for shell surfaces such as panels and notifications.

Build the shell inside an Arch Linux environment:

```sh
cargo check --manifest-path r7de/shell/Cargo.toml
cargo build --release --manifest-path r7de/shell/Cargo.toml
```

The ISO workflow supplies GTK4 and GTK4 layer-shell development packages, builds the binary, then includes its runtime packages in the image.
