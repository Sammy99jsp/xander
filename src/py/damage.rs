use std::{fmt::Write, str::FromStr};

use pyo3::{
    exceptions::{PyException, PyTypeError, PyValueError},
    pyclass, pymethods, PyObject, PyResult, Python,
};

use crate::core::stats::damage::DamagePart;

mod rs {
    pub use crate::core::stats::damage::{Damage, DamageCause, DamageType, DamageTypeMeta};

    pub use crate::core::dice::{DEvalTree, DExpr};
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Damage(pub(in crate::py) rs::Damage);

#[pymethods]
impl Damage {
    fn __repr__(&self) -> String {
        let mut st = String::with_capacity(128);

        for part in &self.0 .0 {
            write!(&mut st, "{part}").unwrap();
        }

        st
    }

    fn _repr_html_(&self) -> String {
        let mut st = String::with_capacity(1024);

        write!(st, r#"<div style="display: grid; grid-auto-flow: column; grid-auto-columns: max-content; gap: 0 5px;">"#).unwrap();
        owo_colors::with_override(false, || {
            for (i, part) in self.0.0.iter().enumerate() {
                write!(&mut st, 
                r#"<div style="display: grid; grid-auto-flow: column; gap: 0 5px; align-items: center; height: 30px; padding: 2px 5px; border: 1px solid rgba(255, 255, 255, 0.2);">
                    <span style="font-weight: bold;">{}</span>
                    <span class="damage_type">{}</span>
                    <a style="margin-left: 2.5px; font-weight: bolder; display: block; aspect-ratio: 1 / 1; height: 20px; align-content: center; justify-content: center; display: flex; justify-self: center; text-decoration: none; color: rgb(63, 124, 172); background-color: rgba(63, 124, 172, 0.20); padding: 1px; border-radius: 4px;" title="{}">
                        <small style="display: block; align-self: center;">#{i}</small>
                    </a>
                </div>"#,
                    part.amount,
                    part.damage_type.name,
                    part.cause,
                ).unwrap();
            }
        });

        write!(st, "</div>").unwrap();

        st
    }

    fn __iadd__(&mut self, rhs: Damage) {
        self.0 += rhs.0;
    }

    fn __add__(&self, rhs: Damage) -> Damage {
        Self(self.0.clone() + rhs.0)
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct DamageCause(rs::DamageCause);

#[pymethods]
impl DamageCause {
    #[classattr]
    #[pyo3(name = "UNKNOWN")]
    fn unknown() -> Self {
        Self(rs::DamageCause::UNKNOWN)
    }
}

#[pyclass]
pub struct DamageType(&'static rs::DamageTypeMeta);

impl DamageType {
    pub(crate) fn of<D: rs::DamageType>(_: D) -> Self {
        Self(D::META)
    }
}

#[pymethods]
impl DamageType {
    fn name(&self) -> String {
        self.0.name().to_string()
    }

    fn description(&self) -> String {
        self.0.description.to_string()
    }

    fn __repr__(&self) -> String {
        self.0.name().to_string()
    }

    fn _repr_html_(&self) -> String {
        format!(r#"<span class="damage_type">{}</span>"#, self.0.name)
    }

    #[pyo3(signature = (amount, cause = DamageCause(rs::DamageCause::UNKNOWN)))]
    fn __call__(&self, amount: PyObject, cause: DamageCause) -> PyResult<Damage> {
        let amount = Python::with_gil(|py| {
            if let Ok(amount) = amount.extract::<i32>(py) {
                if amount <= 0 {
                    return Err(PyValueError::new_err("damage amount cannot be negative"));
                }

                return Ok(Box::new(rs::DEvalTree::Modifier(amount)));
            }

            if let Ok(amount) = amount.extract::<String>(py) {
                let Ok(amount) = rs::DExpr::from_str(&amount) else {
                    return Err(PyValueError::new_err("couldn't parse dice notation"));
                };

                return Ok(Box::new(amount.evaluate()));
            }

            if let Ok(amount) = amount.extract::<super::DExpr>(py) {
                return Ok(Box::new(amount.0.evaluate()));
            }

            Err(PyTypeError::new_err("expected str | int | DExpr"))
        })?;

        Ok(Damage(rs::Damage(vec![DamagePart {
            damage_type: self.0,
            amount,
            cause: cause.0,
            handling: Default::default(),
        }])))
    }
}
