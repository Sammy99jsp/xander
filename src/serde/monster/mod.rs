pub mod speed;

use std::{fmt::Display, sync::Weak};

use serde::Deserialize;

mod rs {
    pub(crate) use crate::core::stats::{
        monster::ty::{lookup, MonsterTag, MonsterType, MonsterTypeMeta},
        monster::{Alignment, Monster, CR, XP},
        stat_block::StatBlock,
    };
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "MonsterRawRaw")]
pub struct MonsterRaw(MonsterRawRaw);

impl MonsterRaw {
    pub fn construct(self, this: Weak<rs::StatBlock>) -> rs::Monster {
        let Self(MonsterRawRaw {
            cr,
            xp,
            alignment,
            kind: ty,
        }) = self;

        rs::Monster {
            ty,
            cr,
            xp: {
                if let Some(xp) = xp {
                    rs::XP::fixed(xp, this)
                } else {
                    rs::XP::derived(this)
                }
            },
            alignment,
        }
    }
}

#[derive(Debug, Deserialize)]
struct MonsterRawRaw {
    // Can't call this 'type' because 'type': 'monster' is a thing already.
    kind: rs::MonsterType,

    cr: rs::CR,

    #[serde(default)]
    xp: Option<u32>,

    #[serde(default)]
    alignment: rs::Alignment,
}

pub struct IndeterminateXP;

impl Display for IndeterminateXP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "An XP could not be determined for this monster's CR. \
        Please manually add an 'xp' field for this monster."
        )
    }
}

impl TryFrom<MonsterRawRaw> for MonsterRaw {
    type Error = IndeterminateXP;

    fn try_from(value: MonsterRawRaw) -> Result<Self, Self::Error> {
        if value.xp.is_none() && value.cr == rs::CR::int(0).unwrap() {
            return Err(IndeterminateXP);
        }

        Ok(Self(value))
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum CRRaw {
    Str(String),
    Int(i64),
    Float(f64),
}

pub struct IncorrectCRFormat;

impl Display for IncorrectCRFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CR rating (0 to 30 inclusive' or '1/8', '1/4', '1/2'; or 0.125, 0.25, 0.5)"
        )
    }
}

impl TryFrom<CRRaw> for rs::CR {
    type Error = IncorrectCRFormat;

    fn try_from(value: CRRaw) -> Result<Self, Self::Error> {
        let int: u8 = match value {
            CRRaw::Str(s) => match s.as_str() {
                "1/8" => return Ok(Self::ONE_EIGHTH),
                "1/4" => return Ok(Self::ONE_QUARTER),
                "1/2" => return Ok(Self::ONE_HALF),
                int => int.parse().map_err(|_| IncorrectCRFormat)?,
            },
            CRRaw::Int(int) => u8::try_from(int).map_err(|_| IncorrectCRFormat)?,
            CRRaw::Float(float) => match float {
                0.125 => return Ok(Self::ONE_EIGHTH),
                0.25 => return Ok(Self::ONE_QUARTER),
                0.5 => return Ok(Self::ONE_HALF),
                f if f.fract() == 0.0 => u8::try_from(f as i64).map_err(|_| IncorrectCRFormat)?,
                _ => return Err(IncorrectCRFormat),
            },
        };

        rs::CR::int(int).ok_or(IncorrectCRFormat)
    }
}
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CreatureTypeRaw {
    #[serde(deserialize_with = "rs::lookup")]
    Ty(&'static rs::MonsterTypeMeta),

    WithTags {
        #[serde(rename = "type")]
        #[serde(deserialize_with = "rs::lookup")]
        ty: &'static rs::MonsterTypeMeta,
        tags: Vec<rs::MonsterTag>,
    },
}

impl From<CreatureTypeRaw> for rs::MonsterType {
    fn from(value: CreatureTypeRaw) -> Self {
        match value {
            CreatureTypeRaw::Ty(ty) => Self {
                ty,
                tags: Default::default(),
            },
            CreatureTypeRaw::WithTags { ty, tags } => Self { ty, tags },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::serde::monster::rs::CR;

    use super::MonsterRaw;

    #[test]
    fn test_cr_raw() {
        let se: CR = serde_json::from_str("0").expect("Valid parse");
        assert_eq!(se, CR::int(0).unwrap());

        let se: CR = serde_json::from_str("\"0\"").expect("Valid parse");
        assert_eq!(se, CR::int(0).unwrap());

        let se: CR = serde_json::from_str("0.0").expect("Valid parse");
        assert_eq!(se, CR::int(0).unwrap());

        let se: CR = serde_json::from_str("12").expect("Valid parse");
        assert_eq!(se, CR::int(12).unwrap());

        let se: CR = serde_json::from_str("\"12\"").expect("Valid parse");
        assert_eq!(se, CR::int(12).unwrap());

        let se: CR = serde_json::from_str("12.0").expect("Valid parse");
        assert_eq!(se, CR::int(12).unwrap());

        let se: CR = serde_json::from_str("30").expect("Valid parse");
        assert_eq!(se, CR::int(30).unwrap());

        let se: CR = serde_json::from_str("\"30\"").expect("Valid parse");
        assert_eq!(se, CR::int(30).unwrap());

        let se: CR = serde_json::from_str("30.0").expect("Valid parse");
        assert_eq!(se, CR::int(30).unwrap());

        let se: CR = serde_json::from_str("\"1/8\"").expect("Valid parse");
        assert_eq!(se, CR::ONE_EIGHTH);

        let se: CR = serde_json::from_str("\"1/4\"").expect("Valid parse");
        assert_eq!(se, CR::ONE_QUARTER);

        let se: CR = serde_json::from_str("\"1/2\"").expect("Valid parse");
        assert_eq!(se, CR::ONE_HALF);

        let se: CR = serde_json::from_str("0.125").expect("Valid parse");
        assert_eq!(se, CR::ONE_EIGHTH);

        let se: CR = serde_json::from_str("0.25").expect("Valid parse");
        assert_eq!(se, CR::ONE_QUARTER);

        let se: CR = serde_json::from_str("0.5").expect("Valid parse");
        assert_eq!(se, CR::ONE_HALF);
    }

    #[test]
    fn test_monster_raw() {
        let raw: MonsterRaw =
            serde_json::from_str(r#"{"cr": 1, "type": "aberration" }"#).expect("valid parse");
        println!("{raw:?}");
    }
}
