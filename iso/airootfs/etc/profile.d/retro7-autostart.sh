# Start R7DE automatically on the first local login shell on tty1.
# Set RETRO7_NO_AUTO_START=1 to keep a text console for troubleshooting.
case "$-" in
  *i*) ;;
  *) return ;;
esac

[ "${RETRO7_NO_AUTO_START:-0}" = "1" ] && return
[ -z "${SSH_TTY:-}" ] || return
[ -z "${WAYLAND_DISPLAY:-}" ] || return
[ "$(tty 2>/dev/null)" = "/dev/tty1" ] || return
[ "$(id -u)" -ne 0 ] || return
command -v r7-session >/dev/null 2>&1 || return

exec r7-session
