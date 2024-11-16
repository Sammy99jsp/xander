//!
//! Formatting choices!
//!

use owo_colors::{colors, OwoColorize, Stream::Stdout, SupportsColorsDisplay};
use std::fmt::Display;

macro_rules! formatting {
    ($ident: ident, $expr: expr) => {
        pub fn $ident<'a, D: owo_colors::OwoColorize + std::fmt::Display>(
            item: &'a D,
        ) -> impl std::fmt::Display + 'a {
            item.if_supports_color(Stdout, $expr)
        }
    };
}

formatting!(die, |s| s.fg::<colors::BrightBlue>());
formatting!(modifier, |s| s.fg::<colors::css::Gold>());

formatting!(strikethrough, |s| s.strikethrough());
