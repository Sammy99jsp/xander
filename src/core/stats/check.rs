use std::{marker::PhantomData, ops::Deref};

use crate::{
    core::dice::{DEvalTree, D20},
    utils::meta::Meta,
};

use super::{
    abilities::{Ability, AbilityMeta},
    stat_block::StatBlock,
};

///
/// Represents a value for valid Difficulty Classes (DC)s.
///
/// ### Type Invariant
/// A [DCValue] cannot be [i16::MIN].
///
/// ### Memory Layout
///
/// A valid [DCValue] ranges from ([i16::MIN] + 1) to [i16::MAX] (inclusive),
/// making use of `rustc` memory layout optimization (niche) for [DCInner].
///
///
#[derive(Clone, Copy)]
#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(0x80_01)] // i16::MIN + 1
#[rustc_layout_scalar_valid_range_end(0x7F_FF)] // i16::MAX
pub struct DCValue(i16);

impl DCValue {
    const fn new(value: i16) -> Option<Self> {
        if value == i16::MIN {
            return None;
        }

        // SAFETY: value is not i32::MIN
        unsafe { Some(Self(value)) }
    }
}

impl std::fmt::Debug for DCValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

#[derive(Clone, Copy)]
pub enum DCInner {
    Known(DCValue),
    Unknown,
}

impl DCInner {
    const fn known(value: i16) -> Option<Self> {
        if let Some(value) = DCValue::new(value) {
            Some(Self::Known(value))
        } else {
            None
        }
    }

    const fn unknown() -> Self {
        Self::Unknown
    }
}

impl std::fmt::Debug for DCInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Known(value) => write!(f, "{value:?}"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

///
/// Difficulty Class
///
/// ---
///
/// > For every ability check, the GM decides which of the
/// > six abilities is relevant to the task at hand and the
/// > difficulty of the task, represented by a Difficulty Class ([DC]).
/// > The more difficult a task, the higher its DC.
///
#[derive(Clone, Copy)]
pub struct DC(DCInner);

impl DC {
    ///
    /// Tries to make a [DC] from a raw [i16],
    /// returning [None] if value == [i16::MIN].
    ///
    /// [i16::MIN] is not allowed, for memory optimization purposes.
    ///
    const fn known(value: i16) -> Option<Self> {
        if let Some(inner) = DCInner::known(value) {
            Some(Self(inner))
        } else {
            None
        }
    }

    const fn unknown() -> Self {
        Self(DCInner::unknown())
    }

    pub const fn inner(&self) -> Option<i16> {
        match self.0 {
            DCInner::Known(DCValue(raw)) => Some(raw),
            DCInner::Unknown => None,
        }
    }
}

/// From the "Typical Difficulty Classes table (pg. 77)"
impl DC {
    /// `DC(5)`
    pub const VERY_EASY: Self = DC::known(5).unwrap();
    /// `DC(10)`
    pub const EASY: Self = DC::known(10).unwrap();
    /// `DC(15)`
    pub const MEDIUM: Self = DC::known(15).unwrap();
    /// `DC(20)`
    pub const HARD: Self = DC::known(20).unwrap();
    /// `DC(25)`
    pub const VERY_HARD: Self = DC::known(25).unwrap();
    /// `DC(30)`
    pub const IMPOSSIBLE: Self = DC::known(30).unwrap();

    /// `DC(Unknown)`
    pub const UNKNOWN: Self = DC::unknown();
}

impl std::fmt::Debug for DC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DC").field(&self.0).finish()
    }
}

pub enum CheckableType {
    Ability(Box<dyn Checkable>),
    Skill(Box<dyn Checkable>),
}

impl Clone for CheckableType {
    fn clone(&self) -> Self {
        match self {
            Self::Ability(arg0) => Self::Ability(arg0.boxed_clone()),
            Self::Skill(arg0) => Self::Skill(arg0.boxed_clone()),
        }
    }
}

impl std::fmt::Debug for CheckableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ability(inner) | Self::Skill(inner) => inner.fmt(f),
        }
    }
}

pub trait Checkable: std::fmt::Debug {
    fn index(&self) -> usize;
    fn boxed_clone(&self) -> Box<dyn Checkable>;
    fn into_checkable_type(self) -> CheckableType
    where
        Self: Sized;
}

pub trait Activeness {
    const ACTIVE: bool;
}

pub struct Active;
impl Activeness for Active {
    const ACTIVE: bool = true;
}

pub struct Passive;
impl Activeness for Passive {
    const ACTIVE: bool = false;
}

#[derive(Clone)]
pub struct Check<Act: Activeness> {
    dc: DC,
    to_check: CheckableType,
    _a: PhantomData<Act>,
}

impl Check<Active> {
    ///
    /// Makes a new ([Active]) check.
    ///
    pub fn new(dc: DC, metric: impl Checkable) -> Self {
        Self {
            dc,
            to_check: metric.into_checkable_type(),
            _a: PhantomData,
        }
    }
}

