"""Tests for the openair Python bindings."""

import json
import subprocess
import sys
from pathlib import Path
from typing import Any, get_args, get_type_hints

import pytest

import openair
from openair import cli, types

EXAMPLE_DATA = Path(__file__).resolve().parents[2] / "example_data"
FIXTURES = [
    "Switzerland.txt",
    "Germany.txt",
    "Germany_Border.txt",
    "France.txt",
    "pao2025.txt",
]

CTR = """\
AC D
AN EXAMPLE CTR
AL GND
AH 5000 ft
DP 46:57:13 N 008:27:52 E
DP 46:57:46 N 008:30:41 E
DP 46:57:55 N 008:28:40 E
DP 46:57:13 N 008:27:52 E
"""


def _type_tags(union: Any) -> set[str]:
    """Collect the Literal "type" tags of a union of TypedDicts."""
    return {get_args(get_type_hints(member)["type"])[0] for member in get_args(union)}


class TestParse:
    """Tests for parse_string and parse_file."""

    def test_parse_string(self) -> None:
        [airspace] = openair.parse_string(CTR)
        assert airspace == {
            "name": "EXAMPLE CTR",
            "class": "D",
            "lowerBound": {"type": "Gnd"},
            "upperBound": {"type": "FeetAmsl", "val": 5000},
            "geom": {
                "type": "Polygon",
                "segments": [
                    {
                        "type": "Point",
                        "lat": pytest.approx(46.953611),
                        "lng": pytest.approx(8.464444),
                    },
                    {
                        "type": "Point",
                        "lat": pytest.approx(46.962778),
                        "lng": pytest.approx(8.511389),
                    },
                    {
                        "type": "Point",
                        "lat": pytest.approx(46.965278),
                        "lng": pytest.approx(8.477778),
                    },
                    {
                        "type": "Point",
                        "lat": pytest.approx(46.953611),
                        "lng": pytest.approx(8.464444),
                    },
                ],
            },
        }

    def test_empty_input(self) -> None:
        assert openair.parse_string("") == []

    def test_class_and_type_tokens_are_raw(self) -> None:
        [airspace] = openair.parse_string(
            "AC FFVL\nAY PROTECT\nAN X\nAL GND\nAH 500ft AGL\nV X=48:38:00 N 007:42:34 E\nDC 1.0\n"
        )
        assert airspace["class"] == "FFVL"
        assert airspace["type"] == "PROTECT"
        assert airspace["geom"]["type"] == "Circle"

    def test_latitude_padded_to_three_digits(self) -> None:
        """Some producers pad latitude degrees like longitude's (`004:45:57 N`)."""
        data = CTR.replace("DP 46:57:13 N", "DP 046:57:13 N")
        [airspace] = openair.parse_string(data)
        assert airspace == openair.parse_string(CTR)[0]

    def test_missing_name_is_none(self) -> None:
        [airspace] = openair.parse_string(CTR.replace("AN EXAMPLE CTR\n", ""))
        assert airspace["name"] is None

    def test_optional_records(self) -> None:
        data = CTR.replace(
            "AL GND\n",
            "AL GND\nAF 123.450\nAG Zurich Info\nAX 7000\n"
            "AA 2023-12-16T12:00Z/2023-12-16T13:00Z\n",
        )
        [airspace] = openair.parse_string(data)
        assert airspace["frequency"] == "123.450"
        assert airspace["callSign"] == "Zurich Info"
        assert airspace["transponderCode"] == 7000
        activation = airspace["activationTimes"]
        assert activation["start"] is not None
        assert activation["start"].startswith("2023-12-16T12:00")
        assert activation["end"] is not None

    @pytest.mark.parametrize(
        ("text", "expected"),
        [
            ("4500.0FT AMSL", {"type": "FeetAmsl", "val": 4500}),
            ("500.0FT GND", {"type": "FeetAgl", "val": 500}),
            ("1371.6m", {"type": "FeetAmsl", "val": 4500}),
            ("FL95", {"type": "FlightLevel", "val": 95}),
            ("UNLIM", {"type": "Unlimited"}),
            ("4500.0.5FT", {"type": "Other", "val": "4500.0.5FT"}),
        ],
    )
    def test_altitudes(self, text: str, expected: dict[str, Any]) -> None:
        [airspace] = openair.parse_string(CTR.replace("AH 5000 ft", f"AH {text}"))
        assert airspace["upperBound"] == expected

    def test_normalize_legacy_classes(self) -> None:
        data = CTR.replace("AC D", "AC R")
        [raw] = openair.parse_string(data)
        [normalized] = openair.parse_string(data, normalize_legacy_classes=True)
        assert raw["class"] == "R"
        assert "type" not in raw
        assert normalized["class"] == "UNC"
        assert normalized["type"] == "R"

    def test_normalize_legacy_classes_conflict(self) -> None:
        data = CTR.replace("AC D", "AC R\nAY Q")
        with pytest.raises(ValueError, match="Legacy class R"):
            openair.parse_string(data, normalize_legacy_classes=True)

    def test_parse_error(self) -> None:
        with pytest.raises(ValueError, match="Missing lower bound"):
            openair.parse_string(CTR.replace("AL GND\n", ""))

    @pytest.mark.parametrize("fixture", FIXTURES)
    def test_fixtures(self, fixture: str) -> None:
        airspaces = openair.parse_file(EXAMPLE_DATA / fixture)
        assert airspaces

    def test_parse_file_accepts_str(self) -> None:
        assert openair.parse_file(str(EXAMPLE_DATA / "Switzerland.txt"))

    def test_parse_file_missing(self, tmp_path: Path) -> None:
        with pytest.raises(OSError, match="Failed to open file"):
            openair.parse_file(tmp_path / "nope.txt")


