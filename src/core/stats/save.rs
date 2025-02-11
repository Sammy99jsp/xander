use std::num::NonZeroU32;

use crate::{
    core::{cause::Cause, dice::DExpr},
    utils::{
        proxy::Dispatch,
        reactive::{Ephemeral, Lifespan},
        ProxyPart,
    },
};

use super::stat_block::StatBlock;

/// Saving throw advantage.
#[derive(Debug, Default)]
pub struct Advantage {
    cause: Lifespan<dyn Cause>,
}

impl ProxyPart<StatBlock, DExpr> for Advantage {
    fn compute(&mut self, _: &StatBlock, prev: &mut DExpr, mut dispatch: Dispatch<'_>) {
        // Do not apply advantage if the underlying cause is dead.
        // Delete this instead.
        if !self.cause.is_alive() {
            dispatch.destroy();
            return;
        }

        *prev = prev.clone().advantage();
    }
}

/// Saving throw disadvantage.
#[derive(Debug, Default)]
pub struct Disadvantage {
    cause: Lifespan<dyn Cause>,
}

impl ProxyPart<StatBlock, DExpr> for Disadvantage {
    fn compute(&mut self, _: &StatBlock, prev: &mut DExpr, mut dispatch: Dispatch<'_>) {
        // Do not apply disadvantage if the underlying cause is dead.
        // Delete this instead.
        if !self.cause.is_alive() {
            dispatch.destroy();
            return;
        }

        *prev = prev.clone().disadvantage();
    }
}

#[derive(Debug)]
pub struct Bonus {
    pub cause: Lifespan<dyn Cause>,
    pub bonus: DExpr,
    pub uses: Option<NonZeroU32>,
}
impl ProxyPart<StatBlock, DExpr> for Bonus {
    fn compute(&mut self, _: &StatBlock, prev: &mut DExpr, mut dispatch: Dispatch<'_>) {
        // Do not apply a bonus if the underlying cause is dead.
        // Delete this instead.
        if !self.cause.is_alive() {
            dispatch.destroy();
            return;
        }

        *prev = prev.clone() + self.bonus.clone();

        let Some(ref mut uses) = self.uses else {
            return;
        };

        *uses = match NonZeroU32::new(uses.get() - 1) {
            Some(u) => u,
            None => {
                dispatch.destroy();
                return;
            }
        };
    }
}
