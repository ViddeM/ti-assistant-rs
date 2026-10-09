use ti_helper_game_data::common::faction::Faction;

use crate::error::{MiltyError, MiltyResult};

/// Tries to parse the name of a faction into a faction (note: Must match w/e naming scheme milty draft is using!)
pub fn parse_faction(name: &str) -> MiltyResult<Faction> {
    Ok(match name {
        "Sardakk N'orr" => Faction::SardakkNorr,
        "The Arborec" => Faction::Arborec,
        "The Barony of Letnev" => Faction::BaronyOfLetnev,
        "The Clan of Saar" => Faction::ClanOfSaar,
        "The Embers of Muaat" => Faction::EmbersOfMuaat,
        "The Emirates of Hacan" => Faction::EmiratesOfHacan,
        "The Federation of Sol" => Faction::FederationOfSol,
        "The Ghosts of Creuss" => Faction::GhostsOfCreuss,
        "The L1z1x Mindnet" => Faction::L1Z1XMindnet,
        "The Mentak Coalition" => Faction::MentakCoalition,
        "The Naalu Collective" => Faction::NaaluCollective,
        "The Nekro Virus" => Faction::NekroVirus,
        "The Universities of Jol-Nar" => Faction::UniversitiesOfJolNar,
        "The Winnu" => Faction::Winnu,
        "The Xxcha Kingdom" => Faction::XxchaKingdom,
        "The Yin Brotherhood" => Faction::YinBrotherhood,
        "The Yssaril Tribes" => Faction::YssarilTribes,
        "The Argent Flight" => Faction::ArgentFlight,
        "The Empyrean" => Faction::Empyrean,
        "The Mahact Gene-sorcerers" => Faction::MahactGeneSorcerers,
        "The Naaz-Rokha Alliance" => Faction::NaazRokhaAlliance,
        "The Nomad" => Faction::Nomad,
        "The Titans of Ul" => Faction::TitansOfUl,
        "The Vuil'raith Cabal" => Faction::VuilRaithCabal,
        "The Council Keleres" => Faction::CouncilKeleres,
        "Last Bastion" => Faction::LastBastion,
        "The Ral Nel Consortium" => Faction::RalNelConsortium,
        "The Deepwrought Scholarate" => Faction::DeepwroughtScholarate,
        "The Crimson Rebellion" => Faction::CrimsonRebellion,
        "The Firmament / The Obsidian" => Faction::FirmamentObsidian,
        "Augurs of Ilyxum"
        | "Celdauri Trade Confederation"
        | "Dih-Mohn Flotilla"
        | "Florzen Profiteers"
        | "Free Systems Compact"
        | "Ghemina Raiders"
        | "Glimmer of Mortheus"
        | "Kollecc Society"
        | "Kortali Tribunal"
        | "Li-Zho Dynasty"
        | "L'Tokk Khrask"
        | "Mirveda Protectorate"
        | "Myko-Mentori"
        | "Nivyn Star Kings"
        | "Olradin League"
        | "Roh'Dhna Mechatronics"
        | "Savages of Cymiae"
        | "Shipwrights of Axis"
        | "Tnelis Syndicate"
        | "Vaden Banking Clans"
        | "Vaylerian Scourge"
        | "Veldyr Sovereignty"
        | "Zealots of Rhodun"
        | "Zelian Purifier"
        | "Bentor Conglomerate"
        | "Berserkers of Kjalengard"
        | "Cheiran Hordes"
        | "Edyn Mandate"
        | "Ghoti Wayfarers"
        | "Gledge Union"
        | "Kyro Sodality"
        | "Lanefir Remnants"
        | "The Monks of Kolume"
        | "Nokar Sellships" => {
            return Err(MiltyError::DiscordantStarsNotSupported(format!(
                "Faction {name}"
            )));
        }
        other => return Err(MiltyError::UnknownFaction(other.to_string())),
    })
}
