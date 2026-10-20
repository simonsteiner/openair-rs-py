//! Library for reading and writing airspace files in `OpenAir` format (used by
//! flight instruments like Skytraxx and others).
//!
//! <http://www.winpilot.com/UsersGuide/UserAirspace.asp>
//!
//! ## Reading
//!
//! Use the [`parse`] function to read airspace files:
//!
//! ```no_run
//! # use std::fs::File;
//! # use std::io::BufReader;
//! let file = File::open("airspace.txt").unwrap();
//! let mut reader = BufReader::new(file);
//! let airspaces = openair::parse(&mut reader)
//!     .collect::<Result<Vec<_>, _>>()
//!     .unwrap();
//! ```
//!
//! ## Writing
//!
//! Use the [`write`] function to write airspace files:
//!
//! ```no_run
//! # use std::fs::File;
//! use openair::{Airspace, Altitude, Class, Coord, Geometry};
//!
//! let airspace = Airspace {
//!     name: Some("Example Zone".to_string()),
//!     class: Class::D,
//!     type_: None,
//!     lower_bound: Altitude::Gnd,
//!     upper_bound: Altitude::FlightLevel(100),
//!     geom: Geometry::Circle {
//!         centerpoint: Coord { lat: 47.0, lng: 8.0 },
//!         radius: 5.0,
//!     },
//!     frequency: None,
//!     call_sign: None,
//!     transponder_code: None,
//!     activation_times: None,
//! };
//!
//! let file = File::create("output.txt").unwrap();
//! openair::write(file, [&airspace]).unwrap();
//! ```
//!
//! ## Implementation Notes
//!
//! Unfortunately the `OpenAir` format is really underspecified. Every device
//! uses varying conventions. For example, there is nothing we can use as clear
//! delimiter for airspaces. Some files delimit airspaces with an empty line,
//! some with a comment. But on the other hand, some files even place comments
//! between the coordinates so that they cannot be used as delimiter either.
//!
//! This parser tries to be very lenient when parsing, based on real life data.
//! The end of an airspace is reached when the next one starts (with an `AC`
//! record) or when the file ends.
//!
//! Note: AT records (label placement hints) are currently ignored
#![deny(clippy::all)]

mod activations;
mod airspace;
mod airspace_types;
mod altitude;
mod classes;
mod coords;
mod geometry;
mod legacy_classes;
mod parser;
mod record;
mod writer;

#[cfg(feature = "python")]
mod python;

pub use crate::{
    activations::ActivationTimes,
    airspace::Airspace,
    airspace_types::AirspaceType,
    altitude::Altitude,
    classes::Class,
    coords::Coord,
    geometry::{Arc, ArcSegment, Direction, Geometry, PolygonSegment},
    legacy_classes::LegacyClassConflict,
    parser::parse,
    writer::write,
};
