use pyo3::{pyclass, pymethods};

mod rs {
    pub(crate) use crate::core::stats::monster::speed::*;
}

use rs::SpeedType as _;

#[pyclass]
#[derive(Debug, Clone, Copy)]
pub struct SpeedType(pub(super) &'static rs::SpeedTypeMeta);

#[pymethods]
impl SpeedType {
    fn __repr__(&self) -> String {
        self.0.name.to_string()
    }

    fn _repr_html_(&self) -> String {
        format!(r#"<a href="\#" title="{}">{}</a>"#, self.0.description, self.0.name)
    }

}
pub const WALKING: SpeedType = SpeedType(rs::Walking::META);
pub const BURROWING: SpeedType = SpeedType(rs::Burrowing::META);
pub const CLIMBING: SpeedType = SpeedType(rs::Climbing::META);
pub const FLYING: SpeedType = SpeedType(rs::Flying::META);
pub const SWIMMING: SpeedType = SpeedType(rs::Swimming::META);
pub const CRAWLING: SpeedType = SpeedType(rs::Crawling::META);