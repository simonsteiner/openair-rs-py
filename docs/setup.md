# Development setup

## Prerequisites

1. **Rust toolchain** from [rustup.rs](https://rustup.rs/). `rust-toolchain.toml`
   pins the version; rustup installs it on first use.
2. **Python 3.10+**
3. **uv** from [docs.astral.sh/uv](https://docs.astral.sh/uv/)
4. **Node.js** (for `npx cspell` in the git hooks)
5. **Nightly rustfmt** for the formatting hook and CI check:

   ```bash
   rustup toolchain install nightly-2025-10-18 --component rustfmt
   ```

   `.rustfmt.toml` uses unstable import-grouping options that stable rustfmt
   ignores.

## Build

```bash
# Create .venv, install the dev dependencies and build the package into it
uv sync
```

`uv sync` and `uv run` rebuild the extension when `Cargo.toml`, `Cargo.lock` or
any `src/**/*.rs` changes, so `uv run pytest` always tests the current Rust code.

To build by hand:

```bash
# Development build with debug symbols, installed into .venv
uv run maturin develop

# Release build for distribution (creates a wheel in target/wheels/)
uv run maturin build --release
```

`[tool.maturin]` in `pyproject.toml` already enables the `python` feature.

## Test

```bash
uv run pytest                      # Python tests (tests/python/)
cargo test --features serde        # Rust tests, including the serde snapshots
cargo run --example parse_file example_data/Switzerland.txt
uv run python python/example.py    # Python usage example
```

Rust tests use [insta](https://insta.rs/) snapshots in `tests/snapshots/`.
Test fixtures live in `example_data/`; see its `README.md` for their sources.

## Git hooks

Git hooks are managed by [lefthook](https://github.com/evilmartians/lefthook)
(config in `lefthook.yml`). Install them once per clone:

```bash
uv run lefthook install
```

On commit, the hooks run against the staged files:

- **Python**: `ruff check --fix`, `ruff format`, `mypy --explicit-package-bases`
- **Rust**: `cargo +nightly-2025-10-18 fmt`, then
  `cargo clippy --features python --all-targets -- -D warnings` on the whole crate
- **Spelling**: `cspell` (project words go in `cspell-dictionary.txt`)
- **File size**: `scripts/hooks/check-file-tokens.py` blocks any file over
  5,000 tokens (bytes / 4) and prints how to split it. Files nobody writes by
  hand, such as the airspace files in `example_data/`, are marked
  `linguist-generated` in `.gitattributes` and skipped.

On push, `cargo test --features serde` and `uv run pytest` run.

Run the hooks by hand with `uv run lefthook run pre-commit` (add `--all-files`
to check the whole repo). To skip them for one commit in an emergency, use
`LEFTHOOK=0 git commit …`.

## CI

`.github/workflows/ci.yml` runs on every pull request and push to `main`:
`cargo check --all-features`, `cargo test` with and without `serde`, clippy with
`--all-features`, the pinned-nightly rustfmt check, ruff and mypy, and pytest
against wheels built on Linux, macOS and Windows for Python 3.10 and 3.14.

## Troubleshooting

- **rustfmt reformats imports differently from CI**: you ran stable `cargo fmt`.
  Use `cargo +nightly-2025-10-18 fmt`.
- **Python tests don't see a Rust change**: run them through `uv run` (which
  rebuilds), or rebuild with `uv run maturin develop`.
