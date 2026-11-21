#!/usr/bin/env bash
# Smoke-test the R7DE panel and Welcome Center inside headless labwc.
set -euo pipefail

for program in labwc r7-shell r7-welcome; do
  command -v "$program" >/dev/null 2>&1 || {
    echo "Missing required test program: $program" >&2
    exit 127
  }
done

test_root="$(mktemp -d)"
runtime_dir="$test_root/runtime"
home_dir="$test_root/home"
mkdir -m 700 "$runtime_dir" "$home_dir"
export HOME="$home_dir"
export XDG_CONFIG_HOME="$HOME/.config"
export XDG_RUNTIME_DIR="$runtime_dir"
export WAYLAND_DISPLAY="wayland-0"
export WLR_BACKENDS="headless"
export WLR_RENDERER="pixman"
export WLR_LIBINPUT_NO_DEVICES=1
export XDG_SESSION_TYPE=wayland
export XDG_CURRENT_DESKTOP="Retro7:labwc"
export GDK_BACKEND=wayland
export GSK_RENDERER=cairo

labwc >"$test_root/labwc.log" 2>&1 &
compositor_pid=$!
panel_pid=""
welcome_pid=""

cleanup() {
  [[ -z "$welcome_pid" ]] || kill "$welcome_pid" 2>/dev/null || true
  [[ -z "$panel_pid" ]] || kill "$panel_pid" 2>/dev/null || true
  kill "$compositor_pid" 2>/dev/null || true
  wait "$compositor_pid" 2>/dev/null || true
  rm -rf "$test_root"
}
trap cleanup EXIT

for _ in $(seq 1 100); do
  [[ -S "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" ]] && break
  if ! kill -0 "$compositor_pid" 2>/dev/null; then
    cat "$test_root/labwc.log" >&2
    echo "labwc exited before creating its Wayland socket." >&2
    exit 1
  fi
  sleep 0.1
done

if [[ ! -S "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" ]]; then
  cat "$test_root/labwc.log" >&2
  echo "labwc did not create a Wayland socket." >&2
  exit 1
fi

r7-shell >"$test_root/r7-shell.log" 2>&1 &
panel_pid=$!
r7-welcome >"$test_root/r7-welcome.log" 2>&1 &
welcome_pid=$!
sleep 4

for pair in "compositor:$compositor_pid" "panel:$panel_pid" "welcome:$welcome_pid"; do
  name="${pair%%:*}"
  pid="${pair#*:}"
  if ! kill -0 "$pid" 2>/dev/null; then
    case "$name" in
      compositor) cat "$test_root/labwc.log" >&2 ;;
      panel) cat "$test_root/r7-shell.log" >&2 ;;
      welcome) cat "$test_root/r7-welcome.log" >&2 ;;
    esac
    echo "The R7DE $name process exited unexpectedly." >&2
    exit 1
  fi
done

echo "R7DE_HEADLESS_OK: compositor, panel and Welcome Center stayed running."
