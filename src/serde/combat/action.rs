use std::sync::Weak;

use serde::Deserialize;

use crate::core::stats::stat_block::StatBlock;

mod rs {
    pub(crate) use crate::core::combat::turn::action::{Action, Actions};
}

#[derive(Debug, Deserialize, Default)]
pub struct ActionsRaw(Vec<rs::Action>);

impl ActionsRaw {
    pub fn construct(self, weak: Weak<StatBlock>) -> rs::Actions {
        rs::Actions::with_entries(weak, self.0)
    }
}
