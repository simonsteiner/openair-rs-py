# openair-rs-py

**Note:** _This is a fork of <https://github.com/glide-rs/openair-rs> (formerly dbrgn/openair-rs) with Python bindings. See [docs/reference/upstream-readme.md](https://github.com/simonsteiner/openair-rs-py/blob/main/docs/reference/upstream-readme.md) for the upstream readme._

Python bindings for reading and writing OpenAir airspace files, backed by the Rust `openair` crate.

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

## Usage

```python
import openair

openair_data = """
AC D
AN EXAMPLE CTR
AL GND
AH 5000 ft
DP 46:57:13 N 008:27:52 E
DP 46:57:46 N 008:30:41 E
DP 46:57:55 N 008:28:40 E
"""

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
      {"type": "Point", "lat": 46.96277, "lng": 8.51138},
      {"type": "Point", "lat": 46.96527, "lng": 8.47777}
    ]
  }
}
```

## OpenAir format specification

- OpenAir v2 (Naviter): <https://github.com/naviter/seeyou_file_formats/blob/main/OpenAir_File_Format_Support.md>
- Original WinPilot spec: <https://web.archive.org/web/20220703063934/http://www.winpilot.com/usersguide/userairspace.asp> (archived; a plain-text copy is in [docs/reference/openair-format.txt](https://github.com/simonsteiner/openair-rs-py/blob/main/docs/reference/openair-format.txt))

## Documentation

- [docs/setup.md](https://github.com/simonsteiner/openair-rs-py/blob/main/docs/setup.md): development setup, tests, git hooks
- [docs/releasing.md](https://github.com/simonsteiner/openair-rs-py/blob/main/docs/releasing.md): cutting a release and publishing to PyPI
- [CHANGELOG.md](https://github.com/simonsteiner/openair-rs-py/blob/main/CHANGELOG.md): release notes
- [GLOSSARY.md](https://github.com/simonsteiner/openair-rs-py/blob/main/GLOSSARY.md): OpenAir and domain terms used in the code and docs
- [AGENTS.md](https://github.com/simonsteiner/openair-rs-py/blob/main/AGENTS.md): rules for coding agents working on this repo
