use std::io::BufRead;

use log::debug;

use crate::{
    ActivationTimes, Airspace, AirspaceType, Altitude, Arc, ArcSegment, Class, Coord, Direction,
    Geometry, PolygonSegment, airspace::FALLBACK_NAME, record::Record,
};

struct OpenAirIterator<R: BufRead> {
    reader: R,
    line: Vec<u8>,
    use_buffered_line: bool,
    /// Tracks whether the last non-ignored record was a header record.
    /// Used to detect transitions from non-header to header, which indicate a new airspace.
    last_was_header: bool,
}

impl<R: BufRead> OpenAirIterator<R> {
    fn new(mut reader: R) -> Self {
        let mut line = Vec::new();
        let result = reader.read_until(b'\n', &mut line);
        let use_buffered_line = result.is_ok();

        // Skip UTF8 byte-order-mark
        if line.starts_with(&[0xEF, 0xBB, 0xBF]) {
            line.drain(0..3);
        }

        Self {
            reader,
            line,
            use_buffered_line,
            last_was_header: true,
        }
    }

    fn next_airspace(&mut self) -> Result<Option<Airspace>, String> {
        // Local variables for accumulating airspace data
        let mut name: Option<String> = None;
        let mut class: Option<Class> = None;
        let mut lower_bound: Option<Altitude> = None;
        let mut upper_bound: Option<Altitude> = None;
        let mut geom: Option<Geometry> = None;
        let mut type_: Option<AirspaceType> = None;
        let mut frequency: Option<String> = None;
        let mut call_sign: Option<String> = None;
        let mut transponder_code: Option<u16> = None;
        let mut activation_times: Option<ActivationTimes> = None;
        let mut var_x: Option<Coord> = None;
        let mut var_d: Option<Direction> = None;

        loop {
            let reached_eof = if self.use_buffered_line {
                // If we are supposed to use the buffered line, then we don't
                // involve the `reader` and just reset the flag instead.
                self.use_buffered_line = false;
                // We also apparently still have a line to process, so we are
                // not at the end of the file yet.
                false
            } else {
                // Otherwise, we should read a new line from the `reader`
                self.line.clear();
                let result = self.reader.read_until(b'\n', &mut self.line);
                let num_read = result.map_err(|e| format!("Could not read line: {e}"))?;
                // ... and if we haven't read any bytes, then we have reached
                // the end of the file.
                num_read == 0
            };

            // If we reached the end of the file, but there was no pending
            // airspace remaining, then we can "finish" the iterator.
            if reached_eof {
                // However, if we have accumulated an airspace, we should return it first
                if let Some(class) = class {
                    debug!("Finish {:?}", name);
                    let label = name.as_deref().unwrap_or(FALLBACK_NAME);
                    let lower_bound =
                        lower_bound.ok_or_else(|| format!("Missing lower bound for '{label}'"))?;
                    let upper_bound =
                        upper_bound.ok_or_else(|| format!("Missing upper bound for '{label}'"))?;
                    let geom = geom.ok_or_else(|| format!("Missing geom for '{label}'"))?;
                    return Ok(Some(Airspace {
                        name,
                        class,
                        type_,
                        lower_bound,
                        upper_bound,
                        geom,
                        frequency,
                        call_sign,
                        transponder_code,
                        activation_times,
                    }));
                }
                return Ok(None);
            }

            // Parse the line as a Record
            let line_str = String::from_utf8_lossy(&self.line);
            let trimmed = line_str.trim_start_matches('\u{feff}');
            let record = Record::parse(trimmed)?;

            // Check if we're transitioning from non-header to header records.
            // This indicates the start of a new airspace, so we should yield the current one.
            let is_header = record.is_header();
            let should_yield = is_header && !self.last_was_header && class.is_some();
            if should_yield {
                // Mark the current line as not consumed yet so that we can
                // reuse it in the `next()` iteration.
                self.use_buffered_line = true;

                // Build and return airspace from accumulated data
                debug!("Finish {:?}", name);
                let label = name.as_deref().unwrap_or(FALLBACK_NAME);
                let lower_bound =
                    lower_bound.ok_or_else(|| format!("Missing lower bound for '{label}'"))?;
                let upper_bound =
                    upper_bound.ok_or_else(|| format!("Missing upper bound for '{label}'"))?;
                let geom = geom.ok_or_else(|| format!("Missing geom for '{label}'"))?;
                // We already checked that class.is_some() in should_yield condition
                let class = class.unwrap();
                return Ok(Some(Airspace {
                    name,
                    class,
                    type_,
                    lower_bound,
                    upper_bound,
                    geom,
                    frequency,
                    call_sign,
                    transponder_code,
                    activation_times,
                }));
            }

            // Update state tracking for header/non-header transitions.
            // Display hints (AT, SP, SB) may sit between header records, so
            // they must not count as the end of the header block.
            let is_ignored = matches!(
                record,
                Record::Empty
                    | Record::Comment
                    | Record::LabelPlacement
                    | Record::Pen
                    | Record::Brush
            );
            if !is_ignored {
                self.last_was_header = is_header;
            }

            // Process the record
            match record {
                Record::Empty => {}
                Record::Comment => {}
                Record::LabelPlacement => {}
                Record::Pen => {}
                Record::Brush => {}
                Record::UnknownExtension(_) => {}
                Record::AirspaceClass(parsed_class) => {
                    if class.is_some() {
                        return Err("Could not set class (already defined)".to_string());
                    }
                    class = Some(parsed_class);
                }
                Record::AirspaceName(parsed_name) => {
                    if name.is_some() {
                        return Err("Could not set name (already defined)".to_string());
                    }
                    name = Some(parsed_name.to_string());
                }
                Record::LowerBound(altitude) => {
                    if lower_bound.is_some() {
                        return Err("Could not set lower_bound (already defined)".to_string());
                    }
                    lower_bound = Some(altitude);
                }
                Record::UpperBound(altitude) => {
                    if upper_bound.is_some() {
                        return Err("Could not set upper_bound (already defined)".to_string());
                    }
                    upper_bound = Some(altitude);
                }
                Record::AirspaceType(parsed_type) => {
                    if type_.is_some() {
                        return Err("Could not set type (already defined)".to_string());
                    }
                    type_ = Some(parsed_type);
                }
                Record::Frequency(parsed_freq) => {
                    if frequency.is_some() {
                        return Err("Could not set frequency (already defined)".to_string());
                    }
                    frequency = Some(parsed_freq.to_string());
                }
                Record::CallSign(parsed_call_sign) => {
                    if call_sign.is_some() {
                        return Err("Could not set call_sign (already defined)".to_string());
                    }
                    call_sign = Some(parsed_call_sign.to_string());
                }
                Record::TransponderCode(code) => {
                    if transponder_code.is_some() {
                        return Err("Could not set transponder_code (already defined)".to_string());
                    }
                    transponder_code = Some(code);
                }
                Record::ActivationTimes(parsed_times) => {
                    if activation_times.is_some() {
                        return Err("Could not set activation_times (already defined)".to_string());
                    }
                    activation_times = Some(parsed_times);
                }
                Record::VarX(coord) => {
                    var_x = Some(coord);
                }
                Record::VarD(direction) => {
                    var_d = Some(direction);
                }
                Record::Point(coord) => {
                    let segment = PolygonSegment::Point(coord);
                    match &mut geom {
                        None => {
                            geom = Some(Geometry::Polygon {
                                segments: vec![segment],
                            });
                        }
                        Some(Geometry::Polygon { segments }) => {
                            segments.push(segment);
                        }
                        Some(Geometry::Circle { .. }) => {
                            return Err("Cannot add a point to a circle".to_string());
                        }
                    }
                }
                Record::CircleRadius(radius) => match (&geom, &var_x) {
                    (None, Some(centerpoint)) => {
                        geom = Some(Geometry::Circle {
                            centerpoint: centerpoint.clone(),
                            radius,
                        });
                    }
                    (Some(_), _) => return Err("Geometry already set".to_string()),
                    (_, None) => return Err("Centerpoint missing".to_string()),
                },
                Record::ArcSegmentData {
                    radius,
                    angle_start,
                    angle_end,
                } => {
                    let centerpoint = var_x.clone().ok_or("Centerpoint missing".to_string())?;
                    let direction = var_d.unwrap_or_default();
                    let arc_segment = ArcSegment {
                        centerpoint,
                        radius,
                        angle_start,
                        angle_end,
                        direction,
                    };
                    let segment = PolygonSegment::ArcSegment(arc_segment);
                    match &mut geom {
                        None => {
                            geom = Some(Geometry::Polygon {
                                segments: vec![segment],
                            });
                        }
                        Some(Geometry::Polygon { segments }) => {
                            segments.push(segment);
                        }
                        Some(Geometry::Circle { .. }) => {
                            return Err("Cannot add a point to a circle".to_string());
                        }
                    }
                }
                Record::ArcData { start, end } => {
                    let centerpoint = var_x.clone().ok_or("Centerpoint missing".to_string())?;
                    let direction = var_d.unwrap_or_default();
                    let arc = Arc {
                        centerpoint,
                        start,
                        end,
                        direction,
                    };
                    let segment = PolygonSegment::Arc(arc);
                    match &mut geom {
                        None => {
                            geom = Some(Geometry::Polygon {
                                segments: vec![segment],
                            });
                        }
                        Some(Geometry::Polygon { segments }) => {
                            segments.push(segment);
                        }
                        Some(Geometry::Circle { .. }) => {
                            return Err("Cannot add a point to a circle".to_string());
                        }
                    }
                }
            }
        }
    }
}

impl<R: BufRead> Iterator for OpenAirIterator<R> {
    type Item = Result<Airspace, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_airspace().transpose()
    }
}

/// Process the reader until EOF, return an iterator over airspaces.
pub fn parse<R: BufRead>(reader: R) -> impl Iterator<Item = Result<Airspace, String>> {
    OpenAirIterator::new(reader)
}
