//!
//! # Dice
//! Handling of dice rolls, adding modifiers, advantage and disadvantage, and more!
//!
//! ### Seeding
//! Xander supports seeding for its random number generation.
//!
//! You can set a fixed seed using [set_seed], or set a random
//! seed using [random_seed].
//!
//! **NOTE**: It is *REQUIRED* for you explicitly set a seed
//! for the RNG in a test mode before rolling -- otherwise, you'll get a panic.
//!
//! **NOTE**: You will also receive a warning if you don't explicitly set an RNG seed in debug mode,
//! however this will not be logged inb release mode.
//!
//! ### Dice Available
//! This module contains all the seven standard dice:
//! [D4], [D6], [D8], [D10], [D12], [D20], and [D100].
//!
//! **Note**: D100s are treated as one big 100-sided die,
//! and not as the result of two percentage dice (one D10 with sides x10, then a standard D10).
//!
//! ## Example
//! ```
//! use xander_engine::core::dice::{self, D20, D10, DExpr, DEvalTree};
//! dice::set_seed(0);
//!
//! let roll: i32 = D20.roll(); // Roll a d20
//!
//! // This is a dice expression:
//! // Roll a d20 and d10, then add the rolls together, then add 5.
//! let expr: DExpr = D20 + D10 + 5;
//!
//! // Rolling is deferred until you .evaluate() a dice expression.
//! // A DEvalTree represents the result of these dice rolls, but
//! // defers the calculation until you call .result()
//! let eval: DEvalTree = expr.evaluate();
//!
//! // The result of all the calculations: the final answer.
//! let res: i32 = eval.result();
//! ```
//!

pub mod expr;

use expr::QDie;
pub use expr::{DEvalTree, DExpr};
use pyo3::pyclass;
use serde::Deserialize;

use std::{
    cell::LazyCell,
    fmt::Display,
    sync::{Mutex, OnceLock},
};

use rand::{rngs::StdRng, thread_rng, Rng, RngCore, SeedableRng};

///
/// A generic `n`-sided die.
///
#[pyclass(frozen)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(try_from = "DExpr")]
pub struct Die(pub(crate) i32);

impl std::fmt::Debug for Die {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "d{}", self.0)
    }
}

impl std::fmt::Display for Die {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

pub struct NotADie;

impl Display for NotADie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "this expression is not a die.")
    }
}

impl TryFrom<DExpr> for Die {
    type Error = NotADie;

    fn try_from(value: DExpr) -> Result<Self, Self::Error> {
        match value {
            DExpr::Die {
                die: QDie(_, die), ..
            } => Ok(die),
            _ => Err(NotADie),
        }
    }
}

static SEED: OnceLock<u64> = OnceLock::new();

#[pyo3::pyfunction]
pub fn set_seed(seed: u64) -> bool {
    SEED.set(seed).is_ok()
}

#[pyo3::pyfunction]
pub fn random_seed() -> bool {
    SEED.set(thread_rng().next_u64()).is_ok()
}

thread_local! {
    static RNG: LazyCell<Mutex<StdRng>> = LazyCell::new(|| {
        let seed = match SEED.get() {
            Some(seed) => *seed,
            None => {
                if cfg!(test) {
                    panic!("dice::SEED not set in a test environment! This is required.\nTry setting the seed with dice::set_seed(..), or dice::random_seed().");
                }

                if cfg!(debug_assertions) {
                    eprintln!("dice::SEED has not been explicitly set.\nTo remove this nag, explicitly set the seed with dice::set_seed(..), or dice::random_seed().\nYou will not see this message in a release build.");
                }

                thread_rng().gen::<u64>()
            }
        };

        Mutex::new(StdRng::seed_from_u64(seed))
    });
}

impl Die {
    pub fn roll(&self) -> i32 {
        RNG.with(|rng| rng.lock().unwrap().gen_range(1..=self.0))
    }

    pub fn sides(&self) -> i32 {
        self.0
    }

    pub fn qty(self, amount: u32) -> QDie {
        QDie(amount, self)
    }
}

pub const D4: Die = Die(4);
pub const D6: Die = Die(6);
pub const D8: Die = Die(8);
pub const D10: Die = Die(10);
pub const D12: Die = Die(12);
pub const D20: Die = Die(20);
pub const D100: Die = Die(100);

#[cfg(test)]
mod tests {
    use crate::core::dice;

    use super::D6;

    #[test]
    #[should_panic]
    fn seed_sanity_test_panic() {
        D6.roll();
    }

    #[test]
    fn seed_sanity_test_non_panic() {
        dice::set_seed(0);
        D6.roll();
    }

    #[test]
    fn seed_sanity_test_non_panic_2() {
        dice::random_seed();
        D6.roll();
    }
}
