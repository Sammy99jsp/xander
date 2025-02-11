use std::{ops::Deref, sync::Arc};

use serde::Deserialize;

use super::stats::stat_block::StatBlock;
use crate::serde::StatBlockRaw;

#[derive(Clone, Debug, Deserialize)]
#[serde(from = "StatBlockRaw")]
pub struct Monster(pub Arc<StatBlock>);

impl From<StatBlockRaw> for Monster {
    fn from(value: StatBlockRaw) -> Self {
        Self(value.construct())
    }
}

impl Deref for Monster {
    type Target = StatBlock;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::dice;

    use super::Monster;

    #[test]
    fn test_parse_monster() {
        dice::random_seed();
        let raw = include_str!("../../tests/rat.json");
        let rat: Monster = serde_json::from_str(raw).expect("valid parse!");
        println!("Rat {}/{} HP", rat.hp(), rat.max_hp())
    }
}
