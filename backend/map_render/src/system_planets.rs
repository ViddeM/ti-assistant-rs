use bevy::math::Vec2;
use ti_helper_game_data::components::{planet::Planet, system::SystemId};

pub struct PlanetVisuals {
    planet: Planet,
    offset: Vec2,
}

macro_rules! pv {
    ($planet: ident) => {
        PlanetVisuals {
            planet: Planet::$planet,
            offset: Vec2::ZERO,
        }
    };
    ($planet: ident, $offset_x: literal, $offset_y: literal) => {
        PlanetVisuals {
            planet: Planet::$planet,
            offset: Vec2::new($offset_x, $offset_y),
        }
    };
}

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
        | Planet::MecatolRex => Vec2::new(0.0, 0.08),
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
        | Planet::Retillion => Vec2::new(-0.12, 0.25),
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
        | Planet::Shalloq => Vec2::new(0.15, -0.20),
        // Top-right planet in trinary systems
        Planet::RigelII | Planet::Abaddon | Planet::Arretze | Planet::Ylir => Vec2::new(0.14, 0.25),
        // Left planet in trinay systems
        Planet::RigelIII | Planet::Loki | Planet::Hercant | Planet::Valk => Vec2::new(-0.18, 0.0),
        // Bottom-right planet in trinary systems
        Planet::RigelI | Planet::Ashtroth | Planet::Kamdorn | Planet::Avar => {
            Vec2::new(0.15, -0.20)
        }
        Planet::Mallice => Vec2::new(0.20, 0.12),
        Planet::Mirage => Vec2::ZERO,
        Planet::CustodiaVigilia => panic!(
            "Custodia vigilla should never be rendered as it is not considered to be on the map!"
        ),
    }
}

pub fn system_planets(system_id: &SystemId) -> eyre::Result<Vec<PlanetVisuals>> {
    let system_number = match system_id.parse::<u32>() {
        Ok(id) => id,
        Err(_) => {
            // All non-numeric system ids do not contain planets (they are all wormholes).
            return Ok(vec![]);
        }
    };

    let visuals = match system_number {
        1 => vec![pv!(Jord)],
        2 => vec![pv!(MollPrimus)],
        3 => vec![pv!(Darien)],
        4 => vec![pv!(Muaat)],
        5 => vec![pv!(Nestphar)],
        6 => vec![pv!(ZeroZeroZero)],
        7 => vec![pv!(Winnu)],
        8 => vec![pv!(MordaiII)],
        9 => vec![pv!(Maaluuk, -0.25, 0.25), pv!(Druaa, 0.25, -0.25)],
        10 => vec![pv!(ArcPrime, -0.25, 0.25), pv!(WrenTerra, 0.25, -0.25)],
        11 => vec![pv!(LisisII, -0.25, 0.25), pv!(Ragh, 0.25, -0.25)],
        12 => vec![pv!(Nar, -0.25, 0.25), pv!(Jol, 0.25, -0.25)],
        13 => vec![pv!(TrenLak, -0.25, 0.25), pv!(Quinarra, 0.25, -0.25)],

        // Remaining systems should not include any planets
        _ => vec![],
    };

    // let offset = match system_number {
    //     // Single planet in the center
    //     0..=8 | 18..=24 | 51..=56 | 59..=63 | 65..=66 => vec![Vec2::ZERO],
    //     // Two planets, one top-left and one bottom-right
    //     9..=15 | 27..=38 | 57 | 69..=74 => vec![Vec2::new(-0.25, 0.25), Vec2::new(0.25, -0.25)],
    //     // Three planet, center-left, top-right, bottom-right
    //     16 | 58 | 75..=76 => vec![
    //         Vec2::new(-0.4, 0.0),
    //         Vec2::new(0.2, 0.25),
    //         Vec2::new(0.2, -0.25),
    //     ],
    //     // Single planet top-left (usually with wormhole to the bottom-right)
    //     25..=26 | 64 | 67..=68 => vec![Vec2::new(-0.25, 0.25)],
    //     // Mallice (slightly top-right)
    //     82 => vec![Vec2::new(0.2, 0.2)],
    //     // The rest are empty or have anomalies etc.
    //     _ => vec![],
    // };

    Ok(visuals)
}
