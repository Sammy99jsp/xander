use std::sync::{Arc, Weak};

use action::ActionCtx;

use super::{movement::MovementCtx, Combatant};

pub mod action;
pub mod attack;

#[derive(Debug)]
pub struct TurnCtx {
    pub movement: MovementCtx,
    pub actions: ActionCtx,
}

impl TurnCtx {
    pub fn new(weak: Weak<Combatant>) -> Self {
        Self {
            movement: MovementCtx::new(weak.clone()),
            actions: ActionCtx::new(weak),
        }
    }

    pub fn combatant(&self) -> &Weak<Combatant> {
        &self.movement.combatant
    }
}
