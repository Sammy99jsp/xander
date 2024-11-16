use std::{collections::HashMap, ops::Index};

use crate::utils::Annotated;

use super::{
    abilities::{
        Ability, AbilityScoreBlock, Charisma as CHA, Constitution as CON, Dexterity as DEX,
        Intelligence as INT, Strength as STR, Wisdom as WIS,
    },
    AbilityScore,
};

pub struct StatBlock {
    ability_scores: AbilityScoreBlock,
}
