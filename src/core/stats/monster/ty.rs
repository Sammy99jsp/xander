//! ## Monster Type
//! > A monster's type speaks to its fundamental nature.
//! > Certain spells, magic items, class features, and other
//! > effects in the game interact in special ways with
//! > creatures of a particular type.
//!
//! -- SRD 5.1E, pg. 254
//!

use std::{collections::HashMap, sync::LazyLock};

use serde::Deserialize;

use crate::utils::meta::Meta;

#[derive(Debug)]
pub struct MonsterTypeMeta {
    pub name: &'static str,
    pub description: &'static str,
}

pub trait MonsterTypeI: Meta<MonsterTypeMeta> {
    const META: &'static MonsterTypeMeta;
}

#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd, Ord, Hash, Eq)]
pub struct MonsterTag(String);

impl std::fmt::Display for MonsterTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Deserialize)]
#[serde(from = "crate::serde::monster::CreatureTypeRaw")]
pub struct MonsterType {
    pub ty: &'static MonsterTypeMeta,
    pub tags: Vec<MonsterTag>,
}

impl MonsterType {
    pub const fn new<T: MonsterTypeI>(tags: Vec<MonsterTag>) -> Self {
        Self { ty: T::META, tags }
    }
}

impl std::fmt::Display for MonsterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ty.name)?;

        if !self.tags.is_empty() {
            write!(f, " (")?;

            self.tags
                .iter()
                .map(|MonsterTag(s)| s.as_str())
                .intersperse(", ")
                .try_for_each(|tag| f.write_str(tag))?;

            write!(f, ")")?;
        }

        Ok(())
    }
}

macro_rules! creature_type {
    ($id: ident, $meta: expr) => {
        pub struct $id;

        impl Meta<MonsterTypeMeta> for $id {
            fn meta(&self) -> &'static MonsterTypeMeta {
                <Self as MonsterTypeI>::META
            }
        }

        impl std::ops::Deref for $id {
            type Target = MonsterTypeMeta;

            fn deref(&self) -> &Self::Target {
                <Self as MonsterTypeI>::META
            }
        }

        impl MonsterTypeI for $id {
            const META: &'static MonsterTypeMeta = &$meta;
        }
    };
}

creature_type!(Aberration, MonsterTypeMeta {
	name: "Aberration",
	description: "Aberrations are utterly alien beings. Many of them have innate magical abilities drawn from the creature's alien mind rather than the mystical forces of the world. The quintessential aberrations are aboleths, beholders, mind flayers, and slaadi.",
});

creature_type!(Beast, MonsterTypeMeta {
	name: "Beast",
	description: "Beasts are nonhumanoid creatures that are a natural part of the fantasy ecology. Some of them have magical powers, but most are unintelligent and lack any society or language. Beasts include all varieties of ordinary animals, dinosaurs, and giant versions of animals.",
});

creature_type!(Celestial, MonsterTypeMeta {
	name: "Celestial",
	description: "Celestials are creatures native to the Upper Planes. Many of them are the servants of deities, employed as messengers or agents in the mortal realm and throughout the planes. Celestials are good by nature, so the exceptional celestial who strays from a good alignment is a horrifying rarity. Celestials include angels, couatls, and pegasi.",
});

creature_type!(Construct, MonsterTypeMeta {
	name: "Construct",
	description: "Constructs are made, not born. Some are programmed by their creators to follow a simple set of instructions, while others are imbued with sentience and capable of independent thought. Golems are the iconic constructs. Many creatures native to the outer plane of Mechanus, such as modrons, are constructs shaped from the raw material of the plane by the will of more powerful creatures.",
});

creature_type!(Dragon, MonsterTypeMeta {
	name: "Dragon",
	description: "Dragons are large reptilian creatures of ancient origin and tremendous power. True dragons, including the good metallic dragons and the evil chromatic dragons, are highly intelligent and have innate magic. Also in this category are creatures distantly related to true dragons, but less powerful, less intelligent, and less magical, such as wyverns and pseudodragons.",
});

creature_type!(Elemental, MonsterTypeMeta {
	name: "Elemental",
	description: "Elementals are creatures native to the elemental planes. Some creatures of this type are little more than animate masses of their respective elements, including the creatures simply called elementals. Others have biological forms infused with elemental energy. The races of genies, including djinn and efreet, form the most important civilizations on the elemental planes. Other elemental creatures include azers and invisible stalkers.",
});

