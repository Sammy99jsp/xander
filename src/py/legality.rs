use pyo3::{
    pyclass, pymethods, types::PyNone, IntoPyObjectExt, Py, PyAny, PyErr, PyObject, PyResult,
    Python,
};

mod rs {
    pub(crate) use crate::utils::legality::*;
}

#[pyclass]
pub struct Legality {
    reason: Option<&'static str>,
    obj: Option<PyObject>,
}

#[pymethods]
impl Legality {
    fn is_legal(&self) -> bool {
        self.obj.is_some()
    }
    
    fn is_illegal(&self) -> bool {
        self.obj.is_none()
    }

    fn inner(&self) -> Option<&Py<PyAny>> {
        self.obj.as_ref().map(|obj| obj.as_any())
    }

    fn __repr__(&self) -> String {
        match self {
            Self {
                reason: Some(r),
                obj: None,
            } => format!("Illegal: {}", r),
            Self {
                reason: None,
                obj: Some(obj),
            } => obj.to_string(),
            _ => unreachable!(),
        }
    }

    fn _repr_html_(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| -> PyResult<PyObject> {
            match self {
                Self {
                    reason: Some(id),
                    obj: None,
                } => format!(r#"<div class="legality" style="display: grid; grid-template-columns: max-content 1fr; gap: 0 1em;"><span style="font-weight: bold;">Illegal</span><strong>{id}</strong></div>"#).into_py_any(py),
                Self {
                    reason: None,
                    obj: Some(obj),
                } => {
                    if let Ok(ret) = obj.call_method0(py, "_repr_html_") {
                        return Ok(ret);
                    };

                    if let Ok(ret) = obj.call_method0(py, "__repr__") {
                        return Ok(ret);
                    }

                    "<Unknown>".to_string().into_py_any(py)
                }
                _ => unreachable!(),
            }
        })
    }
}

impl Legality {
    pub fn void_success() -> PyResult<Self> {
        Python::with_gil(|py| -> PyResult<Self> {
            Ok(Self {
                reason: None,
                obj: Some(().into_bound_py_any(py)?.unbind()),
            })
        })
    }
}

impl<T: for<'a> IntoPyObjectExt<'a>> TryFrom<rs::Legality<T>> for Legality {
    type Error = PyErr;
    fn try_from(value: rs::Legality<T>) -> PyResult<Self> {
        Python::with_gil(|py| -> PyResult<Self> {
            let l = match value {
                rs::Legality::Legal(obj) => Self {
                    reason: None,
                    obj: Some(obj.into_bound_py_any(py)?.unbind()),
                },
                rs::Legality::Illegal(rs::Reason { id }) => Self {
                    reason: Some(id),
                    obj: None,
                },
            };

            Ok(l)
        })
    }
}
