#!/usr/bin/env python3
"""Boot a Retro7 ISO in QEMU and verify that its live shell responds."""
from __future__ import annotations

import argparse
import glob
import os
import pty
import re
import select
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path


SHELL_PROMPT = re.compile(rb"root@retro7-live[^\r\n]*[#\$]\s*$", re.MULTILINE)
LOGIN_PROMPT = re.compile(rb"retro7-live login:", re.MULTILINE)
READY_MARKER = re.compile(rb"(?:^|\n)RETRO7_BOOT_OK(?:\r?\n|$)")


def stop_process(process: subprocess.Popen[bytes]) -> None:
    if process.poll() is not None:
        return
    process.terminate()
    try:
        process.wait(timeout=5)
    except subprocess.TimeoutExpired:
        process.kill()
        process.wait(timeout=5)


def uefi_firmware() -> tuple[Path, Path]:
    codes = [
        Path(path)
        for path in glob.glob("/usr/share/OVMF/OVMF_CODE*.fd")
        if "secboot" not in path.lower() and "ms" not in path.lower()
    ]
    for code in codes:
        vars_path = code.with_name(code.name.replace("CODE", "VARS"))
        if vars_path.is_file():
            return code, vars_path
    raise FileNotFoundError("Matching OVMF_CODE and OVMF_VARS firmware files were not found")


def qemu_command(iso: Path, mode: str, var_store: Path | None) -> list[str]:
    command = [
        "qemu-system-x86_64",
        "-machine",
        "q35,accel=kvm:tcg",
        "-m",
        "2048",
        "-display",
        "none",
        "-serial",
        "stdio",
        "-monitor",
        "none",
        "-no-reboot",
        "-boot",
        "order=d",
        "-cdrom",
        str(iso),
    ]
    if mode == "uefi":
        assert var_store is not None
        code, _ = uefi_firmware()
        command.extend(
            [
                "-drive",
                f"if=pflash,format=raw,readonly=on,file={code}",
                "-drive",
                f"if=pflash,format=raw,file={var_store}",
            ]
        )
    return command


def boot(iso: Path, mode: str, timeout_seconds: int) -> None:
    var_context = tempfile.TemporaryDirectory(prefix="retro7-ovmf-") if mode == "uefi" else None
    var_store: Path | None = None
    if var_context is not None:
        _, vars_source = uefi_firmware()
        var_store = Path(var_context.name) / "OVMF_VARS.fd"
        shutil.copyfile(vars_source, var_store)

    master, slave = pty.openpty()
    process = subprocess.Popen(
        qemu_command(iso, mode, var_store),
        stdin=slave,
        stdout=slave,
        stderr=slave,
        start_new_session=True,
    )
    os.close(slave)
    transcript = bytearray()
    deadline = time.monotonic() + timeout_seconds
    login_sent = False
    command_sent = False

    try:
        while time.monotonic() < deadline:
            if process.poll() is not None:
                break
            if login_sent and not command_sent and login_time is not None and time.monotonic() - login_time >= 5:
                os.write(master, b"printf 'RETRO7_BOOT_OK\\n'\n")
                command_sent = True
            ready, _, _ = select.select([master], [], [], 1.0)
            if not ready:
                continue
            try:
                chunk = os.read(master, 8192)
            except OSError:
                break
            if not chunk:
                break
            transcript.extend(chunk)
            if len(transcript) > 65536:
                del transcript[:-65536]

            if not login_sent and LOGIN_PROMPT.search(transcript):
                os.write(master, b"root\n")
                login_sent = True
            if not command_sent and SHELL_PROMPT.search(transcript):
                os.write(master, b"printf 'RETRO7_BOOT_OK\\n'\n")
                command_sent = True
            if command_sent and READY_MARKER in transcript:
                print(f"{mode.upper()} boot reached a responsive Retro7 live shell.")
                os.write(master, b"poweroff\n")
                return

        print(f"{mode.upper()} boot did not reach a responsive Retro7 live shell.", file=sys.stderr)
        print(transcript.decode("utf-8", errors="replace")[-8000:], file=sys.stderr)
        raise SystemExit(1)
    finally:
        stop_process(process)
        os.close(master)
        if var_context is not None:
            var_context.cleanup()


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--iso", required=True, type=Path)
    parser.add_argument("--mode", required=True, choices=("bios", "uefi"))
    parser.add_argument("--timeout", type=int, default=300)
    args = parser.parse_args()

    if not args.iso.is_file():
        parser.error(f"ISO not found: {args.iso}")
    boot(args.iso.resolve(), args.mode, args.timeout)


if __name__ == "__main__":
    main()
