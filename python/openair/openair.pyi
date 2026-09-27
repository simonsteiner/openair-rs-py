"""Type stub for the compiled Rust extension module."""

import os
from collections.abc import Iterable

from openair.types import Airspace

def parse_string(
    data: str, *, normalize_legacy_classes: bool = False
) -> list[Airspace]: ...
def parse_file(
    path: str | os.PathLike[str], *, normalize_legacy_classes: bool = False
) -> list[Airspace]: ...
def write_string(airspaces: Iterable[Airspace]) -> str: ...
def write_file(airspaces: Iterable[Airspace], path: str | os.PathLike[str]) -> None: ...
