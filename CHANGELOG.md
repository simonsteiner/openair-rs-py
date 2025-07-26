# Changelog

This project follows semantic versioning.

Possible log types:

- `[added]` for new features.
- `[changed]` for changes in existing functionality.
- `[deprecated]` for once-stable features removed in upcoming releases.
- `[removed]` for deprecated features removed in this release.
- `[fixed]` for any bug fixes.
- `[security]` to invite users to upgrade in case of vulnerabilities.

## Python Bindings (openair-rs-py)

### v0.1.2 (2025-07-26)

- [added] .isort.cfg for import sorting configuration
- [added] .pre-commit-config.yaml for pre-commit hooks
- [added] cspell-dictionary.txt and cspell.json for spell checking
- [added] mypy.ini for type checking configuration
- [added] Optional development dependencies in pyproject.toml
- [changed] Updated README.md with pre-commit setup instructions
- [changed] Enhanced mypy configuration to specify files and paths
- [changed] Improved example.py with refined comments and file parsing demo
- [changed] Corrected formatting and added type hints in __init__.py and __init__.pyi
- [changed] Refactored RELEASING.md for improved readability and formatting consistency

### v0.1.1 (2025-06-25)

- [changed] Update pyo3 dependency to version 0.25
- [added] Support for Python 3.13 in project files
- [fixed] GitHub Actions CI workflow with matrix builds and manual dispatch trigger

### v0.1.0 (2025-06-25)

- [added] Python bindings for OpenAir airspace file parser
- [added] `parse_string()` function to parse OpenAir data from strings
- [added] `parse_file()` function to parse OpenAir files
- [added] Python-friendly API returning standard Python dictionaries
- [added] Support for all OpenAir format features: airspace metadata, polygon points, circles, arcs, and extension records
- [added] Maturin-based build system for Python module compilation
- [added] Python examples and documentation

## Original Rust Crate (openair-rs)

### v0.3.2 (2024-10-12)

- [fix] Keep serializing `Class::Ctr` as `CTR`

### v0.3.1 (2024-10-12)

- [fixed] Fix missing example in crates.io release

### v0.3.0 (2024-10-12)

- [added] Allow altitude as "ft MSL" (#9)
- [changed] Rename `Class::CTR` to `Class::Ctr`
- [changed] Update to Rust 2021 edition

### v0.2.0 (2019-06-06)

- [changed] Improved parsing support

### v0.1.4 (2019-04-28)

- [added] Support for serde serialization

### v0.1.3 (2019-04-26)

- [added] Class: Support more airspace classes
- [added] Altitude: Add SFC as alias for GND
- [added] Coord: Allow period as separator
- [changed] Ignore empty lines

### v0.1.2 (2019-04-26)

- First crates.io release
