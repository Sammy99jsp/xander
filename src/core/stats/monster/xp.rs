use std::sync::Weak;

use crate::{core::stats::stat_block::{CreatureType, StatBlock}, proxy_wrapper, utils::Proxy};

type XPValue = u32;
proxy_wrapper!(XP, Proxy<StatBlock, XPValue>);

impl XP {
    pub fn fixed(xp: XPValue, ctx: Weak<StatBlock>) -> Self {
        Self(Proxy::new(xp, ctx))
    }

    pub fn derived(ctx: Weak<StatBlock>) -> Self {
        Self(Proxy::derived(
            |this| match this.ty {
                CreatureType::Player => unimplemented!(),
                CreatureType::Monster(ref monster) => {
                    monster.cr.xp().expect("XP should always be defined")
                }
            },
            ctx,
        ))
    }
}

impl std::fmt::Display for XP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} XP", self.0.get()) // TODO: number formatting.
    }
}
