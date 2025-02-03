//!
//! Common implementations of [ProxyPart]
//!

use std::marker::PhantomData;

use super::{Dispatch, ProxyPart};

pub trait ProxyComputation<Ctx, Value> = for<'a> FnMut(&'a Ctx, &mut Value, Dispatch) + Send + Sync;

///
/// The proxy with no name!
///
/// An implementation of [ProxyPart] without the [Debug] requirement,
/// taking in only a [ProxyPart::compute] closure.
///
#[repr(transparent)]
pub struct Anonymous<Ctx, Value>(
    Box<dyn ProxyComputation<Ctx, Value>>,
    PhantomData<(Ctx, Value)>,
);

impl<Ctx, Value> Anonymous<Ctx, Value> {
    pub fn new<F>(f: F) -> Self
    where
        F: ProxyComputation<Ctx, Value> + 'static,
    {
        Self(Box::new(f), PhantomData)
    }
}

impl<Ctx, Value> std::fmt::Debug for Anonymous<Ctx, Value> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "???")
    }
}

impl<Ctx: Send + Sync, Value: Send + Sync> ProxyPart<Ctx, Value> for Anonymous<Ctx, Value> {
    fn compute(&mut self, ctx: &Ctx, prev: &mut Value, dispatch: Dispatch) {
        self.0.call_mut((ctx, prev, dispatch))
    }
}
