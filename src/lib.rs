extern crate num_cpus;
extern crate osmptparser;

use pyo3::{prelude::*, types::PyType};
use std::collections::HashMap;

use osmptparser::Parser as libParser;

#[pyclass(dict)]
#[derive(Clone)]
struct Node {
    #[pyo3(get, set)]
    pub id: u64,
    pub tags: HashMap<String, String>,
    #[pyo3(get, set)]
    pub lon: f64,
    #[pyo3(get, set)]
    pub lat: f64,
}

#[pymethods]
impl Node {
    #[getter(tags)]
    fn get_tags(&self) -> PyResult<PyObject> {
        Ok(Python::with_gil(|py| self.tags.to_object(py)))
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

#[pyclass(dict)]
struct PublicTransport {
    #[pyo3(get, set)]
    pub id: u64,
    pub tags: HashMap<String, String>,
    pub info: HashMap<String, String>,
    pub stops: Vec<Node>,
    pub geometry: Vec<Vec<(f64, f64)>>, // lon, lat
    pub status: ParseStatus,
}

#[pymethods]
impl PublicTransport {
    #[getter(tags)]
    fn get_tags(&self) -> PyResult<PyObject> {
        Ok(Python::with_gil(|py| self.tags.to_object(py)))
    }

    #[getter(info)]
    fn get_info(&self) -> PyResult<PyObject> {
        Ok(Python::with_gil(|py| self.info.to_object(py)))
    }

    #[getter(stops)]
    fn get_stops(&self) -> PyResult<Vec<Node>> {
        Ok(self.stops.clone())
    }

    #[getter(geometry)]
    fn get_geometry(&self) -> PyResult<PyObject> {
        Ok(Python::with_gil(|py| self.geometry.to_object(py)))
    }

    #[getter(status)]
    fn get_status(&self) -> PyResult<ParseStatus> {
        Ok(self.status.clone())
    }
}

#[pyclass]
struct Parser {
    p: libParser,
}

#[pymethods]
impl Parser {
    #[classmethod]
    fn new_ptv2(
        cls: &Bound<'_, PyType>,
        path: String,
        num_threads_option: Option<usize>,
    ) -> PyResult<Self> {
        let num_threads = match num_threads_option {
            Some(nt) => nt,
            None => num_cpus::get(),
        };

        let p = Python::with_gil(|py| {
            py.allow_threads(move || libParser::new_ptv2(&path, num_threads))
        });
        Ok(Parser { p })
    }

    #[classmethod]
    fn new_aa(
        cls: &Bound<'_, PyType>,
        path: String,
        num_threads_option: Option<usize>,
    ) -> PyResult<Self> {
        let num_threads = match num_threads_option {
            Some(nt) => nt,
            None => num_cpus::get(),
        };

        let p =
            Python::with_gil(|py| py.allow_threads(move || libParser::new_aa(&path, num_threads)));
        Ok(Parser { p })
    }

    #[new]
    #[classmethod]
    fn py_new(
        cls: &Bound<'_, PyType>,
        path: String,
        filter: String,
        num_threads_option: Option<usize>,
    ) -> PyResult<Self> {
        let num_threads = match num_threads_option {
            Some(nt) => nt,
            None => num_cpus::get(),
        };

        let p = Python::with_gil(|py| {
            py.allow_threads(move || libParser::new(&path, num_threads, filter))
        });
        Ok(Parser { p })
    }

    fn get_public_transports(&self, py: Python<'_>, gap: f64) -> PyResult<Vec<PublicTransport>> {
        let p = self.p.clone();
        let ret = py.allow_threads(move || {
            p.par_map(&move |r| {
                let f = r.flatten_ways(gap, false).unwrap();
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
                    geometry: f
                        .0
                        .iter()
                        .map(|v| v.iter().map(|n| (n.lon, n.lat)).collect())
                        .collect(),
                    status: ParseStatus {
                        code: f.1.code,
                        detail: f.1.detail,
                    },
                }
            })
        });
        Ok(ret)
    }
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn pyosmptparser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Node>()?;
    m.add_class::<PublicTransport>()?;
    m.add_class::<Parser>()?;
    m.add_class::<ParseStatus>()?;

    Ok(())
}
