use std::io::Write;

use crate::{Airspace, Geometry, PolygonSegment, record::Record};

impl Airspace {
    /// Writes the airspace in OpenAir format.
    pub fn write<W: Write>(&self, mut writer: W) -> std::io::Result<()> {
        // 1. AC (class) - required
        Record::AirspaceClass(self.class.clone()).write(&mut writer)?;

        // 2. AY (type) - optional
        if let Some(ref type_) = self.type_ {
            Record::AirspaceType(type_.clone()).write(&mut writer)?;
        }

        // 3. AN (name) - optional
        if let Some(ref name) = self.name {
            Record::AirspaceName(name).write(&mut writer)?;
        }

        // 4. AL (lower bound) - required
        Record::LowerBound(self.lower_bound.clone()).write(&mut writer)?;

        // 5. AH (upper bound) - required
        Record::UpperBound(self.upper_bound.clone()).write(&mut writer)?;

        // 6. AF (frequency) - optional
        if let Some(ref frequency) = self.frequency {
            Record::Frequency(frequency).write(&mut writer)?;
        }

        // 7. AG (call sign) - optional
        if let Some(ref call_sign) = self.call_sign {
            Record::CallSign(call_sign).write(&mut writer)?;
        }

        // 8. AX (transponder code) - optional
        if let Some(transponder_code) = self.transponder_code {
            Record::TransponderCode(transponder_code).write(&mut writer)?;
        }

        // 9. AA (activation times) - optional
        if let Some(activation_times) = self.activation_times {
            Record::ActivationTimes(activation_times).write(&mut writer)?;
        }

        // 10. Geometry
        match &self.geom {
            Geometry::Circle {
                centerpoint,
                radius,
            } => {
                Record::VarX(centerpoint.clone()).write(&mut writer)?;
                Record::CircleRadius(*radius).write(&mut writer)?;
            }
            Geometry::Polygon { segments } => {
                for segment in segments {
                    match segment {
                        PolygonSegment::Point(coord) => {
                            Record::Point(coord.clone()).write(&mut writer)?;
                        }
                        PolygonSegment::ArcSegment(arc_segment) => {
                            Record::VarX(arc_segment.centerpoint.clone()).write(&mut writer)?;
                            Record::VarD(arc_segment.direction).write(&mut writer)?;
                            Record::ArcSegmentData {
                                radius: arc_segment.radius,
                                angle_start: arc_segment.angle_start,
                                angle_end: arc_segment.angle_end,
                            }
                            .write(&mut writer)?;
                        }
                        PolygonSegment::Arc(arc) => {
                            Record::VarX(arc.centerpoint.clone()).write(&mut writer)?;
                            Record::VarD(arc.direction).write(&mut writer)?;
                            Record::ArcData {
                                start: arc.start.clone(),
                                end: arc.end.clone(),
                            }
                            .write(&mut writer)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

/// Writes multiple airspaces in OpenAir format.
///
/// Airspaces are separated by blank lines.
pub fn write<'a, W: Write, I: IntoIterator<Item = &'a Airspace>>(
    mut writer: W,
    airspaces: I,
) -> std::io::Result<()> {
    for (i, airspace) in airspaces.into_iter().enumerate() {
        if i != 0 {
            // Write blank line between airspaces
            write!(writer, "\r\n")?;
        }
        airspace.write(&mut writer)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AirspaceType, Altitude, Arc, ArcSegment, Class, Coord, Direction};

    fn write_airspace(airspace: &Airspace) -> String {
        let mut buf = Vec::new();
        airspace.write(&mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }

    #[test]
    fn write_without_name() {
        let airspace = Airspace {
            name: None,
            class: Class::D,
            type_: None,
            lower_bound: Altitude::Gnd,
            upper_bound: Altitude::FlightLevel(100),
            geom: Geometry::Circle {
                centerpoint: Coord {
                    lat: 47.0,
                    lng: 8.0,
                },
                radius: 5.0,
            },
            frequency: None,
            call_sign: None,
            transponder_code: None,
            activation_times: None,
        };

        insta::assert_snapshot!(write_airspace(&airspace), @r"
        AC D
        AL GND
        AH FL100
        V X=47:00:00 N 008:00:00 E
        DC 5
        ");
    }

    #[test]
    fn write_minimal_circle() {
        let airspace = Airspace {
            name: Some("Test Zone".to_string()),
            class: Class::D,
            type_: None,
            lower_bound: Altitude::Gnd,
            upper_bound: Altitude::FlightLevel(100),
            geom: Geometry::Circle {
                centerpoint: Coord {
                    lat: 47.0,
                    lng: 8.0,
                },
                radius: 5.0,
            },
            frequency: None,
            call_sign: None,
            transponder_code: None,
            activation_times: None,
        };

        insta::assert_snapshot!(write_airspace(&airspace), @r"
        AC D
        AN Test Zone
        AL GND
        AH FL100
        V X=47:00:00 N 008:00:00 E
        DC 5
        ");
    }

    #[test]
    fn write_full_circle() {
        let airspace = Airspace {
            name: Some("Full Test Zone".to_string()),
            class: Class::Unknown("CTR".into()),
            type_: Some(AirspaceType::ControlZone),
            lower_bound: Altitude::FeetAmsl(1000),
            upper_bound: Altitude::FeetAmsl(5000),
            geom: Geometry::Circle {
                centerpoint: Coord {
                    lat: 46.5,
                    lng: 9.5,
                },
                radius: 10.0,
            },
            frequency: Some("123.45".to_string()),
            call_sign: Some("TOWER".to_string()),
            transponder_code: Some(7000),
            activation_times: Some("2023-12-16T12:00Z/2023-12-16T13:00Z".parse().unwrap()),
        };

        insta::assert_snapshot!(write_airspace(&airspace), @r"
        AC CTR
        AY CTR
        AN Full Test Zone
        AL 1000ft AMSL
        AH 5000ft AMSL
        AF 123.45
        AG TOWER
        AX 7000
        AA 2023-12-16T12:00:00.000+00:00/2023-12-16T13:00:00.000+00:00
        V X=46:30:00 N 009:30:00 E
        DC 10
        ");
    }

    #[test]
    fn write_polygon_with_points() {
        let airspace = Airspace {
            name: Some("Polygon Zone".to_string()),
            class: Class::A,
            type_: None,
            lower_bound: Altitude::Gnd,
            upper_bound: Altitude::Unlimited,
            geom: Geometry::Polygon {
                segments: vec![
                    PolygonSegment::Point(Coord {
                        lat: 47.0,
                        lng: 8.0,
                    }),
                    PolygonSegment::Point(Coord {
                        lat: 47.0,
                        lng: 9.0,
                    }),
                    PolygonSegment::Point(Coord {
                        lat: 46.0,
                        lng: 9.0,
                    }),
                ],
            },
            frequency: None,
            call_sign: None,
            transponder_code: None,
            activation_times: None,
        };

        insta::assert_snapshot!(write_airspace(&airspace), @r"
        AC A
        AN Polygon Zone
        AL GND
        AH UNLIM
        DP 47:00:00 N 008:00:00 E
        DP 47:00:00 N 009:00:00 E
        DP 46:00:00 N 009:00:00 E
        ");
    }

    #[test]
    fn write_polygon_with_arc_segment() {
        let airspace = Airspace {
            name: Some("Arc Segment Zone".to_string()),
            class: Class::Unknown("R".into()),
            type_: None,
            lower_bound: Altitude::FeetAgl(0),
            upper_bound: Altitude::FeetAmsl(3000),
            geom: Geometry::Polygon {
                segments: vec![
                    PolygonSegment::Point(Coord {
                        lat: 47.0,
                        lng: 8.0,
                    }),
                    PolygonSegment::ArcSegment(ArcSegment {
                        centerpoint: Coord {
                            lat: 47.0,
                            lng: 8.5,
                        },
                        radius: 10.0,
                        angle_start: 270.0,
                        angle_end: 290.0,
                        direction: Direction::Cw,
                    }),
                ],
            },
            frequency: None,
            call_sign: None,
            transponder_code: None,
            activation_times: None,
        };

        insta::assert_snapshot!(write_airspace(&airspace), @r"
        AC R
        AN Arc Segment Zone
        AL 0ft AGL
        AH 3000ft AMSL
        DP 47:00:00 N 008:00:00 E
        V X=47:00:00 N 008:30:00 E
        V D=+
        DA 10, 270, 290
        ");
    }

    #[test]
    fn write_polygon_with_arc() {
        let airspace = Airspace {
            name: Some("Arc Zone".to_string()),
            class: Class::Unknown("Q".into()),
            type_: None,
            lower_bound: Altitude::Gnd,
            upper_bound: Altitude::FlightLevel(50),
            geom: Geometry::Polygon {
                segments: vec![PolygonSegment::Arc(Arc {
                    centerpoint: Coord {
                        lat: 47.0,
                        lng: 8.0,
                    },
                    start: Coord {
                        lat: 47.0,
                        lng: 8.5,
                    },
                    end: Coord {
                        lat: 47.5,
                        lng: 8.0,
                    },
                    direction: Direction::Ccw,
                })],
            },
            frequency: None,
            call_sign: None,
            transponder_code: None,
            activation_times: None,
        };

        insta::assert_snapshot!(write_airspace(&airspace), @r"
        AC Q
        AN Arc Zone
        AL GND
        AH FL50
        V X=47:00:00 N 008:00:00 E
        V D=-
        DB 47:00:00 N 008:30:00 E, 47:30:00 N 008:00:00 E
        ");
    }
}
