pub mod alignment;
pub mod cr;
pub mod ty;
pub mod xp;
pub mod speed;

pub use {
    alignment::Alignment,
    cr::CR,
    ty::{MonsterTag, MonsterType},
    xp::XP,
};

#[derive(Debug)]
pub struct Monster {
    pub ty: MonsterType,
    pub cr: CR,
    pub xp: XP,
    pub alignment: Alignment,
}
