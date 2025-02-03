use paste::paste;

use crate::utils::meta::Meta;

use super::check::{Checkable, CheckableType};

pub trait Ability: Meta<AbilityMeta> + Copy + Checkable {
    const META: &AbilityMeta;
}

pub struct AbilityMeta {
    name: &'static str,
    short: &'static str,
    doc: &'static str,
    index: usize,
}

impl AbilityMeta {
    pub const fn name(&self) -> &str {
        self.name
    }

    pub const fn short(&self) -> &str {
        self.short
    }

    pub const fn doc(&self) -> &str {
        self.doc
    }

    pub(super) const fn index(&self) -> usize {
        self.index
    }
}

macro_rules! ability {
    ($id: ident, $meta: expr) => {
        paste! {

            #[derive(Debug, Clone, Copy)]
            pub struct $id;

            #[doc(hidden)]
            const [<META_ $id:snake:upper>]: &AbilityMeta = &$meta;
            impl Ability for $id {
                const META: &'static AbilityMeta = [<META_ $id:snake:upper>];
            }

            impl Meta<AbilityMeta> for $id {
                fn meta(&self) -> &'static AbilityMeta {
                    [<META_ $id:snake:upper>]
                }
            }

            impl std::ops::Deref for $id {
                type Target = AbilityMeta;

                fn deref(&self) -> &Self::Target {
                    Self::META
                }
            }

            impl Checkable for $id {
                fn index(&self) -> usize {
                    <Self as Meta<AbilityMeta>>::meta(self).index()
                }

                fn boxed_clone(&self) -> Box<dyn Checkable> {
                    Box::new(self.clone())
                }

                fn into_checkable_type(self) -> CheckableType
                    where Self: Sized
                {
                    CheckableType::Ability(Box::new(self))
                }
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
