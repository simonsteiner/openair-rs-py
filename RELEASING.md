# Releasing openair-rs-py

## Prerequisites

Install the dev dependencies (includes maturin): `uv sync`

## Update version numbers

Update version in `pyproject.toml`

## Update changelog

Update changelog in `CHANGELOG.md`
To see changes since the last release, run: `git log --oneline v0.1.4..HEAD`

## Build and test locally

Build the Python package locally to test:

```bash
uv run maturin build --release
uv run maturin develop  # Install into .venv for testing
```

## Commit & tag

```bash
export VERSION=X.Y.Z
git commit -m "Release v${VERSION}"
git tag v${VERSION} -m "Version ${VERSION}"
```

## Publish

### Automatic Publishing via GitHub Actions

The package will be automatically published to PyPI when you push a version tag (see `.github/workflows/publish.yml`). The GitHub Actions workflow will:

1. Build wheels for multiple platforms (Linux, Windows, macOS)
2. Build for multiple architectures (x86_64, aarch64, etc.)
3. Create a source distribution (sdist)
4. Upload everything to PyPI

Simply push the tag and the workflow handles the rest: `git push && git push --tags`

### Manual Publishing (if needed)

If you need to publish manually, you can use maturin directly:

Build wheels for multiple platforms:

```bash
uv run maturin build --release --interpreter python3.10 python3.11 python3.12 python3.13 python3.14
```

Upload to PyPI: `uv run maturin publish`

## Notes

- The Python package name will be `openair-rs-py` as defined in `pyproject.toml`
- GitHub Actions automatically builds wheels for multiple platforms and Python versions
- Make sure you have set the `PYPI_API_TOKEN` secret in your GitHub repository settings
- The workflow builds for Linux (x86_64, x86, aarch64, armv7, s390x, ppc64le), Windows (x64, x86), and macOS (x86_64, aarch64)
- This process only publishes to PyPI (not crates.io) since this is a fork focused on Python usage
- Publishing is triggered automatically when you push a version tag (e.g., `v1.0.0`)
