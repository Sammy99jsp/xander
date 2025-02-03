use std::ops::Deref;

use super::{
    abilities::{Ability, AbilityMeta, Charisma, Dexterity, Intelligence, Strength, Wisdom},
    check::{Checkable, CheckableType},
};
use crate::utils::meta::Meta;
use paste::paste;

pub struct SkillMeta {
    name: &'static str,
    doc: &'static str,
    base_ability: &'static AbilityMeta,
    index: usize,
}

impl SkillMeta {
    pub const fn name(&self) -> &str {
        self.name
    }

    pub const fn doc(&self) -> &str {
        self.doc
    }

    pub const fn ability(&self) -> &AbilityMeta {
        self.base_ability
    }

    pub(super) const fn index(&self) -> usize {
        self.index
    }
}

pub trait Skill: Meta<SkillMeta> + Checkable {
    type Base: Ability;

    const META: &SkillMeta;
}

macro_rules! skill {
    {$id: ident($base: ty), doc: $doc: expr, index: $index: expr,} => {

        paste! {
            #[doc = $doc]
            #[derive(Debug, Clone, Copy)]
            pub struct $id;

            #[doc(hidden)]
            const [<META_ $id:snake:upper>] :&SkillMeta = &SkillMeta {
                name: stringify!([<$id:snake:upper>]),
                doc: $doc,
                base_ability: <$base as Ability>::META,
                index: $index,
            };

            impl Skill for $id {
                type Base = $base;
                const META: &SkillMeta = [<META_ $id:snake:upper>];
            }

            impl Meta<SkillMeta> for $id {
                fn meta(&self) -> &'static SkillMeta {
                    &[<META_ $id:snake:upper>]
                }
            }


            impl Checkable for $id {
                fn index(&self) -> usize {
                    <Self as Meta<SkillMeta>>::meta(self).index()
                }

                fn boxed_clone(&self) -> Box<dyn Checkable> {
                    Box::new(self.clone())
                }

                fn into_checkable_type(self) -> CheckableType
                    where Self: Sized
                {
                    CheckableType::Skill(Box::new(self))
                }
            }

            impl Deref for $id {
                type Target = SkillMeta;

                fn deref(&self) -> &Self::Target {
                    Self::META
                }
            }
        }

    };
}

/* Strength Skills */

skill! {
    Athletics(Strength),
    doc: "Your Strength (Athletics) check cover difficult situations you encounter while climbing jumping, or swimming.",
    index: 0,
}

/* Dexterity Skills */

skill! {
    Acrobatics(Dexterity),
    doc: "Your Dexterity (Acrobatics) check covers your attempt to stay on your feet in a tricky situation, such as when you're trying to run across a sheet of ice, balance on a tightrope, or stay upright on a rocking ship's deck.",
    index: 1,
}

skill! {
    SleightOfHand(Dexterity),
    doc: "Whenever you attempt an act of legerdemain or manual trickery, such as planting something on someone else or concealing an object on your person, make a Dexterity (Sleight of Hand) check.",
    index: 2,
}

skill! {
    Stealth(Dexterity),
    doc: "Make a Dexterity (Stealth) check when you attempt to conceal yourself from enemies, slink past guards, slip away without being noticed, or sneak up on someone without being seen or heard.",
    index: 3,
}

/* Intelligence Skills */

skill! {
    Arcana(Intelligence),
    doc: "Your Intelligence (Arcana) check measures your ability to recall lore about spells, magic items, eldritch symbols, magical traditions, the planes of existence, and the inhabitants of those planes.",
    index: 4,
}

skill! {
    History(Intelligence),
    doc: "Your Intelligence (History) check measures your ability to recall lore about historical events, legendary people, ancient kingdoms, past disputes, recent wars, and lost civilizations.",
    index: 5,
}

skill! {
    Investigation(Intelligence),
    doc: "When you look around for clues and make deductions based on those clues, you make an Intelligence (Investigation) check.",
    index: 6,
}

skill! {
    Nature(Intelligence),
    doc: "Your Intelligence (Nature) check measures your ability to recall lore about terrain, plants and animals, the weather, and natural cycles.",
    index: 7,
}

skill! {
    Religion(Intelligence),
    doc: "Your Intelligence (Religion) check measures your ability to recall lore about deities, rites and prayers, religious hierarchies, holy symbols, and the practices of secret cults.",
    index: 8,
}

/* Wisdom Skills */

skill! {
    AnimalHandling(Wisdom),
    doc: "When there is any question whether you can calm down a domesticated animal, keep a mount from getting spooked, or intuit an animal's intentions, the GM might call for a Wisdom (Animal Handling) check.",
    index: 9,
}

skill! {
    Insight(Wisdom),
    doc: "Your Wisdom (Insight) check decides whether you can determine the true intentions of a creature, such as when searching out a lie or predicting someone's next move.",
    index: 10,
}

skill! {
    Medicine(Wisdom),
    doc: "A Wisdom (Medicine) check lets you try to stabilize a dying companion or diagnose an illness.",
    index: 11,
}

skill! {
    Perception(Wisdom),
    doc: "Your Wisdom (Perception) check lets you spot, hear, or otherwise detect the presence of something.",
    index: 12,
}

skill! {
    Survival(Wisdom),
    doc: "The GM might ask you to make a Wisdom (Survival) check to follow tracks, hunt wild game, guide your group through frozen wastelands, identify signs that owlbears live nearby, predict the weather, or avoid quicksand and other natural hazards.",
    index: 13,
}

/* Charisma Skills */

skill! {
    Deception(Charisma),
    doc: "Your Charisma (Deception) check determines whether you can convincingly hide the truth, either verbally or through your actions.",
    index: 14,
}

skill! {
    Intimidation(Charisma),
    doc: "When you attempt to influence someone through overt threats, hostile actions, and physical violence, the GM might ask you to make a Charisma (Intimidation) check.",
    index: 15,
}

skill! {
    Performance(Charisma),
    doc: "Your Charisma (Performance) check determines how well you can delight an audience with music, dance, acting, storytelling, or some other form of entertainment.",
    index: 16,
}

skill! {
    Persuasion(Charisma),
    doc: "When you attempt to influence someone or a group of people with tact, social graces, or good nature, the GM might ask you to make a Charisma (Persuasion) check.",
    index: 17,
}
