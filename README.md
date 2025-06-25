# openair-rs-py

**Note:** _This is a fork of <https://github.com/dbrgn/openair-rs> with Python bindings. See [README_ORIG.md](./README_ORIG.md) for original readme._

This directory contains Python bindings for the OpenAir airspace file parser written in Rust.

## Openair Format specification

<https://web.archive.org/web/20220703063934/http://www.winpilot.com/usersguide/userairspace.asp> (original page no longer available, archived version linked)

see also [FORMAT.txt](./FORMAT.txt)

For future improvements (version 2.1), see: <https://github.com/naviter/seeyou_file_formats/blob/main/OpenAir_File_Format_Support.md>

## Features

- Fast OpenAir file parsing using Rust
- Python-friendly API returning standard Python dictionaries
- Support for all OpenAir format features:
  - Airspace metadata (name, class, bounds)
  - Polygon points, circles, arcs
  - Extension records (AY/AF/AG)

## Installation

### Prerequisites

1. **Rust toolchain**: Install from [rustup.rs](https://rustup.rs/)
2. **Python 3.8+**
3. **Maturin**: Install with `pip install maturin`

### Building and Installation

#### Setup Virtual Environment (Recommended)

```bash
# Create and activate a virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install maturin in the virtual environment
pip install maturin
```

#### Build Commands

```bash
# Development build with debug symbols (installs directly into current environment)
maturin develop --features python

# Release build for distribution (creates wheel file)
maturin build --release --features python
```

Use `maturin develop` for development - it compiles the Rust code and installs the Python module directly into your current environment. Use `maturin build --release` when you need to create distribution wheels.

## Usage

```python
from openair import parse_string, parse_file

# Parse from string
openair_data = """
AC D
AN EXAMPLE CTR
AL GND
AH 5000 ft
DP 46:57:13 N 008:27:52 E
DP 46:57:46 N 008:30:41 E
"""

airspaces = parse_string(openair_data)

# Parse from file
airspaces = parse_file("path/to/airspace.txt")

# Each airspace is a dictionary with structure:
for airspace in airspaces:
    print(f"Name: {airspace['name']}")
    print(f"Class: {airspace['class']}")
    print(f"Lower bound: {airspace['lowerBound']}")
    print(f"Upper bound: {airspace['upperBound']}")
    print(f"Geometry: {airspace['geom']}")
```

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
