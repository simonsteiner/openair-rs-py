# Releasing openair-rs-py

Releases are cut from GitHub Actions. Between releases, add entries under
`### Unreleased` in the Python Bindings section of `CHANGELOG.md`: the release
turns that section into the release notes, and refuses to run while it is empty.

## Cut a release

Actions → **Release** → *Run workflow* on `main`, choose `patch`, `minor` or
`major`.

`release.yml` then:

1. runs CI (`ci.yml`) on the commit being released
2. bumps the version in `pyproject.toml` and `uv.lock` (`uv version --bump`)
3. renames `### Unreleased` to `### vX.Y.Z (date)` and leaves an empty
   `### Unreleased` above it (`scripts/changelog.py roll`)
4. commits `Release vX.Y.Z`, tags `vX.Y.Z` and pushes both to `main` in one
   atomic push. It fails if `main` moved since the run started
5. dispatches `publish.yml` at the tag

`publish.yml` then checks that the tag matches the `pyproject.toml` version and
has a dated changelog section, runs CI again, builds wheels for Linux (glibc
and musl), Windows and macOS plus the sdist, attests them, uploads them to PyPI
and creates the GitHub Release with the changelog section as its notes and the
wheels attached.

## By hand

Pushing a `vX.Y.Z` tag runs `publish.yml` too, with the same checks:

```bash
export VERSION=X.Y.Z
uv version "$VERSION" --no-sync
python3 scripts/changelog.py roll "$VERSION"
git commit -am "Release v${VERSION}"
git tag -a "v${VERSION}" -m "Version ${VERSION}"
git push --atomic origin main "v${VERSION}"
```

Running `publish.yml` from the Actions tab on a branch is a dry run: it builds
and attests the wheels but publishes nothing.

## Notes

- `pyproject.toml` holds the package version; `openair.__version__` reads it
  from the installed metadata. The `Cargo.toml` version tracks the upstream
  crate and is not bumped here
- Publishing uses PyPI [trusted publishing](https://docs.pypi.org/trusted-publishers/):
  on pypi.org the publisher is `simonsteiner/openair-rs-py`, workflow
  `publish.yml`, environment `pypi`. No API token secret is needed. For a manual
  approval before each upload, add a required reviewer to the `pypi`
  environment (Settings → Environments)
- `release.yml` pushes to `main` with `GITHUB_TOKEN`. If `main` gets branch
  protection, allow GitHub Actions to bypass it
- Only PyPI is published to, not crates.io: this fork is for the Python package
