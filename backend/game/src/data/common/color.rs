use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use ts_rs::TS;

/// A player color.
#[derive(Debug, Clone, Serialize, Deserialize, EnumIter, TS)]
#[ts(export)]
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
