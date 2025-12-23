use std::fmt::Display;

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

/// A player color.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Color {
    Pink,
    Orange,
    Green,
    Red,
    Yellow,
    Black,
    Purple,
    Blue,
}

impl Color {
    pub fn name(&self) -> &'static str {
        match self {
            Color::Pink => "pink",
            Color::Orange => "orange",
            Color::Green => "green",
            Color::Red => "red",
            Color::Yellow => "yellow",
            Color::Black => "black",
            Color::Purple => "purple",
            Color::Blue => "blue",
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::Pink => "Pink",
                Color::Orange => "Orange",
                Color::Green => "Green",
                Color::Red => "Red",
                Color::Yellow => "Yellow",
                Color::Black => "Black",
                Color::Purple => "Purple",
                Color::Blue => "Blue",
            }
        )
    }
}
