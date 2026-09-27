"""Typed shapes of the airspace dicts returned by `parse_file` and `parse_string`.

These mirror the serde serialization of the Rust `openair` crate: tagged unions
use a ``"type"`` key, and field names are camelCase.
"""

from typing import Literal, TypedDict


class Coord(TypedDict):
    """A WGS84 coordinate in decimal degrees."""

    lat: float
    lng: float


Direction = Literal["cw", "ccw"]


class Gnd(TypedDict):
    """Ground / surface level."""

    type: Literal["Gnd"]


class FeetAmsl(TypedDict):
    """Feet above mean sea level."""

    type: Literal["FeetAmsl"]
    val: int


class FeetAgl(TypedDict):
    """Feet above ground level."""

    type: Literal["FeetAgl"]
    val: int


class FlightLevel(TypedDict):
    """Flight level (hundreds of feet on standard pressure)."""

    type: Literal["FlightLevel"]
    val: int


class Unlimited(TypedDict):
    """No upper limit."""

    type: Literal["Unlimited"]


class OtherAltitude(TypedDict):
    """Altitude text the parser could not interpret, kept verbatim."""

    type: Literal["Other"]
    val: str


Altitude = Gnd | FeetAmsl | FeetAgl | FlightLevel | Unlimited | OtherAltitude


class Point(TypedDict):
    """A polygon vertex (``DP`` record)."""

    type: Literal["Point"]
    lat: float
    lng: float


class Arc(TypedDict):
    """An arc between two coordinates (``DB`` record)."""

    type: Literal["Arc"]
    centerpoint: Coord
    start: Coord
    end: Coord
    direction: Direction


class ArcSegment(TypedDict):
    """An arc given by radius and angles (``DA`` record)."""

    type: Literal["ArcSegment"]
    centerpoint: Coord
    radius: float
    angleStart: float
    angleEnd: float
    direction: Direction


PolygonSegment = Point | Arc | ArcSegment


class Polygon(TypedDict):
    """A polygon made of points and arcs; it may be open or closed."""

    type: Literal["Polygon"]
    segments: list[PolygonSegment]


class Circle(TypedDict):
    """A circle; ``radius`` is in nautical miles."""

    type: Literal["Circle"]
    centerpoint: Coord
    radius: float


Geometry = Polygon | Circle


class ActivationTimes(TypedDict):
    """Activation window (``AA`` record) as ISO 8601 strings; ``None`` is open-ended."""

    start: str | None
    end: str | None


# "class" is a Python keyword, so the required keys use the functional syntax.
_AirspaceRequired = TypedDict(
    "_AirspaceRequired",
    {
        "name": str | None,
        "class": str,
        "lowerBound": Altitude,
        "upperBound": Altitude,
        "geom": Geometry,
    },
)


class Airspace(_AirspaceRequired, total=False):
    """An airspace.

    ``class`` is the raw ``AC`` token (``"A"``-``"G"``, ``"UNC"``, or any other
    token such as ``"R"``, ``"CTR"`` or ``"FFVL"``). ``type`` is the raw ``AY``
    token. Optional keys are omitted when the record is absent.
    """

    type: str
    frequency: str
    callSign: str
    transponderCode: int
    activationTimes: ActivationTimes


__all__ = [
    "ActivationTimes",
    "Airspace",
    "Altitude",
    "Arc",
    "ArcSegment",
    "Circle",
    "Coord",
    "Direction",
    "FeetAgl",
    "FeetAmsl",
    "FlightLevel",
    "Geometry",
    "Gnd",
    "OtherAltitude",
    "Point",
    "Polygon",
    "PolygonSegment",
    "Unlimited",
]
