extern crate num_cpus;
extern crate osmptparser;

use pyo3::prelude::*;
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
        let gil = Python::acquire_gil();
        let py = gil.python();
        Ok(self.tags.to_object(py))
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
        let gil = Python::acquire_gil();
        let py = gil.python();
        Ok(self.tags.to_object(py))
    }

    #[getter(info)]
    fn get_info(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        Ok(self.info.to_object(py))
    }

    #[getter(stops)]
    fn get_stops(&self) -> PyResult<Vec<Node>> {
        // let gil = Python::acquire_gil();
        // let py = gil.python();
        Ok(self.stops.clone()) //.into_iter().map(|s| s.to_object(py)).collect())
    }

    #[getter(geometry)]
    fn get_geometry(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        Ok(self.geometry.to_object(py))
        // let geom: Vec<PyObject> = self.geometry.iter().map(|v| {
        //     let v: Vec<PyObject> = v.iter().map(|(lon, lat)| (lon.to_object(py), lat.to_object(py)).to_object(py)).collect();
        //     v.to_object(py)
        // }).collect();
        // Ok(geom)
    }

    #[getter(status)]
    fn get_status(&self) -> PyResult<ParseStatus> {
        // let gil = Python::acquire_gil();
        // let py = gil.python();
        Ok(self.status.clone())
    }
}

#[pyclass]
struct Parser {
    p: libParser,
}

#[pymethods]
impl Parser {
    #[new]
    fn new(path: String, num_threads_option: Option<usize>) -> Self {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let num_threads = match num_threads_option {
            Some(nt) => nt,
            None => num_cpus::get(),
        };
        let p = py.allow_threads(move || libParser::new(&path, num_threads));
        Parser { p }
    }

    fn get_public_transports(&self, py: Python<'_>, gap: f64) -> PyResult<Vec<PublicTransport>> {
        let p = self.p.clone();
        let ret = py.allow_threads(move || {
            p.par_map(&move |r| {
                let f = r.flatten_ways(gap).unwrap();
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
fn pyosmptparser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Node>()?;
    m.add_class::<PublicTransport>()?;
    m.add_class::<Parser>()?;
    m.add_class::<ParseStatus>()?;

    Ok(())
}
