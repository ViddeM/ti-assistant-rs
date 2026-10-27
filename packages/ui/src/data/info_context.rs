use dioxus::prelude::*;
use ti_helper_game_data::{
    common::game_settings::Expansions,
    components::{
        agenda::Agenda,
        leaders::Leader,
        objectives::Objective,
        relic::Relic,
        strategy_card::{StrategyCard, StrategyCardVersion},
        tech::Technology,
    },
};

#[derive(Clone, Copy)]
pub struct InfoContext {
    info: Signal<Option<Info>>,
}

impl InfoContext {
    pub fn new(data: Signal<Option<Info>>) -> Self {
        Self { info: data }
    }

    pub fn get(&self) -> ReadSignal<Option<Info>> {
        self.info.into()
    }

    pub fn open(&mut self, info: Info) {
        self.info.set(Some(info));
    }

    pub fn close(&mut self) {
        self.info.set(None);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Info {
    Agenda(Agenda),
    Leader(Leader),
    Objective(Objective),
    Strategy(StrategyCard),
    Tech(Technology),
    Relic(Relic),
    Custom {
        title: String,
        subtitle: String,
        description: InfoDescription,
    },
}

impl Info {
    pub fn title(&self) -> String {
        match self {
            Info::Agenda(agenda) => agenda.info().name,
            Info::Leader(leader) => leader.info().name(),
            Info::Objective(objective) => objective.info().name,
            Info::Strategy(strategy_card) => strategy_card.to_string(),
            Info::Tech(technology) => technology.info().name,
            Info::Relic(relic) => relic.info().name,
            Info::Custom {
                title,
                subtitle: _,
                description: _,
            } => title.clone(),
        }
    }

    pub fn subtitle(&self) -> String {
        match self {
            Info::Agenda(agenda) => agenda.info().kind.to_string(),
            Info::Leader(leader) => leader.type_name().to_string(),
            Info::Objective(objective) => objective.info().kind.to_string(),
            Info::Strategy(..) => String::new(),
            Info::Tech(technology) => technology.info().tech_type.to_string(),
            Info::Relic(..) => String::new(),
            Info::Custom {
                title: _,
                subtitle,
                description: _,
            } => subtitle.clone(),
        }
    }

    pub fn description(&self, expansions: &Expansions) -> InfoDescription {
        match self {
            Info::Agenda(agenda) => InfoDescription::Description(agenda.info().description),
            Info::Leader(leader) => InfoDescription::Description(match leader {
                Leader::Agent(agent) => agent.info().description,
                Leader::Commander(commander) => commander.info().description,
                Leader::Hero(hero) => hero.info().description,
            }),
            Info::Objective(objective) => InfoDescription::Description(objective.info().condition),
            Info::Strategy(strategy_card) => InfoDescription::Custom(rsx! {
                img {
                    alt: format!("Strategy card {strategy_card}"),
                    src: get_strategy_card_image(strategy_card.version(expansions)),
                }
            }),
            Info::Tech(technology) => {
                InfoDescription::Description(technology.info().effects.join("\n"))
            }
            Info::Relic(relic) => InfoDescription::Description(relic.info().description),
            Info::Custom {
                title: _,
                subtitle: _,
                description,
            } => description.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InfoDescription {
    Description(String),
    Custom(Element),
}

impl InfoDescription {
    pub fn to_element(self) -> Element {
        match self {
            InfoDescription::Description(d) => rsx! {
                p { {d} }
            },
            InfoDescription::Custom(vnode) => vnode,
        }
    }
}

const CONSTRUCTION_OMEGA_IMAGE: Asset =
    asset!("/assets/images/strat_cards/construction_omega.webp");
const CONSTRUCTION_IMAGE: Asset = asset!("/assets/images/strat_cards/construction.webp");
const DIPLOMACY_OMEGA_IMAGE: Asset = asset!("/assets/images/strat_cards/diplomacy_omega.webp");
const DIPLOMACY_IMAGE: Asset = asset!("/assets/images/strat_cards/diplomacy.webp");
const IMPERIAL_IMAGE: Asset = asset!("/assets/images/strat_cards/imperial.webp");
const LEADERSHIP_IMAGE: Asset = asset!("/assets/images/strat_cards/leadership.webp");
const POLITICS_IMAGE: Asset = asset!("/assets/images/strat_cards/politics.webp");
const TECHNOLOGY_IMAGE: Asset = asset!("/assets/images/strat_cards/technology.webp");
const TRADE_IMAGE: Asset = asset!("/assets/images/strat_cards/trade.webp");
const WARFARE_IMAGE: Asset = asset!("/assets/images/strat_cards/warfare.webp");

fn get_strategy_card_image(version: StrategyCardVersion) -> Asset {
    match version {
        StrategyCardVersion::Leadership => LEADERSHIP_IMAGE,
        StrategyCardVersion::Diplomacy => DIPLOMACY_IMAGE,
        StrategyCardVersion::DiplomacyOmega => DIPLOMACY_OMEGA_IMAGE,
        StrategyCardVersion::Politics => POLITICS_IMAGE,
        StrategyCardVersion::Construction => CONSTRUCTION_IMAGE,
        StrategyCardVersion::ConstructionOmega => CONSTRUCTION_OMEGA_IMAGE,
        StrategyCardVersion::ConstructionOmegaOmega => CONSTRUCTION_OMEGA_IMAGE,
        StrategyCardVersion::Trade => TRADE_IMAGE,
        StrategyCardVersion::Warfare => WARFARE_IMAGE,
        StrategyCardVersion::WarfareOmega => WARFARE_IMAGE,
        StrategyCardVersion::Technology => TECHNOLOGY_IMAGE,
        StrategyCardVersion::Imperial => IMPERIAL_IMAGE,
    }
}
