//!
//! Runtime metadata for runtime tracking.
//!

pub trait Meta<Table> {
    fn meta(&self) -> &'static Table;
}
