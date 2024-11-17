//!
//! Runtime metadata for runtime tracking.
//!

use std::ops::Deref;

pub trait Meta<Table> : Deref<Target = Table> {
    fn meta(&self) -> &'static Table;
}

