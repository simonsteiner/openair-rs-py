# OpenAir airspaces

Reading and writing airspace descriptions in the OpenAir text format, as used by
flight instruments and planning tools.

## Language

### File structure

**OpenAir file**:
A text file of records that describes airspaces.
_Avoid_: airspace file

**Record**:
One line of an OpenAir file, identified by its leading code (`AC`, `AN`, `DP`, …).
_Avoid_: line, command

**Airspace**:
Everything from one `AC` record up to the next `AC` record or the end of the file.
_Avoid_: zone, area

**Extension record**:
A record added by OpenAir v2 on top of the original spec: `AY`, `AF`, `AG`, `AX`, `AA`.

**Display record**:
A record that only affects how an airspace is drawn (`SP`, `SB`, `AT`); it carries no airspace data.
_Avoid_: style record, label hint

**Upstream**:
The Rust `openair` crate at glide-rs/openair-rs that this package forks and wraps.

### Classification

**Class**:
The `AC` token of an airspace: `A`–`G`, `UNC` for unclassified, or any other token kept verbatim.
_Avoid_: category

**Type**:
The `AY` token of an airspace, such as `CTR` or `R`.
_Avoid_: kind

**Legacy class**:
An `AC` token from the original spec that names a type rather than a class: `R`, `Q`, `P`, `CTR`, `GP`, `W`, `RMZ`, `TMZ`.
_Avoid_: legacy type, legacy token

**Normalization**:
Moving a legacy class into the type and setting the class to `UNC`.
_Avoid_: conversion, mapping

### Vertical extent

**Lower bound**:
The bottom of an airspace (`AL` record).
_Avoid_: floor, lower limit

**Upper bound**:
The top of an airspace (`AH` record).
_Avoid_: ceiling, upper limit

**Altitude**:
A lower or upper bound: ground, feet AMSL, feet AGL, a flight level, unlimited, or other. A height without a unit is feet AMSL.

**Other altitude**:
Altitude text that could not be read, kept verbatim instead of failing the parse.

### Horizontal extent

**Geometry**:
The horizontal shape of an airspace: a polygon or a circle.
_Avoid_: shape, boundary

**Polygon**:
A geometry made of segments; it may be open or closed.

**Segment**:
One piece of a polygon: a point (`DP`), an arc between two coordinates (`DB`), or an arc segment given by radius and angles (`DA`).
_Avoid_: vertex (a point is only one kind of segment)

**Circle**:
A geometry given by a centre point and a radius in nautical miles (`DC`).

### Operational data

**Frequency**:
The radio frequency of the controlling station (`AF`).

**Call sign**:
The radio call sign of the controlling station (`AG`).

**Transponder code**:
The transponder (squawk) code associated with an airspace (`AX`).
_Avoid_: squawk

**Activation times**:
The window in which an airspace is active (`AA`); either end may be open.
