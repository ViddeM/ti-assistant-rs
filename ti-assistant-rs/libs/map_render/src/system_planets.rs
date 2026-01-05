use bevy::math::Vec2;
use ti_helper_game_data::components::planet::Planet;

/// Returns the visual offset of the planet on the system tile image.
pub fn planet_offset(planet: &Planet) -> Vec2 {
    match planet {
        Planet::Jord
        | Planet::MollPrimus
        | Planet::Darien
        | Planet::Muaat
        | Planet::Nestphar
        | Planet::ZeroZeroZero
        | Planet::Winnu
        | Planet::MordaiII
        | Planet::Elysium
        | Planet::Wellon
        | Planet::VefutII
        | Planet::Thibah
        | Planet::TarMann
        | Planet::Saudor
        | Planet::MeharXull
        | Planet::Creuss
        | Planet::Ixth
        | Planet::Arcturus
        | Planet::Acheron
        | Planet::TheDark
        | Planet::ArchonVail
        | Planet::Perimiter
        | Planet::SemLore
        | Planet::Ang
        | Planet::Vorhal
        | Planet::Primor
        | Planet::HopesEnd
        | Planet::MecatolRex => Vec2::new(0.0, 0.0),
        Planet::Maaluuk
        | Planet::ArcPrime
        | Planet::LisisII
        | Planet::Nar
        | Planet::TrenLak
        | Planet::ArchonRen
        | Planet::Quann
        | Planet::Lodor
        | Planet::NewAlbion
        | Planet::TequRan
        | Planet::Qucenn
        | Planet::Mellon
        | Planet::Lazar
        | Planet::DalBootha
        | Planet::Corneeq
        | Planet::Centauri
        | Planet::Bereg
        | Planet::Arnor
        | Planet::Arinam
        | Planet::Abyz
        | Planet::Naazir
        | Planet::Cormund
        | Planet::Atlas
        | Planet::Everra
        | Planet::Accoen
        | Planet::Kraag
        | Planet::Bakal
        | Planet::Lisis
        | Planet::Cealdri
        | Planet::VegaMajor
        | Planet::Retillion => Vec2::new(-0.1, 0.19),
        Planet::Druaa
        | Planet::WrenTerra
        | Planet::Ragh
        | Planet::Jol
        | Planet::Quinarra
        | Planet::ArchonTau
        | Planet::Starpoint
        | Planet::Torkan
        | Planet::Rarron
        | Planet::Zohbat
        | Planet::Sakulag
        | Planet::Xxehan
        | Planet::Resculon
        | Planet::Gral
        | Planet::LirtaIV
        | Planet::Lor
        | Planet::Meer
        | Planet::Fria
        | Planet::Rokha
        | Planet::JeolIr
        | Planet::Siig
        | Planet::AlioPrima
        | Planet::Velnor
        | Planet::Xanhact
        | Planet::VegaMinor
        | Planet::Shalloq => Vec2::new(0.09, -0.20),
        Planet::RigelII | Planet::Abaddon | Planet::Arretze | Planet::Ylir => Vec2::new(0.12, 0.21),
        Planet::RigelIII | Planet::Loki | Planet::Hercant | Planet::Valk => Vec2::new(-0.24, 0.05),
        Planet::RigelI | Planet::Ashtroth | Planet::Kamdorn | Planet::Avar => {
            Vec2::new(0.15, -0.26)
        }
        Planet::Mallice => Vec2::new(0.20, 0.12),
        Planet::Mirage => Vec2::new(0.12, -0.25),
        Planet::CustodiaVigilia => panic!(
            "Custodia vigilla should never be rendered as it is not considered to be on the map!"
        ),
        Planet::Lesab => todo!(),
        Planet::Olergodt => todo!(),
        Planet::ViraPicsIII => todo!(),
        Planet::Andeara => todo!(),
        Planet::Lemox => todo!(),
        Planet::TheWatchtower => todo!(),
        Planet::Emelpar => todo!(),
        Planet::Faunus => todo!(),
        Planet::Garbozia => todo!(),
        Planet::Tempesta => todo!(),
        Planet::Industrex => todo!(),
        Planet::Capha => todo!(),
        Planet::Kostboth => todo!(),
        Planet::Cresius => todo!(),
        Planet::LazulRex => todo!(),
        Planet::Hercalor => todo!(),
        Planet::Tiamat => todo!(),
        Planet::NewTerra => todo!(),
        Planet::Tinnes => todo!(),
        Planet::Bellatrix => todo!(),
        Planet::TsionStation => todo!(),
        Planet::Tarana => todo!(),
        Planet::OluzStation => todo!(),
        Planet::Cocytus => todo!(),
        Planet::Styx => todo!(),
        Planet::Lethe => todo!(),
        Planet::Phlegethon => todo!(),
        Planet::MecatolRexOmega => todo!(),
        Planet::ThundersEdge => todo!(),
        Planet::Ordinian => todo!(),
        Planet::Avernus => todo!(),
        Planet::Cronos => todo!(),
        Planet::CronosHollow => todo!(),
        Planet::Tallin => todo!(),
        Planet::TallinHollow => todo!(),
        Planet::Revelation => todo!(),
        Planet::MezLoOrzFeiZsha => todo!(),
        Planet::RepoLoOrzQet => todo!(),
        Planet::Ikatena => todo!(),
        Planet::AhkCreuxx => todo!(),
        Planet::Elnath => todo!(),
        Planet::Horizon => todo!(),
        Planet::LuthienVi => todo!(),
    }
}
