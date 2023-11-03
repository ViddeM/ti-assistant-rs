use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Serialize, Deserialize, EnumIter)]
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
