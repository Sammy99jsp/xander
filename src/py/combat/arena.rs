use std::sync::Weak;

use pyo3::{pyclass, pymethods};

mod rs {
    pub(crate) use crate::core::combat::arena::{Arena, SimpleArenaParams};
}

#[pyclass]
pub struct Arena(pub(super) Weak<dyn rs::Arena>);

#[pymethods]
impl Arena {
    fn __repr__(&self) -> String {
        "Arena".to_string()
    }

    #[cfg(feature = "vis")]
    fn _repr_html_(&self) -> String {
        self.0.upgrade().unwrap().as_ref().visualize().as_html()
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Simple(pub(super) rs::SimpleArenaParams);

#[pymethods]
impl Simple {
    #[new]
    fn __init__(width: u32, height: u32) -> Self {
        Self(rs::SimpleArenaParams::new(width as f32, height as f32))
    }
}
