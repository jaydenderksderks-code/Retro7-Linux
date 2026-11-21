#!/usr/bin/env python3
"""Validate a declarative Retro7 theme bundle without executing its contents."""

from __future__ import annotations

import argparse
import math
import re
import sys
import tomllib
from pathlib import Path

MAX_MANIFEST_BYTES = 1024 * 1024
MAX_ASSET_BYTES = 20 * 1024 * 1024
IDENTIFIER = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
VERSION = re.compile(r"^[0-9]+\.[0-9]+\.[0-9]+(?:-[0-9A-Za-z.-]+)?$")
COLOR = re.compile(r"^#[0-9A-Fa-f]{6}$")
IMAGE_SUFFIXES = {".png", ".jpg", ".jpeg", ".webp"}
ROOT_KEYS = {"metadata", "appearance", "assets"}
METADATA_KEYS = {"id", "name", "version", "description"}
APPEARANCE_KEYS = {
    "style",
    "accent",
    "panel_background",
    "panel_foreground",
    "menu_background",
    "transparency",
}
ASSET_KEYS = {"wallpaper", "preview"}


class ThemeValidationError(ValueError):
    """The theme bundle does not satisfy the Retro7 theme format."""


def _fail(message: str) -> None:
    raise ThemeValidationError(message)


def _table(data: dict, key: str) -> dict:
    value = data.get(key)
    if not isinstance(value, dict):
        _fail(f"'{key}' must be a TOML table")
    return value


def _check_keys(table: dict, allowed: set[str], section: str) -> None:
    unknown = sorted(set(table) - allowed)
    if unknown:
        _fail(f"unknown key(s) in {section}: {', '.join(unknown)}")


def _asset_path(root: Path, value: object, field: str) -> None:
    if not isinstance(value, str) or not value or "\\" in value or ":" in value:
        _fail(f"assets.{field} must be a relative POSIX file path")
    relative = Path(value)
    if relative.is_absolute() or any(part in {"", ".", ".."} for part in relative.parts):
        _fail(f"assets.{field} must stay inside the theme directory")
    candidate = root / relative
    cursor = root
    for part in relative.parts:
        cursor = cursor / part
        if cursor.is_symlink():
            _fail(f"assets.{field} must not use symlinks")
    try:
        resolved = candidate.resolve(strict=True)
        resolved.relative_to(root.resolve(strict=True))
    except (OSError, ValueError):
        _fail(f"assets.{field} does not resolve to a file inside the theme")
    if not resolved.is_file():
        _fail(f"assets.{field} is not a regular file")
    if resolved.suffix.lower() not in IMAGE_SUFFIXES:
        _fail(f"assets.{field} must be PNG, JPEG, or WebP")
    if resolved.stat().st_size > MAX_ASSET_BYTES:
        _fail(f"assets.{field} exceeds the 20 MiB size limit")


def validate_theme(path: Path) -> list[str]:
    """Return validation errors; an empty list means the bundle is valid."""
    try:
        _validate_theme(path)
    except ThemeValidationError as error:
        return [str(error)]
    return []


def _validate_theme(path: Path) -> None:
    if path.suffix != ".r7theme" or not path.is_dir() or path.is_symlink():
        _fail("theme must be a real directory whose name ends in .r7theme")

    manifest = path / "manifest.toml"
    if manifest.is_symlink() or not manifest.is_file():
        _fail("theme must contain a regular manifest.toml file")
    if manifest.stat().st_size > MAX_MANIFEST_BYTES:
        _fail("manifest.toml exceeds the 1 MiB size limit")
    try:
        data = tomllib.loads(manifest.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, tomllib.TOMLDecodeError) as error:
        _fail(f"manifest.toml cannot be read: {error}")

    _check_keys(data, ROOT_KEYS, "manifest")
    metadata = _table(data, "metadata")
    _check_keys(metadata, METADATA_KEYS, "metadata")
    missing = sorted(METADATA_KEYS - set(metadata))
    if missing:
        _fail(f"missing metadata field(s): {', '.join(missing)}")
    if not isinstance(metadata["id"], str) or not IDENTIFIER.fullmatch(metadata["id"]):
        _fail("metadata.id must use lowercase letters, digits, and single hyphens")
    if not isinstance(metadata["name"], str) or not 1 <= len(metadata["name"]) <= 80:
        _fail("metadata.name must contain 1 to 80 characters")
    if not isinstance(metadata["version"], str) or not VERSION.fullmatch(metadata["version"]):
        _fail("metadata.version must use major.minor.patch format")
    if not isinstance(metadata["description"], str) or not 1 <= len(metadata["description"]) <= 500:
        _fail("metadata.description must contain 1 to 500 characters")

    appearance = _table(data, "appearance")
    _check_keys(appearance, APPEARANCE_KEYS, "appearance")
    for key in {"accent", "panel_background", "panel_foreground", "menu_background"}:
        if not isinstance(appearance.get(key), str) or not COLOR.fullmatch(appearance[key]):
            _fail(f"appearance.{key} must be a #RRGGBB color")
    style = appearance.get("style")
    if not isinstance(style, str) or style not in {"aero", "frutiger-aero", "classic", "luna", "vista", "y2k"}:
        _fail("appearance.style is not one of the supported built-in styles")
    transparency = appearance.get("transparency")
    if isinstance(transparency, bool) or not isinstance(transparency, (int, float)):
        _fail("appearance.transparency must be a number from 0 to 0.85")
    if not math.isfinite(transparency) or not 0 <= transparency <= 0.85:
        _fail("appearance.transparency must be a number from 0 to 0.85")

    assets = data.get("assets", {})
    if not isinstance(assets, dict):
        _fail("'assets' must be a TOML table")
    _check_keys(assets, ASSET_KEYS, "assets")
    for key, value in assets.items():
        _asset_path(path, value, key)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("theme", type=Path, help="path to a .r7theme directory")
    args = parser.parse_args(argv)
    errors = validate_theme(args.theme)
    if errors:
        for error in errors:
            print(f"invalid: {error}", file=sys.stderr)
        return 1
    print(f"valid: {args.theme}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
