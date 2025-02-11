pub mod meta;
pub mod prettify;
pub mod proxy;
pub mod reactive;
pub mod legality;

use std::marker::PhantomData;

pub use proxy::{Proxy, ProxyPart};
pub use meta::Meta;

pub struct Annotated<Note, Value> {
    value: Value,

    note: PhantomData<Note>,
}

impl<Note: std::fmt::Debug, Value: std::fmt::Debug> std::fmt::Debug for Annotated<Note, Value> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} @ {:?}", self.value, self.note)
    }
}
