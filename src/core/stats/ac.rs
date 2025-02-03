//! Armor Class

use std::sync::Weak;

use crate::{
    core::{cause::Cause, dice::DExpr},
    proxy_wrapper,
    utils::{reactive::Lifespan, Proxy},
};

use super::stat_block::StatBlock;

#[derive(Debug, Clone)]
pub struct ACPart {
    pub(crate) source: Lifespan<dyn Cause>,
    pub(crate) ac: DExpr,
}

proxy_wrapper!(AC, Proxy<StatBlock, ACPart>);

impl AC {
    pub fn with_base(stat: Weak<StatBlock>, base_ac: ACPart) -> Self {
        Self(Proxy::new(base_ac, stat))
    }

    #[inline]
    pub fn value(&self) -> i32 {
        self.get().ac.result()
    }

    /// Checks if a "to hit" roll hits this AC.
    #[inline]
    pub fn does_hit(&self, to_hit: i32) -> bool {
        to_hit >= self.value()
    }
}
