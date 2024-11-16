use std::ops::Index;

use crate::utils::meta::Meta;

use super::{AbilityScore, Stat, StatType};

pub trait Ability: Meta<AbilityMeta> + Copy {
    const META: AbilityMeta;
}

impl<A: Ability> StatType for A {
    type Value = AbilityScore;

    const NAME: &'static str = Self::META.name;
}

pub struct AbilityMeta {
    pub name: &'static str,
    pub short: &'static str,
    pub doc: &'static str,
    index: usize,
}

macro_rules! ability {
    ($id: ident, $meta: expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $id;

        impl Ability for $id {
            const META: AbilityMeta = $meta;
        }
        impl Meta<AbilityMeta> for $id {
            fn meta(&self) -> &'static AbilityMeta {
                &<Self as Ability>::META
            }
        }
    };
}

ability!(
    Strength,
    AbilityMeta {
        name: "STRENGTH",
        short: "STR",
        doc: "measuring physical power",
        index: 0,
    }
);
ability!(
    Dexterity,
    AbilityMeta {
        name: "DEXTERITY",
        short: "DEX",
        doc: "measuring agility",
        index: 1,
    }
);
ability!(
    Constitution,
    AbilityMeta {
        name: "CONSTITUTION",
        short: "CON",
        doc: "measuring endurance",
        index: 2,
    }
);
ability!(
    Intelligence,
    AbilityMeta {
        name: "INTELLIGENCE",
        short: "INT",
        doc: "measuring reasoning and memory",
        index: 3,
    }
);
ability!(
    Wisdom,
    AbilityMeta {
        name: "WISDOM",
        short: "WIS",
        doc: "measuring perception and insight",
        index: 4,
    }
);
ability!(
    Charisma,
    AbilityMeta {
        name: "CHARISMA",
        short: "CHA",
        doc: "measuring force of personality",
        index: 5,
    }
);

#[repr(align(8))]
pub struct AbilityScoreBlock {
    str: Stat<Strength>,
    dex: Stat<Dexterity>,
    con: Stat<Constitution>,
    int: Stat<Intelligence>,
    wis: Stat<Wisdom>,
    cha: Stat<Charisma>,
}

impl std::fmt::Debug for AbilityScoreBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AbilityScoreBlock")
            .field("str", self.str.get())
            .field("dex", self.dex.get())
            .field("con", self.con.get())
            .field("int", self.int.get())
            .field("wis", self.wis.get())
            .field("cha", self.cha.get())
            .finish()
    }
}

impl AbilityScoreBlock {
    fn new(
        str: Stat<Strength>,
        dex: Stat<Dexterity>,
        con: Stat<Constitution>,
        int: Stat<Intelligence>,
        wis: Stat<Wisdom>,
        cha: Stat<Charisma>,
    ) -> Self {
        Self {
            str,
            dex,
            con,
            int,
            wis,
            cha,
        }
    }
}

impl<A: Ability + ?Sized> Index<A> for AbilityScoreBlock {
    type Output = AbilityScore;

    fn index(&self, _: A) -> &Self::Output {
        // SAFETY: We are aligned to 8-bytes,
        //         and indexes are all < 6 so our indexing is safe.
        let ptr = self as *const Self as *const AbilityScore;
        unsafe { ptr.add(A::META.index).as_ref_unchecked() }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::stats::{
        abilities::{Ability, Strength},
        AbilityScore, Stat,
    };

    use super::AbilityScoreBlock;

    const fn s<A: Ability>(score: u8) -> Stat<A> {
        Stat::new(AbilityScore::new(score).unwrap())
    }

    #[test]
    fn test_block() {
        let block = AbilityScoreBlock::new(s(15), s(17), s(12), s(9), s(10), s(7));
        println!("{block:#?}")
    }
}
