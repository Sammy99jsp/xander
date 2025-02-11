use std::{
    fmt::Debug,
    sync::{RwLock, Weak},
};

use serde::Deserialize;

use crate::{
    core::{combat::Combatant, stats::stat_block::StatBlock},
    proxy_wrapper,
    utils::{
        legality::{self, Legality},
        Proxy,
    },
};

use super::attack::AttackAction;

#[derive(Debug)]
pub struct ActionCtx {
    weak: Weak<Combatant>,
    used: RwLock<UsedActions>,
    max: MaxActions,
}

pub const NO_ACTIONS_LEFT_IN_TURN: legality::Reason = legality::Reason {
    id: "NO_ACTIONS_LEFT_IN_TURN",
};

impl ActionCtx {
    pub fn new(weak: Weak<Combatant>) -> Self {
        Self {
            weak,
            used: Default::default(),
            max: Default::default(),
        }
    }

    pub fn can_use(&self) -> Legality<()> {
        if self.used.read().unwrap().actions >= self.max.actions {
            Legality::Illegal(NO_ACTIONS_LEFT_IN_TURN)
        } else {
            Legality::Legal(())
        }
    }

    pub fn mark_used(&self) {
        self.used.write().unwrap().actions += 1;
    }

    pub fn used(&self) -> u32 {
        self.used.read().unwrap().actions
    }
}

#[derive(Debug, Default, Clone)]
pub struct UsedActions {
    actions: u32,
}

#[derive(Debug, Clone)]
pub struct MaxActions {
    // TODO: Account for Multiattack by using a proxy.
    actions: u32,
}

impl Default for MaxActions {
    fn default() -> Self {
        Self { actions: 1 }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum Action {
    Attack(AttackAction),
}

proxy_wrapper!(Actions, Proxy<StatBlock, Vec<Action>>);

impl Actions {
    pub const fn empty(ctx: Weak<StatBlock>) -> Self {
        Self(Proxy::new(Vec::new(), ctx))
    }

    pub const fn with_entries(ctx: Weak<StatBlock>, entries: Vec<Action>) -> Self {
        Self(Proxy::new(entries, ctx))
    }
}
