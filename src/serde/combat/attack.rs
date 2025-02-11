use serde::Deserialize;

use crate::serde::damage::DAMAGE_TYPES;

mod rs {
    pub use crate::core::{
        combat::turn::attack::{MeleeAttackAction, Range, Targeting},
        dice::DExpr,
    };
}

#[derive(Debug, Deserialize)]
pub struct MeleeAttackRaw {
    name: String,
    description: String,
    to_hit: rs::DExpr,
    range: rs::Range,
    
    #[serde(default)]
    target: rs::Targeting,
    damage: Vec<(rs::DExpr, String)>,
}

impl TryFrom<MeleeAttackRaw> for rs::MeleeAttackAction {
    type Error = String;

    fn try_from(value: MeleeAttackRaw) -> Result<Self, Self::Error> {
        let MeleeAttackRaw {
            name,
            description,
            to_hit,
            range,
            damage,
            target,
        } = value;

        Ok(rs::MeleeAttackAction {
            name,
            description,
            to_hit,
            target,
            range,
            damage: damage
                .into_iter()
                .map(|(dice, ty)| {
                    DAMAGE_TYPES
                        .get(&ty.to_lowercase().as_str())
                        .copied()
                        .map(|ty| (dice, ty))
                        .ok_or_else(|| "Unknown damage type".to_string())
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