class TestWrite:
    """Tests for write_string and write_file."""

    def test_write_string_roundtrip(self) -> None:
        airspaces = openair.parse_string(CTR)
        text = openair.write_string(airspaces)
        assert text.startswith("AC D\r\nAN EXAMPLE CTR\r\n")
        assert openair.parse_string(text) == airspaces

    @pytest.mark.parametrize("fixture", FIXTURES)
    def test_write_is_stable(self, fixture: str) -> None:
        written = openair.write_string(openair.parse_file(EXAMPLE_DATA / fixture))
        assert openair.write_string(openair.parse_string(written)) == written

    def test_write_file(self, tmp_path: Path) -> None:
        airspaces = openair.parse_string(CTR)
        out = tmp_path / "out.txt"
        openair.write_file(airspaces, out)
        assert openair.parse_file(out) == airspaces

    def test_write_accepts_handwritten_dicts(self) -> None:
        airspace: openair.Airspace = {
            "name": "HANDMADE",
            "class": "C",
            "lowerBound": {"type": "FlightLevel", "val": 95},
            "upperBound": {"type": "Unlimited"},
            "geom": {
                "type": "Circle",
                "centerpoint": {"lat": 47.0, "lng": 8.0},
                "radius": 5.0,
            },
        }
        text = openair.write_string([airspace])
        assert "AC C\r\n" in text
        assert "DC 5" in text

    @pytest.mark.parametrize(
        "bad",
        [
            [{"name": "X"}],
            [{**openair.parse_string(CTR)[0], "class": ""}],
            [{**openair.parse_string(CTR)[0], "lowerBound": {"type": "Bogus"}}],
        ],
    )
    def test_write_rejects_invalid(self, bad: Any) -> None:
        with pytest.raises(ValueError, match="Invalid airspace"):
            openair.write_string(bad)


class TestTypes:
    """The TypedDicts in openair.types must match what the parser returns."""

    @pytest.mark.parametrize("fixture", FIXTURES)
    def test_keys_and_tags(self, fixture: str) -> None:
        airspace_keys = set(get_type_hints(types.Airspace))
        altitude_tags = _type_tags(types.Altitude)
        geometry_tags = _type_tags(types.Geometry)
        segment_tags = _type_tags(types.PolygonSegment)
        for airspace in openair.parse_file(EXAMPLE_DATA / fixture):
            assert set(airspace) <= airspace_keys
            assert airspace["lowerBound"]["type"] in altitude_tags
            assert airspace["upperBound"]["type"] in altitude_tags
            geom = airspace["geom"]
            assert geom["type"] in geometry_tags
            if geom["type"] == "Polygon":
                for segment in geom["segments"]:
                    assert segment["type"] in segment_tags


class TestCli:
    """Tests for the openair command-line interface."""

    def test_stdout(self, capsys: pytest.CaptureFixture[str]) -> None:
        assert cli.main([str(EXAMPLE_DATA / "Switzerland.txt")]) == 0
        assert len(json.loads(capsys.readouterr().out)) > 0

    def test_output_file_pretty(self, tmp_path: Path) -> None:
        out = tmp_path / "out.json"
        path = str(EXAMPLE_DATA / "Switzerland.txt")
        assert cli.main([path, "--pretty", "-o", str(out)]) == 0
        text = out.read_text(encoding="utf-8")
        assert text.startswith("[\n  {")
        assert json.loads(text) == openair.parse_file(path)

    def test_normalize_flag(
        self, tmp_path: Path, capsys: pytest.CaptureFixture[str]
    ) -> None:
        src = tmp_path / "r.txt"
        src.write_text(CTR.replace("AC D", "AC R"), encoding="utf-8")
        assert cli.main([str(src), "--normalize-legacy-classes"]) == 0
        [airspace] = json.loads(capsys.readouterr().out)
        assert (airspace["class"], airspace["type"]) == ("UNC", "R")

    def test_error(self, tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
        assert cli.main([str(tmp_path / "missing.txt")]) == 1
        assert "Error:" in capsys.readouterr().err

    def test_python_m(self) -> None:
        result = subprocess.run(
            [sys.executable, "-m", "openair", "--version"],
            capture_output=True,
            text=True,
            check=True,
        )
        assert result.stdout.strip() == openair.__version__


def test_openair_parser_is_deprecated() -> None:
    with pytest.deprecated_call():
        assert openair.OpenAirParser.parse_string(CTR)


def test_version() -> None:
    assert openair.__version__ != "0+unknown"
