#![allow(unused)]
use std::{fmt::Display, ops::BitOr};

// Not all terminals support these codes!
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const ITALIC: &str = "\x1b[3m";
const UNDERLINE: &str = "\x1b[4m";
const STRIKETHROUGH: &str = "\x1b[9m";
const HIDDEN: &str = "\x1b[8m";

const ALL_STYLES: [&str; 6] = [RESET, BOLD, ITALIC, UNDERLINE, STRIKETHROUGH, HIDDEN];
// Less clearer but less code
//const ALL_STYLES: [&str; 6] = ["\x1b[0m", "\x1b[1m", "\x1b[3m", "\x1b[4m", "\x1b[9m", "\x1b[8m"];

#[derive(Clone, Copy)]
pub struct Styles(u8);

impl Styles {
    pub const RESET: Self = Self(1);
    pub const BOLD: Self = Self(1 << 1);
    pub const ITALIC: Self = Self(1 << 2);
    pub const UNDERLINE: Self = Self(1 << 3);
    pub const STRIKETHROUGH: Self = Self(1 << 4);
    pub const HIDDEN: Self = Self(1 << 5);

    fn styles_as_string(&self) -> String {
        let mut combined_styles: String = String::new();
        for (index, style) in ALL_STYLES.iter().enumerate() {
            let bit = (self.0 >> index) & 1;
            if (bit == 1) {
                combined_styles.push_str(style);
            }
        }
        return combined_styles;
    }

    pub fn style(self) -> String {
        self.styles_as_string()
    }
}

impl Display for Styles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.styles_as_string())
    }
}

impl BitOr for Styles {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[macro_export]
macro_rules! styled_print {
    ($text:expr, $style:expr) => {
        println!("{}{}{}", Styles::style($style), $text, Styles::RESET)
    };
}
