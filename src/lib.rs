extern crate osmptparser;
extern crate num_cpus;

use pyo3::prelude::*;
use std::collections::HashMap;

use osmptparser::Parser as libParser;

#[pyclass]
pub struct Parser {
    p: libParser,
}

#[pyclass]
#[derive(Clone)]
pub struct Node {
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
        let gil = Python::acquire_gil();
        let py = gil.python();
        Ok(self.tags.clone().into_py(py))
    }

}

#[pyclass(dict)]
#[derive(Clone)]
pub struct ParseStatus {
    #[pyo3(get, set)]
    pub code: u64,
    #[pyo3(get, set)]
    pub detail: String,
}

#[pyclass(dict)]
pub struct PublicTransport {
    #[pyo3(get, set)]
    pub id: u64,
    pub tags: HashMap<String, String>,
    pub stops: Vec<Node>,
    pub geometry: Vec<Vec<(f64, f64)>>, // lon, lat
    pub status: ParseStatus,
}

#[pymethods]
impl PublicTransport {

    #[getter(tags)]
    fn get_tags(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        Ok(self.tags.clone().into_py(py))
    }

    #[getter(stops)]
    fn get_stops(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        Ok(self.stops.clone().into_py(py))
    }

    #[getter(geometry)]
    fn get_geometry(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let geom: Vec<PyObject> = self.geometry.iter().map(|v| {
            let v: Vec<PyObject> = v.iter().map(|(lon, lat)| (lon.to_object(py), lat.to_object(py)).into_py(py)).collect();
            v.into_py(py)
        }).collect();
        Ok(geom.into_py(py))
    }

    #[getter(status)]
    fn get_status(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        Ok(self.status.clone().into_py(py))
    }

}

#[pymethods]
impl Parser {

    #[new]
    fn new(obj: &PyRawObject, path: String, num_threads_option: Option<usize>) {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let num_threads = match num_threads_option {
            Some(nt) => nt,
            None    => num_cpus::get(),
        };
        let p = py.allow_threads(move || {
            libParser::new(&path, num_threads)
        });
        obj.init({
            Parser {
                p,
            }
        });
    }

    fn get_public_transports(&self, py: Python<'_>, gap: f64) -> PyResult<Vec<PublicTransport>> {
        let p = self.p.clone();
        let ret = py.allow_threads(move ||
            p.par_map(& move |r| {
                let (f, s) = r.flatten_ways(gap).unwrap();
                PublicTransport {
                    id: r.id,
                    tags: r.tags,
                    stops: r.stops.iter().map(|n| Node {
                        id: n.id,
                        tags: n.tags.clone(),
                        lon: n.lon,
                        lat: n.lat,
                    }).collect(),
                    geometry: f
                        .iter()
                        .map(|v| v.iter().map(|n| (n.lon, n.lat)).collect())
                        .collect(),
                    status: ParseStatus {
                        code: s.code,
                        detail: s.detail,
                    },
                }
            })
        );
        Ok(ret)
    }
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn pyosmptparser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Parser>()?;
    m.add_class::<PublicTransport>()?;
    m.add_class::<Node>()?;

    Ok(())
}
