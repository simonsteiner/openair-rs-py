"""Roll and read the Python-bindings section of CHANGELOG.md for a release.

Usage:
    python scripts/changelog.py roll 0.2.1    # "### Unreleased" -> dated section
    python scripts/changelog.py notes 0.2.1   # body of "### v0.2.1 (<date>)"
    python scripts/changelog.py title 0.2.1   # "v0.2.1 (<date>)"

``roll`` keeps an empty ``### Unreleased`` heading above the new section and
refuses to release an empty one. ``notes`` and ``title`` exit non-zero when the
version has no dated section, so a workflow step using them fails before
anything is published. Only the ``## Python Bindings`` part is read: the
upstream Rust crate section below it has its own ``### vX.Y.Z`` headings.
"""

import datetime
import pathlib
import re
import sys

PATH = pathlib.Path(__file__).resolve().parent.parent / "CHANGELOG.md"
SECTION = "## Python Bindings (openair-rs-py)"
UNRELEASED = "### Unreleased"


def _bindings(lines: list[str]) -> tuple[int, int]:
    """Return the line range of the Python bindings section."""
    start = lines.index(SECTION)
    end = next(
        (i for i in range(start + 1, len(lines)) if lines[i].startswith("## ")),
        len(lines),
    )
    return start, end


def _body(lines: list[str], heading: int, end: int) -> list[str]:
    """Return the lines under ``heading`` up to the next heading or ``end``."""
    stop = next(
        (i for i in range(heading + 1, end) if lines[i].startswith("#")),
        end,
    )
    return lines[heading + 1 : stop]


def roll(text: str, version: str, date: datetime.date) -> str:
    """Rename the Unreleased section to ``### vX.Y.Z (date)``."""
    lines = text.splitlines()
    start, end = _bindings(lines)
    try:
        heading = lines.index(UNRELEASED, start, end)
    except ValueError:
        sys.exit(f"CHANGELOG.md has no '{UNRELEASED}' section.")
    if not "\n".join(_body(lines, heading, end)).strip():
        sys.exit(f"CHANGELOG.md '{UNRELEASED}' section is empty.")
    if _find(lines, version):
        sys.exit(f"CHANGELOG.md already has a section for v{version}.")
    lines[heading : heading + 1] = [
        UNRELEASED,
        "",
        f"### v{version} ({date.isoformat()})",
    ]
    return "\n".join(lines) + "\n"


def _find(lines: list[str], version: str) -> re.Match[str] | None:
    start, end = _bindings(lines)
    heading = re.compile(rf"^### v{re.escape(version)} \((?P<date>[^)]+)\)$")
    for i in range(start, end):
        match = heading.match(lines[i])
        if match:
            return match
    return None


def notes(text: str, version: str) -> str:
    """Return the body of the dated section for ``version``."""
    lines = text.splitlines()
    start, end = _bindings(lines)
    match = _find(lines, version)
    if not match:
        return ""
    heading = lines.index(match.string, start, end)
    return "\n".join(_body(lines, heading, end)).strip()


def title(text: str, version: str) -> str:
    """Return ``vX.Y.Z (date)`` for the dated section of ``version``."""
    match = _find(text.splitlines(), version)
    return match.string.removeprefix("### ") if match else ""


def main() -> None:
    """Run the subcommand given on the command line."""
    if len(sys.argv) != 3 or sys.argv[1] not in ("roll", "notes", "title"):
        sys.exit("usage: changelog.py {roll,notes,title} <version>")
    command, version = sys.argv[1], sys.argv[2].removeprefix("v")
    text = PATH.read_text(encoding="utf-8")
    if command == "roll":
        PATH.write_text(roll(text, version, datetime.date.today()), encoding="utf-8")
        return
    result = notes(text, version) if command == "notes" else title(text, version)
    if not result:
        sys.exit(f"CHANGELOG.md has no dated section for v{version}.")
    print(result)


if __name__ == "__main__":
    main()
