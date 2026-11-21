#!/usr/bin/env bash
# Smoke-test the R7DE panel inside a headless wlroots/labwc Wayland session.
set -euo pipefail

for program in labwc r7-shell; do
  command -v "$program" >/dev/null 2>&1 || {
    echo "Missing required test program: $program" >&2
    exit 127
  }
done

test_root="$(mktemp -d)"
runtime_dir="$test_root/runtime"
mkdir -m 700 "$runtime_dir"
export XDG_RUNTIME_DIR="$runtime_dir"
export WAYLAND_DISPLAY="wayland-r7de-test"
export WLR_BACKENDS="headless"
export WLR_RENDERER="pixman"
export WLR_LIBINPUT_NO_DEVICES=1
export XDG_SESSION_TYPE=wayland
export XDG_CURRENT_DESKTOP="Retro7:labwc"
export GDK_BACKEND=wayland

labwc >"$test_root/labwc.log" 2>&1 &
compositor_pid=$!
panel_pid=""

cleanup() {
  if [[ -n "$panel_pid" ]]; then
    kill "$panel_pid" 2>/dev/null || true
  fi
  kill "$compositor_pid" 2>/dev/null || true
  wait "$compositor_pid" 2>/dev/null || true
  rm -rf "$test_root"
}
trap cleanup EXIT

for _ in $(seq 1 100); do
  if [[ -S "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" ]]; then
    break
  fi
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
sleep 4

if ! kill -0 "$compositor_pid" 2>/dev/null; then
  cat "$test_root/labwc.log" >&2
  echo "The Wayland compositor stopped unexpectedly." >&2
  exit 1
fi
if ! kill -0 "$panel_pid" 2>/dev/null; then
  cat "$test_root/r7-shell.log" >&2
  echo "The R7DE panel failed to remain running in the Wayland session." >&2
  exit 1
fi

echo "R7DE_HEADLESS_OK: labwc compositor and r7-shell both stayed running."
