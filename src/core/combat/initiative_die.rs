//! [DExpr] for Initiative rolls.

use std::sync::Weak;

use crate::{
    core::{
        dice::{DExpr, D20},
        stats::stat_block::StatBlock,
    },
    proxy_wrapper,
    utils::Proxy,
};

proxy_wrapper!(InitiativeDie, Proxy<StatBlock, DExpr>);

impl InitiativeDie {
    pub fn new(ctx: Weak<StatBlock>) -> Self {
        Self(Proxy::derived(|ctx| D20 + ctx.modifiers.dexterity.get(), ctx))
    }
}
