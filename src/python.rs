use std::io::BufReader;
use pyo3::prelude::*;
#[cfg(feature = "python")]
use serde_json;

// Re-export the main parse function from the library
pub use crate::parse;

/// Parse OpenAir airspace data from a string
#[pyfunction]
fn parse_openair_string(data: String) -> PyResult<String> {
    let mut reader = BufReader::new(data.as_bytes());
    match parse(&mut reader) {
        Ok(airspaces) => {
            // Serialize to JSON for easy Python consumption
            #[cfg(feature = "serde")]
            match serde_json::to_string(&airspaces) {
                Ok(json) => Ok(json),
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Failed to serialize airspaces: {}", e)
                ))
            }
            #[cfg(not(feature = "serde"))]
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Serde feature not enabled".to_string()
            ))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Failed to parse OpenAir data: {}", e)
        ))
    }
}

/// Parse OpenAir airspace data from a file path
#[pyfunction]
fn parse_openair_file(filepath: String) -> PyResult<String> {
    use std::fs::File;
    use std::io::BufReader;
    
    let file = File::open(&filepath)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(
            format!("Failed to open file '{}': {}", filepath, e)
        ))?;
    
    let mut reader = BufReader::new(file);
    match parse(&mut reader) {
        Ok(airspaces) => {
            // Serialize to JSON for easy Python consumption
            #[cfg(feature = "serde")]
            match serde_json::to_string(&airspaces) {
                Ok(json) => Ok(json),
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Failed to serialize airspaces: {}", e)
                ))
            }
            #[cfg(not(feature = "serde"))]
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Serde feature not enabled".to_string()
            ))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Failed to parse OpenAir file '{}': {}", filepath, e)
        ))
    }
}

/// A Python module for parsing OpenAir airspace files
#[pymodule]
fn openair(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_openair_string, m)?)?;
    m.add_function(wrap_pyfunction!(parse_openair_file, m)?)?;
    Ok(())
}
