use ti_helper_game_data::common::{expansions::Expansion, faction::Faction, map::HexMapError};

/// Result for milty operations.
pub type MiltyResult<T> = Result<T, MiltyError>;

/// Errors that can occurr when importing from milty draft.
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum MiltyError {
    #[error("Failed to parse faction for player {player}, unknown faction {faction} (err: {error}")]
    FactionParseError {
        player: String,
        faction: String,
        error: String,
    },
    #[error("Failed to parse player order into a u32, order: {position} (err: {error})")]
    OrderParseError { position: String, error: String },
    #[error("Failed to send request to milty, err: {0}")]
    SendRequestError(#[from] reqwest::Error),
    #[error("Failed to parse milty response, err: {0}")]
    ParseResponseError(reqwest::Error),
    #[error("Milty draft not finished")]
    DraftNotComplete,
    #[error("Discordant stars addon not supported but was enabled in milty, found {0}")]
    DiscordantStarsNotSupported(String),
    #[error(
        "Milty contained faction ({faction:?}) from expansion ({expansion:?}) that was not enabled"
    )]
    FactionsExpansionNotEnabled {
        faction: Faction,
        expansion: Expansion,
    },
    #[error("Multiple players had the name which is not supported")]
    DuplicatePlayerNames,
    #[error("Multiple players had the same faction which is not supported")]
    DuplicatePlayerFactions,
    #[error("Failed to parse hex map, err: {0}")]
    HexMapError(#[from] HexMapError),
    #[error("Unknown faction: {0}")]
    UnknownFaction(String),
}
