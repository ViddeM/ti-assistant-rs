use dioxus::prelude::*;
use ti_helper_game_data::common::faction::Faction;

#[derive(PartialEq, Debug, Clone, Props)]
pub struct FactionIconProps {
    faction: Faction,
    width: Option<u32>,
    height: Option<u32>,
    #[props(default, into)]
    class: String,
}

#[component]
pub fn FactionIcon(
    FactionIconProps {
        faction,
        width,
        height,
        class,
    }: FactionIconProps,
) -> Element {
    rsx! {
        img {
            src: get_faction_icon(&faction),
            alt: format!("Faction Icon ${faction}"),
            width: width.unwrap_or(32),
            height: height.unwrap_or(32),
            class,
        }
    }
}

const ARBOREC_ICON: Asset = asset!("/assets/icons/factions/Arborec.png");
const ARGENT_FLIGHT_ICON: Asset = asset!("/assets/icons/factions/ArgentFlight.png");
const BARONY_OF_LETNEV_ICON: Asset = asset!("/assets/icons/factions/BaronyOfLetnev.png");
const CLAN_OF_SAAR_ICON: Asset = asset!("/assets/icons/factions/ClanOfSaar.png");
const COUNCIL_OF_KELERES_ICON: Asset = asset!("/assets/icons/factions/CouncilKeleres.png");
const EMBERS_OF_MUAAT_ICON: Asset = asset!("/assets/icons/factions/EmbersOfMuaat.png");
const EMIRATES_OF_HACAN_ICON: Asset = asset!("/assets/icons/factions/EmiratesOfHacan.png");
const EMPYREAN_ICON: Asset = asset!("/assets/icons/factions/Empyrean.png");
const FEDERATION_OF_SOL_ICON: Asset = asset!("/assets/icons/factions/FederationOfSol.png");
const GHOSTS_OF_CREUSS_ICON: Asset = asset!("/assets/icons/factions/GhostsOfCreuss.png");
const L1Z1X_MINDNET_ICON: Asset = asset!("/assets/icons/factions/L1Z1XMindnet.png");
const MAHACT_GENE_SORCERERS_ICON: Asset = asset!("/assets/icons/factions/MahactGeneSorcerers.png");
const MENTAK_COALITION_ICON: Asset = asset!("/assets/icons/factions/MentakCoalition.png");
const NAALU_COLLECTIVE_ICON: Asset = asset!("/assets/icons/factions/NaaluCollective.png");
const NAAZ_ROKHA_ALLIANCE_ICON: Asset = asset!("/assets/icons/factions/NaazRokhaAlliance.png");
const NEKRO_VIRUS_ICON: Asset = asset!("/assets/icons/factions/NekroVirus.png");
const NOMAD_ICON: Asset = asset!("/assets/icons/factions/Nomad.png");
const SARDAKK_NORR_ICON: Asset = asset!("/assets/icons/factions/SardakkNorr.png");
const TITANS_OF_UL_ICON: Asset = asset!("/assets/icons/factions/TitansOfUl.png");
const UNIVERSITIES_OF_JOL_NAR_ICON: Asset =
    asset!("/assets/icons/factions/UniversitiesOfJolNar.png");
const VUIL_RAITH_CABAL_ICON: Asset = asset!("/assets/icons/factions/VuilRaithCabal.png");
const WINNU_ICON: Asset = asset!("/assets/icons/factions/Winnu.png");
const XXCHA_KINGDOM_ICON: Asset = asset!("/assets/icons/factions/XxchaKingdom.png");
const YIN_BROTHERHOOD_ICON: Asset = asset!("/assets/icons/factions/YinBrotherhood.png");
const YSSARIL_TRIBES_ICON: Asset = asset!("/assets/icons/factions/YssarilTribes.png");
const LAST_BASTION_ICON: Asset = asset!("/assets/icons/factions/LastBastion.png");
const RAL_NEL_CONSORTIUM_ICON: Asset = asset!("/assets/icons/factions/RalNelConsortium.png");
const CRIMSON_REBELLION_ICON: Asset = asset!("/assets/icons/factions/CrimsonRebellion.png");
const DEEPWROUGHT_SCHOLARATE_ICON: Asset =
    asset!("/assets/icons/factions/DeepwroughtScholarate.png");
const FIRMAMENT_ICON: Asset = asset!("/assets/icons/factions/Firmament.png");

pub fn get_faction_icon(faction: &Faction) -> Asset {
    match faction {
        Faction::Arborec => ARBOREC_ICON,
        Faction::BaronyOfLetnev => BARONY_OF_LETNEV_ICON,
        Faction::ClanOfSaar => CLAN_OF_SAAR_ICON,
        Faction::EmbersOfMuaat => EMBERS_OF_MUAAT_ICON,
        Faction::EmiratesOfHacan => EMIRATES_OF_HACAN_ICON,
        Faction::FederationOfSol => FEDERATION_OF_SOL_ICON,
        Faction::GhostsOfCreuss => GHOSTS_OF_CREUSS_ICON,
        Faction::L1Z1XMindnet => L1Z1X_MINDNET_ICON,
        Faction::MentakCoalition => MENTAK_COALITION_ICON,
        Faction::NaaluCollective => NAALU_COLLECTIVE_ICON,
        Faction::NekroVirus => NEKRO_VIRUS_ICON,
        Faction::SardakkNorr => SARDAKK_NORR_ICON,
        Faction::UniversitiesOfJolNar => UNIVERSITIES_OF_JOL_NAR_ICON,
        Faction::Winnu => WINNU_ICON,
        Faction::XxchaKingdom => XXCHA_KINGDOM_ICON,
        Faction::YinBrotherhood => YIN_BROTHERHOOD_ICON,
        Faction::YssarilTribes => YSSARIL_TRIBES_ICON,
        Faction::ArgentFlight => ARGENT_FLIGHT_ICON,
        Faction::Empyrean => EMPYREAN_ICON,
        Faction::MahactGeneSorcerers => MAHACT_GENE_SORCERERS_ICON,
        Faction::NaazRokhaAlliance => NAAZ_ROKHA_ALLIANCE_ICON,
        Faction::Nomad => NOMAD_ICON,
        Faction::TitansOfUl => TITANS_OF_UL_ICON,
        Faction::VuilRaithCabal => VUIL_RAITH_CABAL_ICON,
        Faction::CouncilKeleres => COUNCIL_OF_KELERES_ICON,
        Faction::LastBastion => LAST_BASTION_ICON,
        Faction::RalNelConsortium => RAL_NEL_CONSORTIUM_ICON,
        Faction::DeepwroughtScholarate => DEEPWROUGHT_SCHOLARATE_ICON,
        Faction::CrimsonRebellion => CRIMSON_REBELLION_ICON,
        Faction::FirmamentObsidian => FIRMAMENT_ICON,
    }
}
