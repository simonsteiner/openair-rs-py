# openair-rs-py

Python bindings for reading and writing OpenAir airspace files, published to PyPI as `openair-rs-py`. The Rust crate in `src/` (a fork of upstream glide-rs/openair-rs) does all parsing and writing; `src/python.rs` exposes it through PyO3 as `openair.openair`, and the pure-Python package in `python/openair/` re-exports it, adds the typed dict shapes and the `openair` CLI. Airspaces cross the boundary as plain dicts in the same shape as the crate's serde JSON.
Domain terms are in `GLOSSARY.md`; there are no ADRs yet.

## Commands

```bash
uv sync  # create .venv, install dev deps, build the extension
uv run pytest  # all Python tests; rebuilds the extension if Rust changed
uv run pytest tests/python/test_openair.py  # one Python test file (add ::TestParse for one class)
cargo test --features serde  # all Rust tests, including serde snapshots
cargo test --features serde --test parser  # one Rust test file (tests/parser.rs)
cargo clippy --features python --all-targets -- -D warnings  # Rust lint
cargo +nightly-2025-10-18 fmt  # Rust format (pinned nightly, not stable)
uv run ruff check --fix && uv run ruff format  # Python lint and format
uv run mypy --explicit-package-bases python tests/python  # Python type check
uv run lefthook run pre-commit --all-files  # every pre-commit hook on the whole repo
cargo run --example parse_file example_data/Switzerland.txt  # parse a fixture from Rust
```

## Conventions

### Rust and Python boundary

- **Parsing and writing live only in the Rust crate.** The Python package never re-implements them; it calls `parse_string`, `parse_file`, `write_string` and `write_file` from `src/python.rs`.
- **Airspaces are plain dicts, not classes.** Keys are camelCase and tagged unions carry a `"type"` key, mirroring serde.
- **Change the dict shape in three places.** A serde change in `src/` needs matching updates to `python/openair/types.py` (the `TypedDict`s) and `python/openair/openair.pyi` (the extension stub).
- **`class` is the raw `AC` token.** Unknown tokens are kept verbatim; only `normalize_legacy_classes=True` (Rust `Airspace::normalize_legacy_class`) turns a legacy class into `UNC` plus a type.

### Changelog and versions

- **Every user-visible change gets a `CHANGELOG.md` entry under `### Unreleased`** in the `## Python Bindings (openair-rs-py)` section, as a list item with a log type (`[added]`, `[changed]`, `[fixed]`, …). The upstream Rust section below it is not edited.
- **`pyproject.toml` holds the package version.** The `Cargo.toml` version tracks upstream and is never bumped here.

### Tests and fixtures

- **Real-world fixtures live in `example_data/`.** Record the source of every new file in `example_data/README.md`.
- **Rust tests use insta snapshots** in `tests/snapshots/`; Python tests are in `tests/python/`.
- **Project words for the spell checker go in `cspell-dictionary.txt`.**

## Hard constraints

- **Never bump the version or date a changelog section in a feature PR.** The Release workflow (`docs/releasing.md`) does both.
- **Never publish to crates.io.** This fork publishes only to PyPI.
- **Never "fix" the encoding of `example_data/Germany.txt`.** Its non-UTF-8 bytes are a deliberate test case.

## Gotchas

- **Stable `cargo fmt` is wrong here.** `.rustfmt.toml` uses unstable options that only `cargo +nightly-2025-10-18 fmt` applies, and CI checks with that nightly.
- **Rust tests need `--features serde`** for the serde snapshot tests; the bindings need `--features python`, which maturin enables by itself.
- **A height without a unit is feet AMSL.** `AH 2300 MSL` and `AH 4572` are both feet, as the OpenAir spec says.
- **`gh` defaults to the upstream repo in this fork.** Pass `--repo simonsteiner/openair-rs-py` to `gh pr` and `gh issue` commands.

## Docs

- `README.md` — what the package is, install, usage, CLI, migrating from 0.1.x; also the PyPI page.
- `docs/setup.md` — read before building, running hooks, or when a tool disagrees with CI.
- `docs/releasing.md` — read before touching the release or publish workflows or cutting a release.
- `docs/reference/` — the OpenAir format spec (`openair-format.txt`) and the upstream crate README; read when parser behaviour is in question.
- `CHANGELOG.md` — release notes and the `### Unreleased` section to add to.
- `GLOSSARY.md` — domain terms; use them in code, docs and commit messages.
