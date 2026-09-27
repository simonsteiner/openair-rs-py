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

### Unreleased

### v0.2.1 (2026-09-27)

- [fixed] Latitudes whose degrees are padded to three digits like longitude's (`DP 004:45:57.000 N 076:00:46.000 W`) parse again. 0.1.x read them; 0.2.0 took upstream's two-digit limit and rejected the whole file. Degrees above 90 are still rejected
- [changed] Not in the v0.2.0 notes, but changed there with the upstream altitude parser: a height with no unit (`AH 2300 MSL`, `AH 4572`) is read as feet, as the OpenAir format specifies. 0.1.x read `AH 2300 MSL` as metres and could not read `AH 4572` at all
- [added] Releases are cut from a GitHub Actions workflow, and each one gets a GitHub Release with its changelog section as notes and the wheels attached. See `RELEASING.md`

### v0.2.0 (2026-09-27)

Rust core synced with upstream glide-rs/openair-rs v0.6.0.

- [changed] **Breaking:** airspace dicts follow upstream v0.6.0. `class` is the raw `AC` token (`"R"`, `"Q"`, `"CTR"`, `"FFVL"`, ...) instead of names like `"Restricted"`; `type` is the raw `AY` token; `name` may be `None`; `frequency`, `callSign`, `transponderCode` and `activationTimes` appear when present. See "Migrating from 0.1.x" in the README
- [changed] **Breaking:** malformed decimal altitudes (e.g. `4500.0.5FT`) are returned as `Other` instead of failing the whole parse
- [added] `write_string()` / `write_file()` to write OpenAir, using upstream's writer
- [added] `normalize_legacy_classes=True` for `parse_string()` / `parse_file()` (upstream `Airspace::normalize_legacy_class()`)
- [added] `openair.types` TypedDicts describing the airspace dicts; typed stub for the compiled module
- [added] `openair` console script and `python -m openair` (the CLI was previously not shipped in the wheel), with `--normalize-legacy-classes` and `--version`
- [added] `openair.__version__`
- [added] Python 3.14 wheels
- [added] pytest suite, and CI running Rust tests, clippy, rustfmt, Python lint and the pytest suite against built wheels on Linux, macOS and Windows
- [changed] Parsing returns Python objects directly (no JSON string round-trip) and releases the GIL; the module is marked free-threading safe
- [changed] Upgraded PyO3 from 0.25 to 0.29
- [changed] License metadata uses a PEP 639 SPDX expression (`MIT OR Apache-2.0`); README is the PyPI description
- [changed] Publishing uses PyPI trusted publishing instead of an API token; updated GitHub Actions and replaced the retired `macos-13` runner
- [changed] Dev tooling: lefthook (`uv run lefthook install`) replaces pre-commit; ruff replaces black, isort, flake8 and pydocstyle; dev dependencies are a uv dependency group; `uv.lock` and `Cargo.lock` are committed; Dependabot for GitHub Actions and uv
- [deprecated] `OpenAirParser`; use the module-level functions
- [removed] `parse_openair_string()` / `parse_openair_file()` (JSON string functions)
- [removed] Support for Python 3.8 and 3.9 (end-of-life); `requires-python` is now `>=3.10`
- [fixed] Airspaces whose header records are interleaved with `SP`/`SB`/`AT` display records (e.g. `pao2025.txt`) no longer fail with "Missing lower bound" (regression in upstream's header-based separation)
- [fixed] Decimal altitudes (`4500.0FT AMSL`, `1371.6m`) re-applied on top of upstream's new altitude parser; decimal meters are converted before rounding

### v0.1.4 (2025-07-27)

- [added] The Python package now includes a `py.typed` marker file, enabling PEP 561 type hint support for downstream users and static type checkers (e.g., mypy, Pyright).

### v0.1.3 (2025-07-27)

- [added] Extended airspace class parsing with additional types and support for French classes
- [added] Support for 'Other' airspace class and French airspace classes in parser
- [added] Command-line interface (CLI) for parsing OpenAir files and outputting JSON
- [added] Validation for altitude input to prevent multiple decimal points
- [added] Unit test for parsing NOTAM reference class
- [changed] Enhanced altitude parsing: round near-integer float values, use constant for float tolerance, and log debug information
- [changed] Improved documentation links and wording in docs and comments
- [chore] Added Palz-Alsace-Open 2025 airspace file
- [chore] update cspell dictionary and configuration for improved spell checking

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

### v0.6.0 (2026-08-22)

- [added] Add support for OpenAir v2 `AY` airspace types (#52)
- [added] Add `Airspace::normalize_legacy_class()` (#52)
- [changed] Align `AC` airspace classes with the OpenAir v2 specification (#52)
- [changed] Expose `ActivationTimes` fields (#53)

### v0.5.0 (2026-04-19)

- [added] Implement writing functionality for OpenAir files (#43)
- [added] Add support for activation times extension records (#38)
- [added] Implement header-based airspace separation (#44)
- [changed] Make `Airspace::name` optional (#48)
- [changed] Update to Rust Edition 2024 (#39)
- [changed] Replace `regex` dependency with manual parsers for coordinate and altitude parsing

### v0.4.0 (2025-10-18)

- [added] Add support for AC UNC (Unclassified) (#19)
- [added] Add support for AX transponder code extension records (#13)
- [changed] Ignore unknown A* extension records (#20)
- [changed] Drop `lazy_static` dependency (#16)
- [fixed] Add missing pub keywords to public structs (#15)
- [fixed] Fix `Coord::parse_component()` to correctly parse DDM format (#17)

### v0.3.2 (2024-10-12)

- [fixed] Keep serializing `Class::Ctr` as `CTR`

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