impl Check<Passive> {
    ///
    /// Makes a new [Passive] check.
    ///
    /// > A passive check is a special kind of ability check that
    /// > doesn't involve any die rolls.
    ///
    pub fn passive(dc: DC, metric: impl Checkable) -> Self {
        Self {
            dc,
            to_check: metric.into_checkable_type(),
            _a: PhantomData,
        }
    }
}

impl<A: Activeness> std::fmt::Debug for Check<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut t = f.debug_tuple("Check");

        if !A::ACTIVE {
            t.field_with(|f| write!(f, "Passive"));
        }

        t.field(&self.to_check).field(&self.dc).finish()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Critical;

#[derive(Clone, Copy)]
pub enum RollOutcome {
    Pass,
    Fail,
    Indeterminate,
}

impl std::fmt::Debug for RollOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pass => write!(f, "Pass"),
            Self::Fail => write!(f, "Fail"),
            Self::Indeterminate => write!(f, "Indeterminate"),
        }
    }
}

///
/// The outcome of a [Check].
///
/// Contains the raw dice values via a [DEvalTree],
/// and the result via [Outcome].
///
#[derive(Clone)]
pub struct Outcome<Cause> {
    cause: Cause,
    roll_outcome: RollOutcome,
    roll: DEvalTree,
}

impl<Cause> Outcome<Cause> {
    pub fn is_pass(&self) -> bool {
        matches!(self.roll_outcome, RollOutcome::Pass)
    }

    pub fn is_fail(&self) -> bool {
        matches!(self.roll_outcome, RollOutcome::Fail)
    }

    pub fn value(&self) -> &DEvalTree {
        &self.roll
    }
}

impl<A: Activeness> Outcome<Check<A>> {
    pub fn check(&self) -> &Check<A> {
        &self.cause
    }
}

impl<Cause: std::fmt::Debug> std::fmt::Debug for Outcome<Cause> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Outcome")
            .field(&self.cause)
            .field(&self.roll_outcome)
            .field_with(|f| write!(f, "{} = {}", self.roll, self.roll.result()))
            .finish()
    }
}

impl StatBlock {
    pub fn check<A: Activeness>(&self, check: Check<A>) -> Outcome<Check<A>> {
        // SAFETY: check.to_check .index() returns a valid Ability::META/Skill::META .index
        let modifier = unsafe {
            match &check.to_check {
                CheckableType::Ability(a) => self.modifiers.get_index(a.index()).deref(),
                CheckableType::Skill(s) => self.skills.get_index(s.index()).deref(),
            }
        };

        // TODO: Add a "check" lookup for effectors.

        let roll = if A::ACTIVE {
            // Normal ("active") check.
            (D20 + modifier.get()).evaluate()
        } else {
            // Passive check.

            // TODO:    If the character has advantage on the check, add 5.
            //          For disadvantage, subtract 5.
            (10 + modifier.get()).evaluate()
        };

        // "If the total equals or exceeds the DC, the ability check is a success."
        let outcome = match check.dc {
            DC(DCInner::Unknown) => RollOutcome::Indeterminate,
            DC(DCInner::Known(DCValue(dc))) => {
                let success = roll.result() >= (dc as i32);
                if success {
                    RollOutcome::Pass
                } else {
                    RollOutcome::Fail
                }
            }
        };

        Outcome {
            cause: check,
            roll_outcome: outcome,
            roll,
        }
    }
}

// TODO: Group Checks

trait Saveable: Meta<AbilityMeta> + std::fmt::Debug {}
impl<S> Saveable for S where S: Meta<AbilityMeta> + std::fmt::Debug {}

///
/// A saving throw.
///
/// ---
///
/// > A saving throw -- also called a save -- represents an
/// > attempt to resist a spell, a trap, a poison, a disease,
/// > or a similar threat.
///
pub struct Save {
    dc: DC,
    // Cheating here, because Ability is not dyn object-safe.
    ability: Box<dyn Saveable>,
}

impl std::fmt::Debug for Save {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Save")
            .field(&self.ability)
            .field(&self.dc)
            .finish()
    }
}

impl Save {
    pub fn new<A>(dc: DC, ability: A) -> Self
    where
        A: Ability + 'static,
    {
        Self {
            dc,
            ability: Box::new(ability),
        }
    }
}

impl StatBlock {
    ///
    /// Make a saving throw.
    ///
    pub fn save(&self, save: Save) -> Outcome<Save> {
        // To make a saving throw, roll a d20 and add the
        // appropriate ability modifier.
        let modifier = unsafe { self.modifiers.get_index(save.ability.index()).deref() };

        // TODO: A saving throw can be modified by a situational
        // bonus or penalty and can be affected by advantage
        // and disadvantage, as determined by the GM.

        let roll = (D20 + modifier.get()).evaluate();

        let roll_outcome = match &save.dc {
            DC(DCInner::Known(DCValue(dc))) => {
                if roll.result() >= *dc as i32 {
                    RollOutcome::Pass
                } else {
                    RollOutcome::Fail
                }
            }
            DC(DCInner::Unknown) => RollOutcome::Indeterminate,
        };

        Outcome {
            cause: save,
            roll_outcome,
            roll,
        }
    }
}
