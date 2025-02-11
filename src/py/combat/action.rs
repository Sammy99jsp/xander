use pyo3::{pyclass, pymethods};

mod py {
    pub(crate) use crate::py::combat::attack::Attack;
}

mod rs {
    pub(crate) use crate::core::combat::turn::{action::Action, attack::AttackAction};
}

#[pyclass]
pub struct Action(pub(in crate::py) rs::Action);

#[pymethods]
impl Action {
    fn __repr__(&self) -> String {
        todo!()
    }

    fn _repr_html_(&self) -> String {
        todo!()
    }

    #[allow(unreachable_patterns)]
    fn as_attack(&self) -> Option<py::Attack> {
        match &self.0 {
            rs::Action::Attack(attack) => Some(py::Attack(attack.clone())),
            _ => None,
        }
    }
}
