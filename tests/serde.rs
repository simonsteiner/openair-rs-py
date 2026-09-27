#![cfg(feature = "serde")]

use insta::assert_json_snapshot;
use openair::*;

#[test]
fn serialize_json() {
    let airspace = Airspace {
        name: Some("SUPERSPACE".into()),
        class: Class::Unknown("P".into()),
        lower_bound: Altitude::Gnd,
        upper_bound: Altitude::FeetAgl(3000),
        geom: Geometry::Polygon {
            segments: vec![
                PolygonSegment::Point(Coord { lat: 1.0, lng: 2.0 }),
                PolygonSegment::Point(Coord { lat: 1.1, lng: 2.0 }),
                PolygonSegment::Arc(Arc {
                    centerpoint: Coord {
                        lat: 1.05,
                        lng: 2.05,
                    },
                    start: Coord { lat: 1.1, lng: 2.0 },
                    end: Coord { lat: 1.0, lng: 2.1 },
                    direction: Direction::Cw,
                }),
                PolygonSegment::ArcSegment(ArcSegment {
                    centerpoint: Coord { lat: 3.0, lng: 3.0 },
                    radius: 1.5,
                    angle_start: 30.0,
                    angle_end: 45.0,
                    direction: Direction::Ccw,
                }),
                PolygonSegment::Point(Coord { lat: 1.0, lng: 2.0 }),
            ],
        },
        type_: Some(AirspaceType::Unknown("FUTURE".into())),
        frequency: None,
        call_sign: None,
        transponder_code: None,
        activation_times: None,
    };
    assert_json_snapshot!(airspace);
}

#[test]
fn serialize_json_ctr() {
    let airspace = Airspace {
        name: Some("Control Zone".into()),
        class: Class::Unclassified,
        lower_bound: Altitude::Gnd,
        upper_bound: Altitude::FeetAgl(1000),
        geom: Geometry::Polygon { segments: vec![] },
        type_: Some(AirspaceType::ControlZone),
        frequency: None,
        call_sign: None,
        transponder_code: None,
        activation_times: None,
    };
    assert_json_snapshot!(airspace);
}

#[test]
fn json_roundtrip_fixtures() {
    for fixture in [
        "example_data/Switzerland.txt",
        "example_data/Germany.txt",
        "example_data/Germany_Border.txt",
        "example_data/France.txt",
    ] {
        let data = std::fs::read(fixture).unwrap();
        let airspaces = openair::parse(data.as_slice())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let json = serde_json::to_string(&airspaces).unwrap();
        let back: Vec<openair::Airspace> = serde_json::from_str(&json).unwrap();
        assert_eq!(back, airspaces, "{fixture}");
    }
}

#[test]
fn deserialize_rejects_empty_class() {
    let json = r#"{"name":null,"class":"","lowerBound":{"type":"Gnd"},
        "upperBound":{"type":"Gnd"},"geom":{"type":"Polygon","segments":[]}}"#;
    let err = serde_json::from_str::<openair::Airspace>(json).unwrap_err();
    assert!(err.to_string().contains("Airspace class is empty"), "{err}");
}
