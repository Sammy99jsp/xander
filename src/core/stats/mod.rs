pub mod abilities;
pub mod skills;
pub mod stat_block;
pub mod check;
pub mod damage;
pub mod health;
pub mod monsters;
pub mod ac;

///
/// Represents an score in an ability.
///
/// ### Memory Layout
///
/// A valid [AbilityScore] ranges from 1 to 30 (inclusive),
/// making use of `rustc` memory layout optimizations (niches) for [Option]<[AbilityScore]>, etc.
///
#[rustc_layout_scalar_valid_range_start(1)]
#[rustc_layout_scalar_valid_range_end(30)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct AbilityScore(u8);

impl std::fmt::Debug for AbilityScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for AbilityScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AbilityScore {
    const MIN: u8 = 1;
    const MAX_INCLUSIVE: u8 = 30;

    #[inline(always)]
    pub const fn new(raw_score: u8) -> Option<Self> {
        if Self::MIN <= raw_score && raw_score <= Self::MAX_INCLUSIVE {
            return Some(unsafe { Self(raw_score) });
        }

        None
    }

    #[inline(always)]
    pub const fn get(self) -> u8 {
        self.0
    }

    #[inline(always)]
    pub const fn modifier(&self) -> AbilityModifier {
        // SAFETY: We are 1..=30, therefore we should get
        //         -5..=10 as a result of the following expression,
        //         making it a valid [AbilityModifier].
        unsafe { AbilityModifier((self.0 as i8 - 10).div_floor(2)) }
    }
}

///
/// Represents a modifier to dice rolls.
///
/// ### Memory Layout
///
/// A valid [Modifier] ranges from -5 to +10 (inclusive),
/// making use of `rustc` memory layout optimizations (niches) for [Option]<[AbilityModifier]>, etc.
///
#[rustc_layout_scalar_valid_range_start(0b11111011)]
#[rustc_layout_scalar_valid_range_end(0b00001010)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct AbilityModifier(i8);

impl AbilityModifier {
    const MIN: i8 = -5;
    const MAX_INCLUSIVE: i8 = 10;

    #[inline(always)]
    pub const fn new(raw_modifier: i8) -> Option<Self> {
        if Self::MIN <= raw_modifier && raw_modifier <= Self::MAX_INCLUSIVE {
            return Some(unsafe { Self(raw_modifier) });
        }

        None
    }

    #[inline(always)]
    pub const fn get(self) -> i8 {
        self.0
    }
}

impl std::fmt::Display for AbilityModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "±0"),
            -5..=-1 => write!(f, "{}", self.0),
            1..=10 => write!(f, "+{}", self.0),
            _ => unreachable!("Ability modifier has invalid memory layout: (i8::MIN..-5) and (6..i8::MAX) should not be valid!")
        }
    }
}

impl std::fmt::Debug for AbilityModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::{AbilityModifier, AbilityScore};

    // Check against the definition of the SRD 5.1E table.
    #[test]
    #[rustfmt::skip]
    fn test_get_modifier() {
        let v = (1..=30)
            .map(AbilityScore::new)
            .flat_map(|score| score.as_ref().map(AbilityScore::modifier))
            .collect::<Vec<_>>();


        assert!(matches!(
            v.as_slice(),
            &[
                /* 1     */ AbilityModifier(-5),
                /* 2-3   */ AbilityModifier(-4), AbilityModifier(-4),
                /* 4-5   */ AbilityModifier(-3), AbilityModifier(-3),
                /* 6-7   */ AbilityModifier(-2), AbilityModifier(-2),
                /* 8-9   */ AbilityModifier(-1), AbilityModifier(-1),
                /* 10-11 */ AbilityModifier(0),  AbilityModifier(0),
                /* 12-13 */ AbilityModifier(1),  AbilityModifier(1),
                /* 14-15 */ AbilityModifier(2),  AbilityModifier(2),
                /* 16-17 */ AbilityModifier(3),  AbilityModifier(3),
                /* 18-19 */ AbilityModifier(4),  AbilityModifier(4),
                /* 20-21 */ AbilityModifier(5),  AbilityModifier(5),
                /* 22-23 */ AbilityModifier(6),  AbilityModifier(6),
                /* 24-25 */ AbilityModifier(7),  AbilityModifier(7),
                /* 26-27 */ AbilityModifier(8),  AbilityModifier(8),
                /* 28-29 */ AbilityModifier(9),  AbilityModifier(9),
                /* 30    */ AbilityModifier(10),
            ]
        ))
    }
}
