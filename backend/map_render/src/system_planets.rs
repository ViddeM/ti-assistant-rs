use bevy::math::Vec2;
use ti_helper_game::data::components::{planet::Planet, system::SystemId};

use crate::console_error;

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
        Planet::Jord
        | Planet::MollPrimus
        | Planet::Darien
        | Planet::Muaat
        | Planet::Nestphar
        | Planet::ZeroZeroZero
        | Planet::Winnu
        | Planet::MordaiII
        | Planet::Elysium => Vec2::new(0.0, 0.08),
        Planet::Maaluuk
        | Planet::ArcPrime
        | Planet::LisisII
        | Planet::Nar
        | Planet::TrenLak
        | Planet::Retillion => Vec2::new(-0.12, 0.25),
        Planet::Druaa
        | Planet::WrenTerra
        | Planet::Ragh
        | Planet::Jol
        | Planet::Quinarra
        | Planet::Shalloq => Vec2::new(0.15, -0.20),
        // TODO
        _ => {
            console_error(&format!("ERROR: NOT IMPLEMENTED FOR PLANET: {planet:?}"));
            Vec2::new(25.0, 25.0)
        }
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
