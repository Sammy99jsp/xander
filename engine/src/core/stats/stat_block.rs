use crate::utils::Annotated;

use super::{
    abilities::{
        Charisma as CHA, Constitution as CON, Dexterity as DEX, Intelligence as INT,
        Strength as STR, Wisdom as WIS,
    },
    AbilityScore,
};

pub struct StatBlock {
    str: Annotated<STR, AbilityScore>,
    dex: Annotated<DEX, AbilityScore>,
    con: Annotated<CON, AbilityScore>,
    int: Annotated<INT, AbilityScore>,
    wis: Annotated<WIS, AbilityScore>,
    cha: Annotated<CHA, AbilityScore>,
}
