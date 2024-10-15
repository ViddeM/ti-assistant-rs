use bevy::math::Vec2;
use ti_helper_game_data::components::planet::Planet;

/// Returns the visual offset of the planet on the system tile image.
pub fn planet_offset(planet: &Planet) -> Vec2 {
    match planet {
        // Single planet systems
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
        // Upper planet in two-planet systems
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
        // Lower planet in two-planet systems
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
        // Top-right planet in trinary systems
        Planet::RigelII | Planet::Abaddon | Planet::Arretze | Planet::Ylir => Vec2::new(0.12, 0.21),
        // Left planet in trinay systems
        Planet::RigelIII | Planet::Loki | Planet::Hercant | Planet::Valk => Vec2::new(-0.24, 0.05),
        // Bottom-right planet in trinary systems
        Planet::RigelI | Planet::Ashtroth | Planet::Kamdorn | Planet::Avar => {
            Vec2::new(0.15, -0.26)
        }
        Planet::Mallice => Vec2::new(0.20, 0.12),
        Planet::Mirage => Vec2::new(0.12, -0.25),
        Planet::CustodiaVigilia => panic!(
            "Custodia vigilla should never be rendered as it is not considered to be on the map!"
        ),
    }
}
