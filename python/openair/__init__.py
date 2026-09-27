"""Read and write airspace files in OpenAir format.

Parsing and writing are done by the Rust `openair` crate. Airspaces are plain
dicts; see `openair.types` for their shape.

Example:
    >>> import openair
    >>> airspaces = openair.parse_file("airspace.txt")  # doctest: +SKIP
    >>> openair.write_file(airspaces, "copy.txt")  # doctest: +SKIP
"""

import warnings
from importlib.metadata import PackageNotFoundError, version

from openair.openair import parse_file, parse_string, write_file, write_string
from openair.types import Airspace

try:
    __version__ = version("openair-rs-py")
except PackageNotFoundError:  # pragma: no cover - running from a source tree
    __version__ = "0+unknown"


class OpenAirParser:
    """Deprecated: use the module-level `parse_file` and `parse_string`."""

    @staticmethod
    def parse_string(data: str) -> list[Airspace]:
        """Parse OpenAir data from a string (deprecated)."""
        warnings.warn(
            "OpenAirParser is deprecated; use openair.parse_string",
            DeprecationWarning,
            stacklevel=2,
        )
        return parse_string(data)

    @staticmethod
    def parse_file(filepath: str) -> list[Airspace]:
        """Parse OpenAir data from a file (deprecated)."""
        warnings.warn(
            "OpenAirParser is deprecated; use openair.parse_file",
            DeprecationWarning,
            stacklevel=2,
        )
        return parse_file(filepath)


__all__ = [
    "Airspace",
    "OpenAirParser",
    "__version__",
    "parse_file",
    "parse_string",
    "write_file",
    "write_string",
]
