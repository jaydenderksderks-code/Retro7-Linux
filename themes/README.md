# Retro7 theme bundles

A theme is a directory ending in `.r7theme` with a `manifest.toml`. The first schema is declarative: it selects a supported visual style, color tokens, transparency, and optional local image assets. Theme content is never executed.

## Schema 1

Required fields:

- `[metadata]`: `id`, `name`, `version`, and `description`.
- `[appearance]`: supported `style`, four `#RRGGBB` colors, and `transparency` from 0 through 0.85.

Optional fields in `[assets]` are `wallpaper` and `preview`. Each must point to an existing PNG, JPEG, or WebP file within the theme directory. Symlinks, traversal paths, unknown keys, and assets larger than 20 MiB are rejected. Executable theme scripts are not part of schema 1.

Validate a bundle with:

```sh
python3 themes/validator/validate.py themes/retro7-aero.r7theme
python3 -m unittest discover -s themes/validator -p 'test_*.py'
```

`retro7-aero.r7theme` is the default preset and `frutiger-aero.r7theme` is the Frutiger Aero preset. The schema is intentionally small until the compositor and Theme Manager can consume it.
