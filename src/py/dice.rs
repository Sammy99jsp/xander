use std::str::FromStr;

use pyo3::{
    exceptions::{PyException, PyTypeError},
    pyclass, pymethods, PyObject, PyResult, Python,
};

use crate::core::dice::Die;

#[pymethods]
impl Die {
    #[new]
    fn __new__(sides: i32) -> Self {
        Self(sides)
    }

    #[getter(sides)]
    fn __sides(&self) -> i32 {
        self.0
    }

    fn __repr__(&self) -> String {
        format!("{}", self)
    }

    #[pyo3(name = "roll")]
    fn __roll(&self) -> i32 {
        self.roll()
    }

    #[pyo3(name = "advantage")]
    fn __advantage(&self) -> DExpr {
        DExpr(self.advantage())
    }

    #[pyo3(name = "disadvantage")]
    fn __disadvantage(&self) -> DExpr {
        DExpr(self.disadvantage())
    }

    #[pyo3(name = "qty")]
    fn __qty(&self, amount: u32) -> DExpr {
        DExpr(crate::core::dice::DExpr::Die {
            die: self.qty(amount),
            both_adv_dis: false,
        })
    }

    fn __add__(&self, rhs: PyObject) -> PyResult<DExpr> {
        Python::with_gil(|py| {
            // This is god awful, and involves a bunch of cloning.

            if let Ok(die) = rhs.extract::<Die>(py) {
                return Ok(DExpr(*self + die));
            }

            if let Ok(dexpr) = rhs.extract::<DExpr>(py) {
                return Ok(DExpr(*self + dexpr.0.clone()));
            }

            if let Ok(modifier) = rhs.extract::<i32>(py) {
                return Ok(DExpr(*self + modifier));
            }

            Err(PyTypeError::new_err("Cannot add to that type."))
        })
    }
}

#[doc(hidden)]
#[derive(Clone)]
#[pyclass(frozen)]
pub struct DExpr(pub(super) crate::core::dice::DExpr);

#[pymethods]
impl DExpr {
    #[new]
    fn new(raw: PyObject) -> PyResult<Self> {
        Python::with_gil(|py| {
            if let Ok(raw) = raw.extract::<String>(py) {
                return crate::core::dice::DExpr::from_str(&raw)
                    .map(DExpr)
                    .map_err(|errs| PyException::new_err(errs.to_string()));
            }

            Err(PyTypeError::new_err("Cannot create DExpr from that type."))
        })
    }

    fn __add__(&self, rhs: PyObject) -> PyResult<Self> {
        Python::with_gil(|py| {
            // This is god awful, and involves a bunch of cloning.

            if let Ok(die) = rhs.extract::<Die>(py) {
                return Ok(Self(self.0.clone() + die));
            }

            if let Ok(dexpr) = rhs.extract::<DExpr>(py) {
                return Ok(Self(self.0.clone() + dexpr.0.clone()));
            }

            if let Ok(modifier) = rhs.extract::<i32>(py) {
                return Ok(Self(self.0.clone() + modifier));
            }

            Err(PyTypeError::new_err("Cannot add to that type."))
        })
    }

    fn __repr__(&self) -> String {
        format!("{}", self.0)
    }

    fn advantage(&self) -> Self {
        Self(self.0.clone().advantage())
    }

    fn disadvantage(&self) -> Self {
        Self(self.0.clone().disadvantage())
    }

    fn evaluate(&self) -> DEvalTree {
        DEvalTree(self.0.evaluate())
    }
}

#[doc(hidden)]
#[derive(Clone)]
#[pyclass(frozen)]
pub struct DEvalTree(pub(in crate::py) crate::core::dice::DEvalTree);

#[pymethods]
impl DEvalTree {
    fn __repr__(&self) -> String {
        format!("{}", self.0)
    }

    fn result(&self) -> i32 {
        self.0.result()
    }
}
