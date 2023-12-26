use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

/// A player color.
#[derive(Debug, Clone, Serialize, Deserialize, EnumIter)]
#[allow(missing_docs)]
pub enum Color {
    Blue,
    Green,
    Red,
    Yellow,
    Black,
    Purple,
    Orange,
    Pink,
}
