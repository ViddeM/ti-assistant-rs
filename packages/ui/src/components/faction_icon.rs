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
