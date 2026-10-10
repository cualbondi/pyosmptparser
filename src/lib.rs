use pyo3::prelude::*;
use pyo3::types::PyType;

use osmptparser::{Parser as LibParser, Tags};

fn threads(num_threads_option: Option<usize>) -> usize {
    num_threads_option.unwrap_or_else(num_cpus::get)
}

#[pyclass(dict)]
#[derive(Clone)]
struct Node {
    #[pyo3(get, set)]
    pub id: u64,
    pub tags: Tags,
    #[pyo3(get, set)]
    pub lon: f64,
    #[pyo3(get, set)]
    pub lat: f64,
}

#[pymethods]
impl Node {
    #[getter(tags)]
    fn get_tags<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        Ok(self.tags.as_ref().into_pyobject(py)?.into_any())
    }

    fn __repr__(&self) -> String {
        format!("Node(id={}, lon={}, lat={})", self.id, self.lon, self.lat)
    }
}

#[pyclass]
#[derive(Clone)]
struct ParseStatus {
    #[pyo3(get, set)]
    code: u64,
    #[pyo3(get, set)]
    detail: String,
}

#[pymethods]
impl ParseStatus {
    fn __repr__(&self) -> String {
        format!("ParseStatus(code={}, detail={:?})", self.code, self.detail)
    }
}

impl From<osmptparser::ParseStatus> for ParseStatus {
    fn from(status: osmptparser::ParseStatus) -> Self {
        ParseStatus {
            code: status.code(),
            detail: status.detail().to_string(),
        }
    }
}

#[pyclass(dict)]
struct PublicTransport {
    #[pyo3(get, set)]
    pub id: u64,
    pub tags: Tags,
    pub info: Tags,
    pub stops: Vec<Node>,
    pub geometry: Vec<Vec<(f64, f64)>>, // lon, lat
    pub status: ParseStatus,
}

#[pymethods]
impl PublicTransport {
    #[getter(tags)]
    fn get_tags<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        Ok(self.tags.as_ref().into_pyobject(py)?.into_any())
    }

    #[getter(info)]
    fn get_info<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        Ok(self.info.as_ref().into_pyobject(py)?.into_any())
    }

    #[getter(stops)]
    fn get_stops(&self) -> Vec<Node> {
        self.stops.clone()
    }

    #[getter(geometry)]
    fn get_geometry<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        Ok(self.geometry.clone().into_pyobject(py)?.into_any())
    }

    #[getter(status)]
    fn get_status(&self) -> ParseStatus {
        self.status.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "PublicTransport(id={}, name={:?}, status={})",
            self.id,
            self.tags.get("name").map(String::as_str).unwrap_or(""),
            self.status.code
        )
    }
}

#[pyclass]
struct Parser {
    p: LibParser,
}

#[pymethods]
impl Parser {
    /// Parse the public transport v2 relations of a .pbf file
    #[classmethod]
    #[pyo3(signature = (path, num_threads=None))]
    fn new_ptv2(
        _cls: &Bound<'_, PyType>,
        py: Python<'_>,
        path: String,
        num_threads: Option<usize>,
    ) -> PyResult<Self> {
        let n = threads(num_threads);
        let p = py.detach(move || LibParser::new_ptv2(&path, n))?;
        Ok(Parser { p })
    }

    /// Parse the administrative area relations of a .pbf file
    #[classmethod]
    #[pyo3(signature = (path, num_threads=None))]
    fn new_aa(
        _cls: &Bound<'_, PyType>,
        py: Python<'_>,
        path: String,
        num_threads: Option<usize>,
    ) -> PyResult<Self> {
        let n = threads(num_threads);
        let p = py.detach(move || LibParser::new_aa(&path, n))?;
        Ok(Parser { p })
    }

    /// Parse the relations and ways of a .pbf file matching an osmptparser tag filter
    /// (e.g. "route=bus,trolleybus&type=route")
    #[new]
    #[pyo3(signature = (path, filter, num_threads=None))]
    fn py_new(
        py: Python<'_>,
        path: String,
        filter: String,
        num_threads: Option<usize>,
    ) -> PyResult<Self> {
        let n = threads(num_threads);
        let p = py.detach(move || LibParser::new(&path, n, filter))?;
        Ok(Parser { p })
    }

    /// Build one PublicTransport per parsed relation, joining its ways with the given gap
    /// tolerance in meters
    fn get_public_transports(&self, py: Python<'_>, gap: f64) -> Vec<PublicTransport> {
        let p = self.p.clone();
        py.detach(move || {
            p.par_map(&move |r| {
                let (ways, status) = r.flatten_ways(gap, false);
                PublicTransport {
                    id: r.id,
                    tags: r.tags,
                    info: r.info,
                    stops: r
                        .stops
                        .iter()
                        .map(|n| Node {
                            id: n.id,
                            tags: n.tags.clone(),
                            lon: n.lon,
                            lat: n.lat,
                        })
                        .collect(),
                    geometry: ways
                        .iter()
                        .map(|v| v.iter().map(|n| (n.lon, n.lat)).collect())
                        .collect(),
                    status: status.into(),
                }
            })
        })
    }
}

/// Python bindings for osmptparser
#[pymodule]
fn pyosmptparser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<Node>()?;
    m.add_class::<PublicTransport>()?;
    m.add_class::<Parser>()?;
    m.add_class::<ParseStatus>()?;
    Ok(())
}
