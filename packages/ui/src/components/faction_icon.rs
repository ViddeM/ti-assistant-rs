use std::sync::Arc;

use dioxus::prelude::*;
use ti_helper_game_data::{
    common::faction::Faction,
    state::faction_state::{FactionState, FirmamentSide, FirmamentState},
};

#[derive(PartialEq, Debug, Clone, Props)]
pub struct FactionIconProps {
    faction: Faction,
    faction_state: Arc<FactionState>,
    width: Option<u32>,
    height: Option<u32>,
    #[props(default, into)]
    class: String,
}

#[component]
pub fn FactionIcon(
    FactionIconProps {
        faction,
        faction_state,
        width,
        height,
        class,
    }: FactionIconProps,
) -> Element {
    rsx! {
        img {
            src: get_faction_icon(&faction, &faction_state),
            alt: format!("Faction Icon ${faction}"),
            width: width.unwrap_or(32),
            height: height.unwrap_or(32),
            class,
        }
    }
}

const ARBOREC_ICON: Asset = asset!("/assets/icons/factions/webp/Arborec.webp");
const ARGENT_FLIGHT_ICON: Asset = asset!("/assets/icons/factions/webp/ArgentFlight.webp");
const BARONY_OF_LETNEV_ICON: Asset = asset!("/assets/icons/factions/webp/BaronyOfLetnev.webp");
const CLAN_OF_SAAR_ICON: Asset = asset!("/assets/icons/factions/webp/ClanOfSaar.webp");
const COUNCIL_OF_KELERES_ICON: Asset = asset!("/assets/icons/factions/webp/CouncilKeleres.webp");
const EMBERS_OF_MUAAT_ICON: Asset = asset!("/assets/icons/factions/webp/EmbersOfMuaat.webp");
const EMIRATES_OF_HACAN_ICON: Asset = asset!("/assets/icons/factions/webp/EmiratesOfHacan.webp");
const EMPYREAN_ICON: Asset = asset!("/assets/icons/factions/webp/Empyrean.webp");
const FEDERATION_OF_SOL_ICON: Asset = asset!("/assets/icons/factions/webp/FederationOfSol.webp");
const GHOSTS_OF_CREUSS_ICON: Asset = asset!("/assets/icons/factions/webp/GhostsOfCreuss.webp");
const L1Z1X_MINDNET_ICON: Asset = asset!("/assets/icons/factions/webp/L1Z1XMindnet.webp");
const MAHACT_GENE_SORCERERS_ICON: Asset =
    asset!("/assets/icons/factions/webp/MahactGeneSorcerers.webp");
const MENTAK_COALITION_ICON: Asset = asset!("/assets/icons/factions/webp/MentakCoalition.webp");
const NAALU_COLLECTIVE_ICON: Asset = asset!("/assets/icons/factions/webp/NaaluCollective.webp");
const NAAZ_ROKHA_ALLIANCE_ICON: Asset =
    asset!("/assets/icons/factions/webp/NaazRokhaAlliance.webp");
const NEKRO_VIRUS_ICON: Asset = asset!("/assets/icons/factions/webp/NekroVirus.webp");
const NOMAD_ICON: Asset = asset!("/assets/icons/factions/webp/Nomad.webp");
const SARDAKK_NORR_ICON: Asset = asset!("/assets/icons/factions/webp/SardakkNorr.webp");
const TITANS_OF_UL_ICON: Asset = asset!("/assets/icons/factions/webp/TitansOfUl.webp");
const UNIVERSITIES_OF_JOL_NAR_ICON: Asset =
    asset!("/assets/icons/factions/webp/UniversitiesOfJolNar.webp");
const VUIL_RAITH_CABAL_ICON: Asset = asset!("/assets/icons/factions/webp/VuilRaithCabal.webp");
const WINNU_ICON: Asset = asset!("/assets/icons/factions/webp/Winnu.webp");
const XXCHA_KINGDOM_ICON: Asset = asset!("/assets/icons/factions/webp/XxchaKingdom.webp");
const YIN_BROTHERHOOD_ICON: Asset = asset!("/assets/icons/factions/webp/YinBrotherhood.webp");
const YSSARIL_TRIBES_ICON: Asset = asset!("/assets/icons/factions/webp/YssarilTribes.webp");
const LAST_BASTION_ICON: Asset = asset!("/assets/icons/factions/webp/LastBastion.webp");
const RAL_NEL_CONSORTIUM_ICON: Asset = asset!("/assets/icons/factions/webp/RalNelConsortium.webp");
const CRIMSON_REBELLION_ICON: Asset = asset!("/assets/icons/factions/webp/CrimsonRebellion.webp");
const DEEPWROUGHT_SCHOLARATE_ICON: Asset =
    asset!("/assets/icons/factions/webp/DeepwroughtScholarate.webp");
