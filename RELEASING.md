# Releasing openair-rs-py

## Prerequisites

Make sure you have the required tools installed:

    $ pip install maturin twine

Set variables:

    $ export VERSION=X.Y.Z

## Update version numbers

Update version in pyproject.toml:

    $ vim pyproject.toml

## Update changelog

    $ vim CHANGELOG.md

## Build and test locally

Build the Python package locally to test:

    $ maturin build --release
    $ maturin develop  # Install in current Python environment for testing

## Commit & tag

    $ git commit -m "Release v${VERSION}"
    $ git tag v${VERSION} -m "Version ${VERSION}"

## Publish

### Automatic Publishing via GitHub Actions

The package will be automatically published to PyPI when you push a version tag. The GitHub Actions workflow will:

1. Build wheels for multiple platforms (Linux, Windows, macOS)
2. Build for multiple architectures (x86_64, aarch64, etc.)
3. Create a source distribution (sdist)
4. Upload everything to PyPI

Simply push the tag and the workflow handles the rest:

    $ git push && git push --tags

### Manual Publishing (if needed)

If you need to publish manually, you can use maturin directly:

Build wheels for multiple platforms:

    $ maturin build --release --interpreter python3.8 python3.9 python3.10 python3.11 python3.12

Upload to PyPI:

    $ maturin publish

Or manually upload with twine:

    $ twine upload target/wheels/*.whl

## Notes

- The Python package name will be `openair-rs-py` as defined in `pyproject.toml`
- GitHub Actions automatically builds wheels for multiple platforms and Python versions
- Make sure you have set the `PYPI_API_TOKEN` secret in your GitHub repository settings
- The workflow builds for Linux (x86_64, x86, aarch64, armv7, s390x, ppc64le), Windows (x64, x86), and macOS (x86_64, aarch64)
- This process only publishes to PyPI (not crates.io) since this is a fork focused on Python usage
- Publishing is triggered automatically when you push a version tag (e.g., `v1.0.0`)
