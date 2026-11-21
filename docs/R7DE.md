# Retro7 Desktop (R7DE)

R7DE is Retro7's own Wayland desktop environment, with a Rust/GTK shell and a compositor that is intended to be implemented with Smithay. It is not a re-skin of KDE or XFCE.

## Current implementation

- `r7de/shell` contains the GTK4 panel, R7 Explorer file browser, R7 Terminal emulator, R7 Editor, and R7 Settings theme selector.
- The panel uses GTK layer-shell to attach to the top edge of a Wayland compositor.
- The panel includes a Retro7 launcher menu, application launchers, a clock, and Aero-inspired styling.
- R7 Explorer browses folders and opens files through the default application.
- R7 Terminal uses the GTK4 VTE widget, launches the user's shell, and supports scrollback.
- R7 Editor supports opening and saving local text files.
- R7 Settings offers six panel styles and saves the choice under `~/.config/retro7/theme.conf`; the running panel watches that file and updates live.
- The ISO build compiles the GTK binaries from source and places all five applications in `/usr/local/bin`.
- `r7-session` starts a DBus session and the current bootstrap compositor.
- The labwc autostart configuration launches the R7DE panel and optional network/polkit helpers.
- A local login on tty1 starts R7DE automatically for a non-root live user. Set `RETRO7_NO_AUTO_START=1` to keep a text console for troubleshooting.
- `scripts/test-r7de-headless.sh` smoke-tests the panel inside a headless Wayland compositor in CI.

## Compositor status: bootstrap, not final

The first integrated graphical live session uses **labwc** as a temporary wlroots-based compositor. This gives the current GTK layer-shell panel a real Wayland session, basic window management, and a way to launch the existing R7 applications.

This is an integration milestone, not the final Retro7 architecture. The planned native Smithay compositor has **not** been implemented yet. Do not describe this labwc-based image as a completed custom-compositor desktop. The session launcher and desktop entry are designed so the compositor can later be replaced without changing the panel applications.

## Current limitations

- The native Smithay compositor and Retro7-specific window-management behavior are not implemented.
- The live autostart hook is for non-root interactive logins on tty1. A root login remains a text console; run `r7-session` as a regular user with a valid logind session.
- The panel and applications remain early prototypes.
- Notifications, complete theme manager/creator, Calamares integration, and installed-system session management remain future work.
- Hardware startup and graphics-driver coverage still need real-machine testing.

## Start and troubleshoot

From a local tty as a regular user:

```sh
r7-session
```

To keep tty1 as a text console on the next login:

```sh
RETRO7_NO_AUTO_START=1
```

The variable must be set in the login environment before the profile hook runs. Alternatively, switch to another TTY with Ctrl+Alt+F2 and inspect:

```sh
journalctl --user -b
cat ~/.local/state/retro7/r7-shell.log
```

## Architecture and build

The GTK shell and compositor are separate processes. The compositor owns window focus, input routing, outputs, and protocol security. GTK layer-shell is used for shell surfaces such as the panel and notifications.

Build the shell inside an Arch Linux environment:

```sh
cargo check --manifest-path r7de/shell/Cargo.toml
cargo build --release --manifest-path r7de/shell/Cargo.toml
```

The ISO workflow installs the GTK runtime, labwc bootstrap compositor, DBus, XWayland, and supporting desktop utilities.
