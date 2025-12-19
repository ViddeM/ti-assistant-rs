use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use ts_rs::TS;

use crate::{
    common::{color::Color, faction::Faction, game_settings::Expansions},
    components::{
        action_card::{ActionCard, ActionCardInfo},
        agenda::{Agenda, AgendaInfo},
        frontier_card::{FrontierCard, FrontierCardInfo},
        leaders::{Leader, LeaderInfo},
        objectives::{Objective, ObjectiveInfo, public::PublicObjective, secret::SecretObjective},
        planet::{Planet, PlanetInfo},
        planet_attachment::{PlanetAttachment, PlanetAttachmentInfo},
        relic::{Relic, RelicInfo},
        system::{System, SystemId, systems},
        tech::{TechInfo, Technology},
    },
    enum_map::EnumMap,
};

const MIN_PLAYER_COUNT: usize = 3;

/// All information that is static for a game of TI4.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GameOptions {
    /// The minimum number of players allowed for the game.
    min_players: usize,
    /// The maximum number of players allowed for the game.
    max_players: usize,
    /// What colors players are allowed to have.
    colors: Vec<Color>,
    /// What factions exist within the game.
    factions: Vec<FactionResponse>,
    /// What systems exists in the game.
    systems: EnumMap<SystemId, System>,
    /// What technologies exist in the game.
    technologies: EnumMap<Technology, TechInfo>,
    /// What planets exist in the game.
    planet_infos: EnumMap<Planet, PlanetInfo>,
    /// What planet attachments exist in the game.
    planet_attachments: EnumMap<PlanetAttachment, PlanetAttachmentInfo>,
    /// What objectives exist in the game.
    objectives: EnumMap<Objective, ObjectiveInfo>,
    /// What action cards exist in the game.
    action_cards: EnumMap<ActionCard, ActionCardInfo>,
    /// What agendas exist in the game.
    agendas: EnumMap<Agenda, AgendaInfo>,
    /// What leaders exist in the game.
    leaders: EnumMap<Leader, LeaderInfo>,
    /// Map from all factions in the game to the leaders of that faction.
    leaders_by_faction: EnumMap<Faction, Vec<Leader>>,
    /// What frontier cards exists in the game.
    frontier_cards: EnumMap<FrontierCard, FrontierCardInfo>,
    /// What relics exists in the game.
    relics: EnumMap<Relic, RelicInfo>,
}

impl std::ops::Deref for GameOptions {
    type Target = EnumMap<Faction, Vec<Leader>>;

    fn deref(&self) -> &Self::Target {
        &self.leaders_by_faction
    }
}

impl GameOptions {
    /// Returns GameOptions for the specified expansions.
    pub fn new(expansions: &Expansions) -> Self {
        let leaders: EnumMap<_, _> = Leader::iter()
            .filter(|leader| leader.is_enabled_in(expansions))
            .map(|leader| (leader, leader.info()))
            .collect();

        Self {
            min_players: MIN_PLAYER_COUNT,
            max_players: expansions.max_number_of_players(),
            factions: Faction::iter()
                .filter(|f| expansions.is_enabled(&f.expansion()))
                .map(|faction| FactionResponse {
                    faction,
                    name: faction.name(),
                })
                .collect::<Vec<FactionResponse>>(),
            colors: Color::iter().collect(),
            systems: systems()
                .into_iter()
                .filter(|(_, s)| expansions.is_enabled(&s.expansion))
                .collect(),
            planet_infos: Planet::iter()
                .map(|p| (p.clone(), p.info()))
                .filter(|(_, info)| expansions.is_enabled(&info.expansion))
                .collect(),
            planet_attachments: PlanetAttachment::iter()
                .map(|a| (a.clone(), a.info()))
                .filter(|(_, info)| expansions.is_enabled(&info.expansion))
                .collect(),
            objectives: PublicObjective::iter()
                .map(Objective::from)
                .chain(SecretObjective::iter().map(Objective::from))
                .map(|o| {
                    let info = o.info();
                    (o, info)
                })
                .filter(|(_, o)| expansions.is_enabled(&o.expansion))
                .collect(),
            technologies: Technology::iter()
                .filter(|tech| tech.is_enabled_in(expansions))
                .map(|t| (t.clone(), t.info()))
                .collect(),
            action_cards: ActionCard::iter()
                .map(|card| (card.clone(), card.info()))
                .filter(|(_, card)| expansions.is_enabled(&card.expansion))
                .collect(),
            agendas: Agenda::iter()
                .map(|agenda| (agenda, agenda.info()))
                .filter(|(_, agenda)| expansions.is_enabled(&agenda.expansion))
                .filter(|(agenda, _)| !(expansions.prophecy_of_kings && agenda.disabled_in_pok()))
                .collect(),
            leaders_by_faction: leaders
                .iter()
                .map(|(leader, info)| (info.faction(), leader))
                .fold(EnumMap::new(), |mut acc, (faction, leader)| {
                    acc.entry(faction).or_default().push(*leader);
                    acc
                }),
            leaders,
            frontier_cards: FrontierCard::iter()
                .map(|f| (f.clone(), f.info()))
                .filter(|(_, card)| expansions.is_enabled(&card.expansion))
                .collect(),
            relics: Relic::iter()
                .map(|relic| (relic.clone(), relic.info()))
                .filter(|(_, relic)| expansions.is_enabled(&relic.expansion))
                .collect(),
        }
    }
}

/// A faction in the game.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct FactionResponse {
    /// The faction ID.
    faction: Faction,
    /// The name of the faction in 'pretty' format.
    name: String,
}
