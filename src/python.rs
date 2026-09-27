//! Python bindings, built with `maturin` (feature `python`).
//!
//! Airspaces cross the boundary as plain Python dicts/lists in the same shape
//! as the serde JSON output; `python/openair/types.py` describes it.

use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

use pyo3::{
    exceptions::{PyIOError, PyValueError},
    prelude::*,
};
use pythonize::{depythonize, pythonize};

use crate::Airspace;

fn parse_airspaces<R: BufRead>(
    reader: R,
    normalize_legacy_classes: bool,
) -> PyResult<Vec<Airspace>> {
    let mut airspaces = crate::parse(reader)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| PyValueError::new_err(format!("Failed to parse OpenAir data: {e}")))?;
    if normalize_legacy_classes {
        for airspace in &mut airspaces {
            airspace
                .normalize_legacy_class()
                .map_err(|e| PyValueError::new_err(e.to_string()))?;
        }
    }
    Ok(airspaces)
}

fn to_python<'py>(py: Python<'py>, airspaces: &[Airspace]) -> PyResult<Bound<'py, PyAny>> {
    pythonize(py, airspaces)
        .map_err(|e| PyValueError::new_err(format!("Failed to convert airspaces: {e}")))
}

fn from_python(airspaces: &Bound<'_, PyAny>) -> PyResult<Vec<Airspace>> {
    depythonize(airspaces).map_err(|e| PyValueError::new_err(format!("Invalid airspace: {e}")))
}

/// Parse OpenAir airspace data from a string.
#[pyfunction]
#[pyo3(signature = (data, *, normalize_legacy_classes = false))]
fn parse_string<'py>(
    py: Python<'py>,
    data: &str,
    normalize_legacy_classes: bool,
) -> PyResult<Bound<'py, PyAny>> {
    let airspaces = py.detach(|| parse_airspaces(data.as_bytes(), normalize_legacy_classes))?;
    to_python(py, &airspaces)
}

/// Parse OpenAir airspace data from a file path.
#[pyfunction]
#[pyo3(signature = (path, *, normalize_legacy_classes = false))]
fn parse_file<'py>(
    py: Python<'py>,
    path: PathBuf,
    normalize_legacy_classes: bool,
) -> PyResult<Bound<'py, PyAny>> {
    let airspaces = py.detach(|| {
        let file = File::open(&path).map_err(|e| {
            PyIOError::new_err(format!("Failed to open file '{}': {e}", path.display()))
        })?;
        parse_airspaces(BufReader::new(file), normalize_legacy_classes)
    })?;
    to_python(py, &airspaces)
}

/// Write airspaces to a string in OpenAir format.
#[pyfunction]
fn write_string(py: Python<'_>, airspaces: &Bound<'_, PyAny>) -> PyResult<String> {
    let airspaces = from_python(airspaces)?;
    py.detach(|| {
        let mut buf = Vec::new();
        crate::write(&mut buf, &airspaces).map_err(|e| PyIOError::new_err(e.to_string()))?;
        String::from_utf8(buf).map_err(|e| PyValueError::new_err(e.to_string()))
    })
}

/// Write airspaces to a file in OpenAir format.
#[pyfunction]
fn write_file(py: Python<'_>, airspaces: &Bound<'_, PyAny>, path: PathBuf) -> PyResult<()> {
    let airspaces = from_python(airspaces)?;
    py.detach(|| {
        let file = File::create(&path).map_err(|e| {
            PyIOError::new_err(format!("Failed to create file '{}': {e}", path.display()))
        })?;
        let mut writer = BufWriter::new(file);
        crate::write(&mut writer, &airspaces)
            .and_then(|()| writer.flush())
            .map_err(|e| PyIOError::new_err(e.to_string()))
    })
}

/// A Python module for reading and writing OpenAir airspace files.
#[pymodule(gil_used = false)]
fn openair(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_string, m)?)?;
    m.add_function(wrap_pyfunction!(parse_file, m)?)?;
    m.add_function(wrap_pyfunction!(write_string, m)?)?;
    m.add_function(wrap_pyfunction!(write_file, m)?)?;
    Ok(())
}
