//! ## Challenge Rating

use std::num::NonZeroU32;

use serde::Deserialize;
use crate::serde::monster::CRRaw;

use crate::core::dice::DExpr;

#[rustfmt::skip]
pub const VALID_CRS: [&str; 34] = ["0", "1/8", "1/4", "1/2", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30"];

#[doc(hidden)]
#[rustfmt::skip]
const CR_PROFICIENCY_BONUSES: [i32; 34] = [2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 8, 9, 9];

#[doc(hidden)]
#[rustfmt::skip]
const CHALLENGE_TO_XP: [Option<NonZeroU32>; 34] = [None, NonZeroU32::new(25), NonZeroU32::new(50), NonZeroU32::new(100), NonZeroU32::new(200), NonZeroU32::new(450), NonZeroU32::new(700), NonZeroU32::new(1_100), NonZeroU32::new(1_800), NonZeroU32::new(2_300), NonZeroU32::new(2_900), NonZeroU32::new(3_900), NonZeroU32::new(5_000), NonZeroU32::new(5_900), NonZeroU32::new(7_200), NonZeroU32::new(8_400), NonZeroU32::new(10_000), NonZeroU32::new(11_500), NonZeroU32::new(13_000), NonZeroU32::new(15_000), NonZeroU32::new(18_000), NonZeroU32::new(20_000), NonZeroU32::new(22_000), NonZeroU32::new(25_000), NonZeroU32::new(33_000), NonZeroU32::new(41_000), NonZeroU32::new(50_000), NonZeroU32::new(62_000), NonZeroU32::new(75_000), NonZeroU32::new(90_000), NonZeroU32::new(105_000), NonZeroU32::new(120_000), NonZeroU32::new(135_000), NonZeroU32::new(155_000)];
//                                                 ^^^^ XP for a level 0 monster is not defined: either 0 or 10.
//                                                      So we will leave the user to define this.

///
/// Represents a valid Challenge Rating.
///
/// ### Memory Layout
///
/// Challenge ratings are stored as u8 indices into
/// the [VALID_CRS] table.
///
/// A valid [CR] index ranges from 0 to 33 (inclusive) -- comprising of
/// integers 0 to 30 inclusive; and 1/8, NonZeroU16::new_unchecked(1)/4, and 1/2.
/// We make use of rustc memory layout optimizations (niches) for [Option]<[CR]>, etc
/// by telling the compiler it can use values of 34 and over.
///
#[rustc_layout_scalar_valid_range_start(0)]
#[rustc_layout_scalar_valid_range_end(33)]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(try_from = "CRRaw")]
pub struct CR(u8);

impl CR {
    // Fractional ratings.
    pub const ONE_EIGHTH: Self = unsafe { Self(1) };
    pub const ONE_QUARTER: Self = unsafe { Self(2) };
    pub const ONE_HALF: Self = unsafe { Self(3) };

    /// Get an integer CR.
    /// Returns [None] if the rating is above 30.
    pub const fn int(rating: u8) -> Option<Self> {
        if rating > 30 {
            return None;
        }

        if rating == 0 {
            return Some(unsafe { Self(0) });
        }

        Some(unsafe { Self(rating + 3) })
    }

    /// Get the proficiency bonus attached for this CR.
    pub const fn proficiency_bonus(&self) -> DExpr {
        DExpr::Constant(CR_PROFICIENCY_BONUSES[self.0 as usize])
    }

    /// Get the XP for this CR, if defined (i.e. not CR 0).
    pub const fn xp(&self) -> Option<u32> {
        match CHALLENGE_TO_XP[self.0 as usize] {
            Some(xp) => Some(xp.get()),
            None => None,
        }
    }
}

impl std::fmt::Debug for CR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CR({})", VALID_CRS[self.0 as usize])
    }
}

impl std::fmt::Display for CR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}