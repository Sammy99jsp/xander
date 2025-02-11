use serde::Deserialize;

use crate::core::{dice::DExpr, stats::damage::DamageTypeMeta};

use super::{Range, Targeting};

#[derive(Debug, Deserialize, Clone)]
#[serde(try_from = "crate::serde::combat::attack::MeleeAttackRaw")]
pub struct MeleeAttackAction {
    pub name: String,
    pub description: String,
    pub to_hit: DExpr,
    pub range: Range,
    pub target: Targeting,
    pub damage: Vec<(DExpr, &'static DamageTypeMeta)>,
}

