# openair-rs-py

**Note:** _This is a fork of <https://github.com/glide-rs/openair-rs> (formerly dbrgn/openair-rs) with Python bindings. See [README_ORIG.md](./README_ORIG.md) for the upstream readme._

Python bindings for reading and writing OpenAir airspace files, backed by the Rust `openair` crate.

## Openair Format specification

<https://web.archive.org/web/20220703063934/http://www.winpilot.com/usersguide/userairspace.asp> (original page no longer available, archived version linked)

see also [FORMAT.txt](./FORMAT.txt)

OpenAir v2 (Naviter): <https://github.com/naviter/seeyou_file_formats/blob/main/OpenAir_File_Format_Support.md>

## Features

- Fast OpenAir parsing and writing in Rust, returning plain Python dicts
- Typed: `openair.types` describes the dicts as `TypedDict`s (PEP 561 `py.typed`)
- OpenAir v2 `AC` classes and `AY` types; any other token (e.g. French `FFVL`,
  `ZSM`, legacy `R`/`Q`/`CTR`) is kept verbatim
- Extension records: `AY`, `AF`, `AG`, `AX`, `AA`
- Polygons, circles and arcs; decimal altitudes such as `4500.0FT AMSL`
- `openair` command-line tool that converts OpenAir to JSON

## Installation

```bash
pip install openair-rs-py
```

## Development

### Prerequisites

1. **Rust toolchain**: Install from [rustup.rs](https://rustup.rs/)
2. **Python 3.10+**
3. **uv**: Install from [docs.astral.sh/uv](https://docs.astral.sh/uv/)

### Building and Installation

#### Setup Development Environment

```bash
# Create .venv, install the dev dependencies and build the package into it
uv sync
```

#### Build Commands

```bash
# Development build with debug symbols (installs directly into .venv)
uv run maturin develop --features python

# Release build for distribution (creates wheel file)
uv run maturin build --release --features python
```

Use `maturin develop` for development - it compiles the Rust code and installs the Python module directly into your current environment. Use `maturin build --release` when you need to create distribution wheels.

## Usage

```python
import openair

# Parse from a string or a file (str or pathlib.Path)
airspaces = openair.parse_string(openair_data)
airspaces = openair.parse_file("path/to/airspace.txt")

for airspace in airspaces:
    print(airspace["name"], airspace["class"], airspace.get("type"))
    print(airspace["lowerBound"], airspace["upperBound"], airspace["geom"]["type"])

# Move legacy AC tokens (R, Q, P, CTR, GP, W, RMZ, TMZ) into AY and set class UNC
airspaces = openair.parse_file("path/to/airspace.txt", normalize_legacy_classes=True)

# Write OpenAir again (to a string or a file)
text = openair.write_string(airspaces)
openair.write_file(airspaces, "out.txt")
```

Parse errors raise `ValueError`, unreadable files raise `OSError`.

For type checking, annotate with `openair.Airspace` (or the other types in `openair.types`).

### Command line

```bash
openair example_data/Switzerland.txt --pretty -o switzerland.json
python -m openair example_data/Switzerland.txt
```

### Migrating from 0.1.x

- `class` is now the raw `AC` token: `"R"`, `"Q"`, `"P"`, `"CTR"`, `"FFVL"`, …
  (0.1.x returned names like `"Restricted"`, `"Danger"`, `"Ffvl"`). Pass
  `normalize_legacy_classes=True` to get OpenAir v2 style `class: "UNC"` plus `type`.
- `name` may be `None`; `frequency`, `callSign`, `transponderCode` and
  `activationTimes` appear when present.
- Malformed altitudes such as `4500.0.5FT` no longer raise; they come back as
  `{"type": "Other", "val": "4500.0.5FT"}`.
- `OpenAirParser` is deprecated; use the module-level functions.

## Example Output

The parser returns airspaces as Python dictionaries with this structure:

```json
{
  "name": "EXAMPLE CTR",
  "class": "D",
  "lowerBound": {"type": "Gnd"},
  "upperBound": {"type": "FeetAmsl", "val": 5000},
  "geom": {
    "type": "Polygon",
    "segments": [
      {"type": "Point", "lat": 46.95361, "lng": 8.46444},
      {"type": "Point", "lat": 46.96277, "lng": 8.51138}
    ]
  }
}
```

---

## Code Quality & Formatting

Git hooks are managed by [lefthook](https://github.com/evilmartians/lefthook) (config in `lefthook.yml`). Install them once per clone:

```bash
uv run lefthook install
```

On commit, the hooks run against the staged files:

- **Python**: `ruff check --fix`, `ruff format`, `mypy`
- **Rust**: `cargo fmt`, `cargo clippy -- -D warnings`
- **Spelling**: `cspell` (project words go in `cspell-dictionary.txt`)

On push, `cargo test` runs.

Run the hooks manually with `uv run lefthook run pre-commit` (add `--all-files` to check the whole repo).

If you need to skip hooks for a commit, use `git commit --no-verify`.
