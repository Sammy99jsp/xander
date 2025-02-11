//!
//! ## Monster Alignment
//!
//! Though the SRD does not explicitly describe
//! the alignments, It does contain monsters with
//! every kind of alignment.
//!
//! Technically, since descriptions are in PHB,
//! I can't add them here in source code.
//!

use serde::Deserialize;

/// > A monster's alignment provides a clue to its
/// > disposition and how it behaves in a roleplaying or
/// > combat situation.
///
/// SRD 5.1E, pg. 255
#[derive(PartialEq, Eq, Clone, Copy, Deserialize)]
#[serde(try_from = "String")]
pub struct Alignment(u8);

impl Default for Alignment {
    fn default() -> Self {
        Self::UNALIGNED
    }
}

#[rustfmt::skip]
impl Alignment {
    /* 00_XXX_YYY X = Good-evil axis, Y = Lawful-Chaotic axis */
    pub const UNALIGNED: Self       = Self(0);

    pub const LAWFUL_GOOD: Self     = Self(0b00_001_001);
    pub const LAWFUL_NEUTRAL: Self  = Self(0b00_010_001);
    pub const LAWFUL_EVIL: Self     = Self(0b00_100_001);

    pub const NEUTRAL_GOOD: Self    = Self(0b00_001_010);
    pub const TRUE_NEUTRAL: Self    = Self(0b00_010_010);
    pub const NEUTRAL_EVIL: Self    = Self(0b00_100_010);

    pub const CHAOTIC_GOOD: Self    = Self(0b00_001_100);
    pub const CHAOTIC_NEUTRAL: Self = Self(0b00_010_100);
    pub const CHAOTIC_EVIL: Self    = Self(0b00_100_100);
}

impl std::fmt::Debug for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Alignment::")?;

        let inner = match self {
            &Self::UNALIGNED => return f.write_str("UNALIGNED"),
            &Self::TRUE_NEUTRAL => return f.write_str("TRUE_NEUTRAL"),
            Self(inner) => inner,
        };

        const AXES: [&str; 6] = ["LAWFUL", "NEUTRAL", "CHAOTIC", "GOOD", "NEUTRAL", "EVIL"];
        AXES.iter()
            .enumerate()
            .filter(|(i, _)| ((1 << i) & inner) != 0)
            .intersperse((usize::MAX, &"_"))
            .try_for_each(|(_, val)| f.write_str(val))
    }
}

impl std::fmt::Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = match self {
            &Self::UNALIGNED => return f.write_str("unaligned"),
            &Self::TRUE_NEUTRAL => return f.write_str("neutral"),
            Self(inner) => inner,
        };

        const AXES: [&str; 6] = ["lawful", "neutral", "chaotic", "good", "neutral", "evil"];
        AXES.iter()
            .enumerate()
            .filter(|(i, _)| ((1 << i) & inner) != 0)
            .intersperse((usize::MAX, &" "))
            .try_for_each(|(_, val)| f.write_str(val))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UnknownAlignment;

impl std::fmt::Display for UnknownAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Unknown alignment")
    }
}

impl TryFrom<String> for Alignment {
    type Error = UnknownAlignment;

    #[rustfmt::skip]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let lower = value.to_lowercase();
        let (lawful_chaotic, good_evil) = match lower.as_str() {
            "unaligned" => return Ok(Self::UNALIGNED),
            "true neutral" | "neutral" => return Ok(Self::TRUE_NEUTRAL),
            v => v.split_once(" ").ok_or(UnknownAlignment)?,
        };

        let mut inner = 0;
        inner |= match lawful_chaotic {
            "lawful"  => 0b00_000_001,
            "neutral" => 0b00_000_010,
            "chaotic" => 0b00_000_100,
            _ => return Err(UnknownAlignment),
        };

        inner |= match good_evil {
            "good"    => 0b00_001_000,
            "neutral" => 0b00_010_000,
            "evil"    => 0b00_100_000,
            _ => return Err(UnknownAlignment)
        };

