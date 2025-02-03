//!
//! # Cause API
//!
//! A [Cause] is something that has at least one [Effect] attached.
//! Effects will last as long as their causes.
//!
//!
//!
//! ### Implementation
//!

use std::{fmt::Debug, sync::Weak};

use crate::utils::meta::Meta;

pub enum CauseType {
    Property,
    Magic,
    Environment,
}

pub struct CauseMeta {
    ty: CauseType,
}

pub trait Cause: Meta<CauseMeta> + Debug {}

///
/// Outcomes due to a particular [Effect].
///
#[derive(Debug)]
pub enum Outcomes<R> {
    Contributes(Vec<R>),
    Overrides(Vec<R>),
}

///
/// Provides [Outcomes] as a result of a
/// [Cause].
///
pub struct Effect<Ctx, R> {
    cause: Weak<dyn Cause>,
    effect: Box<dyn Fn(&Ctx, *const ()) -> Outcomes<R>>,
}

impl<Ctx, R> Debug for Effect<Ctx, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Effect").field(&self.cause).finish()
    }
}