const FIRMAMENT_ICON: Asset = asset!("/assets/icons/factions/webp/Firmament.webp");
const OBSIDIAN_ICON: Asset = asset!("/assets/icons/factions/webp/Obsidian.webp");

pub fn get_faction_icon(faction: &Faction, state: &FactionState) -> Asset {
    match (faction, state) {
        (Faction::Arborec, FactionState::None) => ARBOREC_ICON,
        (Faction::BaronyOfLetnev, FactionState::None) => BARONY_OF_LETNEV_ICON,
        (Faction::ClanOfSaar, FactionState::None) => CLAN_OF_SAAR_ICON,
        (Faction::EmbersOfMuaat, FactionState::None) => EMBERS_OF_MUAAT_ICON,
        (Faction::EmiratesOfHacan, FactionState::None) => EMIRATES_OF_HACAN_ICON,
        (Faction::FederationOfSol, FactionState::None) => FEDERATION_OF_SOL_ICON,
        (Faction::GhostsOfCreuss, FactionState::None) => GHOSTS_OF_CREUSS_ICON,
        (Faction::L1Z1XMindnet, FactionState::None) => L1Z1X_MINDNET_ICON,
        (Faction::MentakCoalition, FactionState::None) => MENTAK_COALITION_ICON,
        (Faction::NaaluCollective, FactionState::None) => NAALU_COLLECTIVE_ICON,
        (Faction::NekroVirus, FactionState::None) => NEKRO_VIRUS_ICON,
        (Faction::SardakkNorr, FactionState::None) => SARDAKK_NORR_ICON,
        (Faction::UniversitiesOfJolNar, FactionState::None) => UNIVERSITIES_OF_JOL_NAR_ICON,
        (Faction::Winnu, FactionState::None) => WINNU_ICON,
        (Faction::XxchaKingdom, FactionState::None) => XXCHA_KINGDOM_ICON,
        (Faction::YinBrotherhood, FactionState::None) => YIN_BROTHERHOOD_ICON,
        (Faction::YssarilTribes, FactionState::None) => YSSARIL_TRIBES_ICON,
        (Faction::ArgentFlight, FactionState::None) => ARGENT_FLIGHT_ICON,
        (Faction::Empyrean, FactionState::None) => EMPYREAN_ICON,
        (Faction::MahactGeneSorcerers, FactionState::None) => MAHACT_GENE_SORCERERS_ICON,
        (Faction::NaazRokhaAlliance, FactionState::None) => NAAZ_ROKHA_ALLIANCE_ICON,
        (Faction::Nomad, FactionState::None) => NOMAD_ICON,
        (Faction::TitansOfUl, FactionState::None) => TITANS_OF_UL_ICON,
        (Faction::VuilRaithCabal, FactionState::None) => VUIL_RAITH_CABAL_ICON,
        (Faction::CouncilKeleres, FactionState::None) => COUNCIL_OF_KELERES_ICON,
        (Faction::LastBastion, FactionState::None) => LAST_BASTION_ICON,
        (Faction::RalNelConsortium, FactionState::None) => RAL_NEL_CONSORTIUM_ICON,
        (Faction::DeepwroughtScholarate, FactionState::None) => DEEPWROUGHT_SCHOLARATE_ICON,
        (Faction::CrimsonRebellion, FactionState::None) => CRIMSON_REBELLION_ICON,
        (Faction::FirmamentObsidian, FactionState::Firmament(FirmamentState { side })) => {
            match side {
                FirmamentSide::Firmament => FIRMAMENT_ICON,
                FirmamentSide::Obsidian => OBSIDIAN_ICON,
            }
        }
        _ => panic!("Invalid faction/faction state combination {faction:?} {state:?}"),
    }
}