        Ok(Self(inner))
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use crate::core::stats::monster::alignment::Alignment;

    #[test]
    fn test_alignment_display() {
        assert_eq!(Alignment::UNALIGNED.to_string().as_str(), "unaligned");

        assert_eq!(Alignment::LAWFUL_GOOD.to_string().as_str(), "lawful good");
        assert_eq!(Alignment::NEUTRAL_GOOD.to_string().as_str(), "neutral good");
        assert_eq!(Alignment::CHAOTIC_GOOD.to_string().as_str(), "chaotic good");

        assert_eq!(
            Alignment::LAWFUL_NEUTRAL.to_string().as_str(),
            "lawful neutral"
        );
        assert_eq!(Alignment::TRUE_NEUTRAL.to_string().as_str(), "neutral");
        assert_eq!(
            Alignment::CHAOTIC_NEUTRAL.to_string().as_str(),
            "chaotic neutral"
        );

        assert_eq!(Alignment::LAWFUL_EVIL.to_string().as_str(), "lawful evil");
        assert_eq!(Alignment::NEUTRAL_EVIL.to_string().as_str(), "neutral evil");
        assert_eq!(Alignment::CHAOTIC_EVIL.to_string().as_str(), "chaotic evil");
    }

    #[test]
    fn test_alignment_parse() {
        assert_eq!(Ok(Alignment::UNALIGNED), "unaligned".to_string().try_into());
        assert_eq!(Ok(Alignment::TRUE_NEUTRAL), "true neutral".to_string().try_into());

        assert_eq!(Ok(Alignment::LAWFUL_GOOD), "lawful good".to_string().try_into());
        assert_eq!(Ok(Alignment::NEUTRAL_GOOD), "neutral good".to_string().try_into());
        assert_eq!(Ok(Alignment::CHAOTIC_GOOD), "chaotic good".to_string().try_into());

        assert_eq!(Ok(Alignment::LAWFUL_NEUTRAL), "lawful neutral".to_string().try_into());
        assert_eq!(Ok(Alignment::TRUE_NEUTRAL), "neutral".to_string().try_into());
        assert_eq!(Ok(Alignment::CHAOTIC_NEUTRAL), "chaotic neutral".to_string().try_into());

        assert_eq!(Ok(Alignment::LAWFUL_EVIL), "lawful evil".to_string().try_into());
        assert_eq!(Ok(Alignment::NEUTRAL_EVIL), "neutral evil".to_string().try_into());
        assert_eq!(Ok(Alignment::CHAOTIC_EVIL), "chaotic evil".to_string().try_into());
    }

    #[test]
    fn test_alignment_serde() {
        assert_matches!(serde_json::from_str(r#""unaligned""#), Ok(Alignment::UNALIGNED));
        assert_matches!(serde_json::from_str(r#""true neutral""#), Ok(Alignment::TRUE_NEUTRAL));

        assert_matches!(serde_json::from_str(r#""lawful good""#), Ok(Alignment::LAWFUL_GOOD));
        assert_matches!(serde_json::from_str(r#""neutral good""#), Ok(Alignment::NEUTRAL_GOOD));
        assert_matches!(serde_json::from_str(r#""chaotic good""#), Ok(Alignment::CHAOTIC_GOOD));

        assert_matches!(serde_json::from_str(r#""lawful neutral""#), Ok(Alignment::LAWFUL_NEUTRAL));
        assert_matches!(serde_json::from_str(r#""neutral""#), Ok(Alignment::TRUE_NEUTRAL));
        assert_matches!(serde_json::from_str(r#""chaotic neutral""#), Ok(Alignment::CHAOTIC_NEUTRAL));

        assert_matches!(serde_json::from_str(r#""lawful evil""#), Ok(Alignment::LAWFUL_EVIL));
        assert_matches!(serde_json::from_str(r#""neutral evil""#), Ok(Alignment::NEUTRAL_EVIL));
        assert_matches!(serde_json::from_str(r#""chaotic evil""#), Ok(Alignment::CHAOTIC_EVIL));
    }
}
