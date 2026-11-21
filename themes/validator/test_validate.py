from __future__ import annotations

import tempfile
import unittest
from pathlib import Path

from validate import validate_theme

VALID_MANIFEST = """[metadata]
id = "test-theme"
name = "Test Theme"
version = "1.0.0"
description = "Theme used by validator tests."

[appearance]
style = "aero"
accent = "#3478C6"
panel_background = "#316FA8"
panel_foreground = "#FFFFFF"
menu_background = "#DCEEFF"
transparency = 0.2
"""


class ThemeValidatorTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name) / "test.r7theme"
        self.root.mkdir()
        (self.root / "manifest.toml").write_text(VALID_MANIFEST, encoding="utf-8")

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def test_accepts_valid_declarative_theme(self) -> None:
        self.assertEqual(validate_theme(self.root), [])

    def test_rejects_unknown_script_field(self) -> None:
        manifest = VALID_MANIFEST + '\n[script]\nrun = "install.sh"\n'
        (self.root / "manifest.toml").write_text(manifest, encoding="utf-8")
        self.assertTrue(any("unknown key" in error for error in validate_theme(self.root)))

    def test_rejects_path_traversal_asset(self) -> None:
        manifest = VALID_MANIFEST + '\n[assets]\nwallpaper = "../outside.png"\n'
        (self.root / "manifest.toml").write_text(manifest, encoding="utf-8")
        self.assertTrue(any("stay inside" in error for error in validate_theme(self.root)))

    def test_rejects_non_image_asset(self) -> None:
        (self.root / "theme.py").write_text("print('unsafe')", encoding="utf-8")
        manifest = VALID_MANIFEST + '\n[assets]\npreview = "theme.py"\n'
        (self.root / "manifest.toml").write_text(manifest, encoding="utf-8")
        self.assertTrue(any("PNG, JPEG, or WebP" in error for error in validate_theme(self.root)))

    def test_rejects_non_string_style(self) -> None:
        manifest = VALID_MANIFEST.replace('style = "aero"', 'style = ["aero"]')
        (self.root / "manifest.toml").write_text(manifest, encoding="utf-8")
        self.assertTrue(any("supported built-in styles" in error for error in validate_theme(self.root)))

    def test_rejects_windows_asset_path(self) -> None:
        manifest = VALID_MANIFEST + '\n[assets]\nwallpaper = "C:\\\\outside.png"\n'
        (self.root / "manifest.toml").write_text(manifest, encoding="utf-8")
        self.assertTrue(any("relative POSIX" in error for error in validate_theme(self.root)))

    def test_rejects_invalid_opacity(self) -> None:
        manifest = VALID_MANIFEST.replace("transparency = 0.2", "transparency = 1.0")
        (self.root / "manifest.toml").write_text(manifest, encoding="utf-8")
        self.assertTrue(any("0.85" in error for error in validate_theme(self.root)))


if __name__ == "__main__":
    unittest.main()