creature_type!(Fey, MonsterTypeMeta {
	name: "Fey",
	description: "Fey are magical creatures closely tied to the forces of nature. They dwell in twilight groves and misty forests. In some worlds, they are closely tied to the Feywild, also called the Plane of Faerie. Some are also found in the Outer Planes, particularly the planes of Arborea and the Beastlands. Fey include dryads, pixies, and satyrs.",
});

creature_type!(Fiend, MonsterTypeMeta {
	name: "Fiend",
	description: "Fiends are creatures of wickedness that are native to the Lower Planes. A few are the servants of deities, but many more labor under the leadership of archdevils and demon princes. Evil priests and mages sometimes summon fiends to the material world to do their bidding. If an evil celestial is a rarity, a good fiend is almost inconceivable. Fiends include demons, devils, hell hounds, rakshasas, and yugoloths.",
});

creature_type!(Giant, MonsterTypeMeta {
	name: "Giant",
	description: "Giants tower over humans and their kind. They are humanlike in shape, though some have multiple heads (ettins) or deformities (fomorians). The six varieties of true giant are hill giants, stone giants, frost giants, fire giants, cloud giants, and storm giants. Besides these, creatures such as ogres and trolls are giants.",
});

creature_type!(Humanoid, MonsterTypeMeta {
	name: "Humanoid",
	description: "Humanoids are the main peoples of a fantasy gaming world, both civilized and savage, including humans and a tremendous variety of other species. They have language and culture, few if any innate magical abilities (though most humanoids can learn spellcasting), and a bipedal form. The most common humanoid races are the ones most suitable as player characters: humans, dwarves, elves, and halflings. Almost as numerous but far more savage and brutal, and almost uniformly evil, are the races of goblinoids (goblins, hobgoblins, and bugbears), orcs, gnolls, lizardfolk, and kobolds.",
});

creature_type!(Monstrosity, MonsterTypeMeta {
	name: "Monstrosity",
	description: "Monstrosities are monsters in the strictest sense—frightening creatures that are not ordinary, not truly natural, and almost never benign. Some are the results of magical experimentation gone awry (such as owlbears), and others are the product of terrible curses (including minotaurs and yuanti). They defy categorization, and in some sense serve as a catchall category for creatures that don't fit into any other type.",
});

creature_type!(Ooze, MonsterTypeMeta {
	name: "Ooze",
	description: "Oozes are gelatinous creatures that rarely have a fixed shape. They are mostly subterranean, dwelling in caves and dungeons and feeding on refuse, carrion, or creatures unlucky enough to get in their way. Black puddings and gelatinous cubes are among the most recognizable oozes.",
});

creature_type!(Plant, MonsterTypeMeta {
	name: "Plant",
	description: "Plants in this context are vegetable creatures, not ordinary flora. Most of them are ambulatory, and some are carnivorous. The quintessential plants are the shambling mound and the treant. Fungal creatures such as the gas spore and the myconid also fall into this category.",
});

creature_type!(Undead, MonsterTypeMeta {
	name: "Undead",
	description: "Undead are onceliving creatures brought to a horrifying state of undeath through the practice of necromantic magic or some unholy curse. Undead include walking corpses, such as vampires and zombies, as well as bodiless spirits, such as ghosts and specters.",
});

pub static REGISTRY: LazyLock<HashMap<&'static str, &'static MonsterTypeMeta>> =
    LazyLock::new(|| {
        let mut map = HashMap::new();
        map.insert("aberration", Aberration::META);
        map.insert("beast", Beast::META);
        map.insert("celestial", Celestial::META);
        map.insert("construct", Construct::META);
        map.insert("dragon", Dragon::META);
        map.insert("elemental", Elemental::META);
        map.insert("fey", Fey::META);
        map.insert("fiend", Fiend::META);
        map.insert("giant", Giant::META);
        map.insert("humanoid", Humanoid::META);
        map.insert("monstrosity", Monstrosity::META);
        map.insert("ooze", Ooze::META);
        map.insert("plant", Plant::META);
        map.insert("undead", Undead::META);
        map
    });

pub(crate) fn lookup<'de, D>(deserializer: D) -> Result<&'static MonsterTypeMeta, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct Visitor;

    impl serde::de::Visitor<'_> for Visitor {
        type Value = &'static MonsterTypeMeta;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, r#"Expected a valid CreatureType (e.g. "humanoid")"#)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            REGISTRY
                .get(v.to_lowercase().as_str())
                .copied()
                .ok_or_else(|| E::custom("Unknown creature type"))
        }
    }

    deserializer.deserialize_str(Visitor)
}
