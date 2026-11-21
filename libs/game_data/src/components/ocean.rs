use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// An ocean card of [crate::common::faction::Faction::DeepwroughtScholarate]. It is equivalent to a
/// planet card with 1 resource and 1 influence, but it isn't a planet.
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter,
)]
pub enum Ocean {
    /// Deep Abyss.
    DeepAbyss,
    /// Brine Pool.
    BrinePool,
    /// Coral Reef.
    CoralReef,
    /// Ice Shelf.
    IceShelf,
    /// Lost Fleet.
    LostFleet,
}

impl Ocean {
    /// The name of the ocean card.
    pub fn name(&self) -> &'static str {
        match self {
            Ocean::DeepAbyss => "Deep Abyss",
            Ocean::BrinePool => "Brine Pool",
            Ocean::CoralReef => "Coral Reef",
            Ocean::IceShelf => "Ice Shelf",
            Ocean::LostFleet => "Lost Fleet",
        }
    }
}
