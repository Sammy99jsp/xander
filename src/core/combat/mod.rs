//!
//! Combat
//!

use std::{
    collections::HashMap,
    sync::{Arc, RwLock, Weak},
};

use arena::Arena;
use crossbeam_utils::atomic::AtomicCell;
use turn::TurnCtx;

use super::{geom::P3, stats::stat_block::StatBlock};

pub mod arena;
pub mod initiative_die;
pub mod movement;
pub mod turn;

pub struct Combat {
    pub initiative: Initiative,
    pub arena: Arc<dyn Arena>,
}

impl Combat {
    pub fn new<Cons, A>(ordering: impl Into<Option<InitiativeOrdering>>, cons: Cons) -> Arc<Self>
    where
        A: Arena + 'static,
        Cons: FnOnce(Weak<Self>) -> A,
    {
        Arc::new_cyclic(|this| Self {
            initiative: Initiative::new(ordering),
            arena: Arc::new(cons(this.clone())),
        })
    }

    pub fn len(&self) -> usize {
        self.initiative.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn step(&self) {
        // Advance this combat.
        // Note: This does not always imply a turn change by combatants.
        self.initiative.step()
    }
}

/// The members of a [Combat].
///
/// This newtype struct has the invariant that
/// the combat is always in some initiative order.
#[derive(Debug, Default)]
pub struct Initiative {
    members: RwLock<Vec<Arc<Combatant>>>,

    /// Ordering policy
    order: InitiativeOrdering,

    /// Cached orderings.
    orderings: RwLock<HashMap<(usize, usize), std::cmp::Ordering>>,

    /// Current turn.
    turn: RwLock<Option<Arc<turn::TurnCtx>>>,

    /// Current combatant
    current: AtomicCell<usize>,
    /// Rounds in this combat.
    rounds: AtomicCell<usize>,
}

impl Initiative {
    /// Creates a new, empty imitative ordering.
    pub fn new(ordering: impl Into<Option<InitiativeOrdering>>) -> Self {
        Self {
            order: ordering.into().unwrap_or_default(),
            ..Default::default()
        }
    }

    fn sort(&self) {
        self.members.write().unwrap().sort_by(|lhs, rhs| {
            let ordering = {
                self.orderings
                    .read()
                    .unwrap()
                    .get(&(Arc::as_ptr(lhs) as usize, Arc::as_ptr(rhs) as usize))
                    .copied()
            };

            if let Some(o) = ordering {
                o
            } else {
                // This .reverse() is very important -- we have highest imitative first!
                let o = lhs.initiative.cmp(&rhs.initiative, self.order).reverse();
                self.orderings
                    .write()
                    .unwrap()
                    .insert((Arc::as_ptr(lhs) as usize, Arc::as_ptr(rhs) as usize), o);
                o
            }
        });
    }

    /// Adds a new combatant to the initiative.
    ///
    /// This also makes use of a cache for the ordering,
    /// (some future orderings may be user-dependent, so this prevents
    /// from having to ask users constantly about previously established orderings).
    pub fn add(&self, member: Arc<Combatant>) {
        {
            self.members.write().unwrap().push(member);
        }

        self.sort();
    }

    /// Similar to [Self::add], but adds many combatants at once.
    pub fn extend(&mut self, members: impl IntoIterator<Item = Combatant>) {
        self.members
            .write()
            .unwrap()
            .extend(members.into_iter().map(Arc::new));
        self.sort();
    }

    pub fn as_vec(&self) -> Vec<Arc<Combatant>> {
        self.members.read().unwrap().iter().cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.members.read().unwrap().len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn current(&self) -> Arc<Combatant> {
        self.members.read().unwrap()[self.current.load()].clone()
    }

    pub fn current_turn(&self) -> Option<Arc<TurnCtx>> {
        self.turn.get_cloned().unwrap()
    }

    pub fn step(&self) {
        // Prompt whoever's turn it is to do something.

        if self.turn.read().unwrap().is_none() {
            // Update turn.
            self.turn
                .replace(Some(Arc::new(turn::TurnCtx::new(Arc::downgrade(
                    &self.current(),
                )))))
                .unwrap();
        }
    }

    pub fn advance_turn(&self) {
        self.current.fetch_add(1);

        // We've reached the end, let's start a new round.
        if self.current.load() >= self.len() {
            self.current.store(0);
            self.rounds.fetch_add(1);
        }

        self.turn
            .replace(Some(Arc::new(turn::TurnCtx::new(Arc::downgrade(
                &self.current(),
            )))))
            .unwrap();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InitiativeRoll(pub(crate) i32);

impl InitiativeRoll {
    /// Compare this imitative roll to another, using a defined [InitiativeOrdering].   
    #[inline]
    fn cmp(
        &self,
        rhs: &Self,
        ordering: impl Into<Option<InitiativeOrdering>>,
    ) -> std::cmp::Ordering {
        match ordering.into().unwrap_or_default() {
            InitiativeOrdering::Stable => self.0.cmp(&rhs.0),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum InitiativeOrdering {
    /// Stable ordering -- if a was originally first, and a == b, then a will come before b.
    #[default]
    Stable,
    // PlayerFirst,
    // Ask
}

#[derive(Debug)]
pub struct Combatant {
    pub combat: Weak<Combat>,
    pub name: String,
    pub initiative: InitiativeRoll,
    pub stats: Arc<StatBlock>,
    pub position: RwLock<P3>,
}
