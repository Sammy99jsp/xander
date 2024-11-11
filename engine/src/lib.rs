#![feature(rustc_attrs, int_roundings)]
#![allow(internal_features)]

pub mod core;
pub mod utils;

pub struct Ability {
    pub name: &'static str,
    pub short: &'static str,
    pub doc: &'static str,
}

pub const STRENGTH: Ability = Ability {
    name: "strength",
    short: "STR",
    doc: "measuring physical power",
};

pub const DEXTERITY: Ability = Ability {
    name: "dexterity",
    short: "DEX",
    doc: "measuring agility",
};

pub const CONSTITUTION: Ability = Ability {
    name: "constitution",
    short: "CON",
    doc: "measuring endurance",
};

pub const INTELLIGENCE: Ability = Ability {
    name: "intelligence",
    short: "INT",
    doc: "measuring reasoning and memory",
};

pub const WISDOM: Ability = Ability {
    name: "wisdom",
    short: "WIS",
    doc: "measuring perception and insight",
};

pub const CHARISMA: Ability = Ability {
    name: "charisma",
    short: "CHA",
    doc: "measuring force of personality",
};
