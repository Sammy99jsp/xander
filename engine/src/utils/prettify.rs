//!
//! Formatting choices!
//!

use owo_colors::{colors, Stream::Stdout};

macro_rules! formatting {
    ($ident: ident, $expr: expr) => {
        pub fn $ident<D: owo_colors::OwoColorize + std::fmt::Display>(
            item: &D,
        ) -> impl std::fmt::Display + '_ {
            item.if_supports_color(Stdout, $expr)
        }
    };
}

formatting!(die, |s| s.fg::<colors::BrightBlue>());
formatting!(modifier, |s| s.fg::<colors::css::Gold>());

formatting!(strikethrough, |s| s.strikethrough());
