//! Legality.
//!

use std::{convert::Infallible, ops::ControlFlow};

/// Used to tell the agent whether a move is legal or not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Legality<T> {
    Legal(T),
    Illegal(Reason),
}

impl<T> Legality<T> {
    pub fn is_legal(&self) -> bool {
        match self {
            Legality::Legal(_) => true,
            Legality::Illegal(_) => false,
        }
    }
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Legality<U> {
        match self {
            Legality::Legal(t) => Legality::Legal(f(t)),
            Legality::Illegal(reason) => Legality::Illegal(reason),
        }
    }
}

impl<T> std::ops::Try for Legality<T> {
    type Output = T;

    type Residual = Legality<Infallible>;

    fn from_output(output: Self::Output) -> Self {
        Self::Legal(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Legality::Legal(t) => ControlFlow::Continue(t),
            Legality::Illegal(reason) => ControlFlow::Break(Legality::Illegal(reason)),
        }
    }
}

impl<T> std::ops::FromResidual for Legality<T> {
    fn from_residual(legality: Legality<Infallible>) -> Self {
        match legality {
            Legality::Illegal(reason) => Self::Illegal(reason),
        }
    }
}

pub trait ThenLegal<T>: Sized {
    fn then_legal_or(self, reason: Reason) -> Legality<T>;
}

impl ThenLegal<()> for bool {
    fn then_legal_or(self, reason: Reason) -> Legality<()> {
        match self {
            true => Legality::Legal(()),
            false => Legality::Illegal(reason),
        }
    }
}

/// The reason why something is illegal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reason {
    pub id: &'static str,
}
