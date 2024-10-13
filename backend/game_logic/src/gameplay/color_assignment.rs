use eyre::OptionExt;
use std::collections::{BinaryHeap, HashMap};
use strum::IntoEnumIterator;
use ti_helper_game_data::common::{color::Color, faction::Faction}; // 0.7.2

#[derive(Debug, Clone)]
struct ColorPrio {
    color: Color,
    weight: f32,
}

impl ColorPrio {
    fn base(color: Color) -> Self {
        Self { color, weight: 0.0 }
    }
}

impl PartialEq for ColorPrio {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Eq for ColorPrio {}

impl PartialOrd for ColorPrio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ColorPrio {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.total_cmp(&other.weight)
    }
}

macro_rules! w {
    ($color:ident, $weight:literal) => {
        ColorPrio {
            color: Color::$color,
            weight: $weight,
        }
    };
}

macro_rules! prio {
   ($($prio:expr),+) => {
    {
        let mut heap = BinaryHeap::new();
        $(
            heap.push($prio);
        )+
        heap
    }
    };

}

/// Prios stolen from https://twilight-imperium.fandom.com/wiki/Faction_Colors#Method_of_determining_play_piece_color_for_each_faction
fn get_faction_color_prio(faction: &Faction) -> BinaryHeap<ColorPrio> {
    match faction {
        Faction::Arborec => prio!(
            w!(Green, 1.6),
            w!(Black, 0.1),
            w!(Yellow, 0.1),
            w!(Blue, 0.1)
        ),
        Faction::BaronyOfLetnev => prio![w!(Red, 0.95), w!(Black, 0.8), w!(Blue, 0.1)],
        Faction::ClanOfSaar => prio![w!(Orange, 0.85), w!(Green, 0.5), w!(Yellow, 0.4)],
        Faction::EmbersOfMuaat => prio![w!(Red, 1.25), w!(Orange, 0.65)],
        Faction::EmiratesOfHacan => prio![w!(Yellow, 1.2), w!(Orange, 0.7)],
        Faction::FederationOfSol => prio![w!(Blue, 1.25), w!(Yellow, 0.75)],
        Faction::GhostsOfCreuss => prio![w!(Blue, 1.7), w!(Black, 0.1), w!(Purple, 0.1)],
        Faction::L1Z1XMindnet => prio!(w!(Black, 0.7), w!(Blue, 0.6), w!(Red, 0.6)),
        Faction::MentakCoalition => prio!(w!(Orange, 0.95), w!(Black, 0.5), w!(Yellow, 0.45)),
        Faction::NaaluCollective => prio!(w!(Green, 1.15), w!(Yellow, 0.45), w!(Orange, 0.3)),
        Faction::NekroVirus => prio!(w!(Red, 1.75), w!(Black, 0.15)),
        Faction::SardakkNorr => prio!(w!(Black, 1.0), w!(Red, 0.9)),
        Faction::UniversitiesOfJolNar => prio!(w!(Blue, 1.6), w!(Purple, 0.3)),
        Faction::Winnu => prio!(w!(Orange, 0.75), w!(Purple, 0.6), w!(Yellow, 0.55)),
        Faction::XxchaKingdom => prio!(w!(Green, 1.1), w!(Blue, 0.8)),
        Faction::YinBrotherhood => prio!(w!(Purple, 1.05), w!(Black, 0.6), w!(Yellow, 0.25)),
        Faction::YssarilTribes => prio!(
            w!(Green, 0.93),
            w!(Yellow, 0.63),
            w!(Red, 0.25),
            w!(Black, 0.1)
        ),
        Faction::ArgentFlight => prio!(w!(Orange, 1.6), w!(Blue, 0.15), w!(Green, 0.15)),
        Faction::Empyrean => prio!(w!(Purple, 1.6), w!(Red, 0.15), w!(Pink, 0.15)),
        Faction::MahactGeneSorcerers => prio!(w!(Yellow, 1.6), w!(Purple, 0.3)),
        Faction::NaazRokhaAlliance => prio!(w!(Green, 1.6), w!(Yellow, 0.3)),
        Faction::Nomad => prio!(w!(Blue, 1.25), w!(Purple, 0.65)),
        Faction::TitansOfUl => prio!(w!(Pink, 1.9)),
        Faction::VuilRaithCabal => prio!(w!(Red, 1.35), w!(Black, 0.4), w!(Pink, 0.1)),
        Faction::CouncilKeleres => prio!(
            w!(Purple, 0.7),
            w!(Blue, 0.5),
            w!(Orange, 0.35),
            w!(Yellow, 0.35)
        ),
    }
}

/// Assign colors to the factions according to priority.
pub fn assign_colors(factions: Vec<Faction>) -> eyre::Result<HashMap<Faction, Color>> {
    let mut map: HashMap<Color, (ColorPrio, &Faction)> = HashMap::new();

    for faction in factions.iter() {
        let mut prios = get_faction_color_prio(faction);
        log::debug!("Trying to select color for faction {faction:?}...");
        select_color_for_faction(&mut prios, &mut map, faction)?;
    }

    log::debug!("Final colors map: {map:?}");

    Ok(map.into_iter().map(|(a, (_, b))| (*b, a)).collect())
}

fn select_color_for_faction<'a>(
    prios: &mut BinaryHeap<ColorPrio>,
    map: &mut HashMap<Color, (ColorPrio, &'a Faction)>,
    faction: &'a Faction,
) -> eyre::Result<()> {
    log::debug!("Select color called, current map: {map:?}");
    // TODO: Cleanup...
    while let Some(my_prio) = prios.pop() {
        if let Some((other_prio, other_faction)) = map.get(&my_prio.color) {
            log::debug!(
                "Color {:?} have already been assigned to faction {:?}",
                my_prio.color,
                other_faction
            );
            if my_prio.weight <= other_prio.weight {
                log::debug!("They ({other_faction:?} :: {other_prio:?}) had higher prio than us ({faction:?} :: {my_prio:?})");
                // Just check the next color.
                continue;
            }

            // Find a new color for them.
            let mut remaining_prios = get_faction_color_prio(other_faction);
            while let Some(color_prio) = remaining_prios.pop() {
                if color_prio.color == other_prio.color {
                    break;
                }
            }

            let other_faction = other_faction.to_owned();
            let other_prio = other_prio.clone();

            log::debug!(
                "2: Taking the color {:?} (faction {faction:?})",
                my_prio.color
            );
            map.insert(my_prio.color.clone(), (my_prio, faction));

            log::debug!("Trying to find a new color for other faction {other_faction:?} who previously had {:?}, remaining prios for them {remaining_prios:?}", other_prio.color);
            if remaining_prios.is_empty() {
                // They ran out of colors, pick an unused one.
                let color = get_random_unused_color(map)?;
                log::debug!("They ran out of prio options, choosing random {color:?}");
                map.insert(color.clone(), (ColorPrio::base(color), other_faction));
            } else {
                log::debug!("Selecting new color for faction {other_faction:?}");
                select_color_for_faction(&mut remaining_prios, map, other_faction)?;
            }
            return Ok(());
        }

        log::debug!("Taking the color {:?} (faction {faction:?})", my_prio.color);
        map.insert(my_prio.color.clone(), (my_prio, faction));
        return Ok(());
    }

    // We have no more prioritised colors, pick any free one.
    let color = get_random_unused_color(map)?;
    log::debug!("Unable to find prio color, taking default {color:?}");
    map.insert(color.clone(), (ColorPrio::base(color), faction));
    Ok(())
}

fn get_random_unused_color<T>(map: &HashMap<Color, T>) -> eyre::Result<Color> {
    Color::iter().find(|c| !map.contains_key(c))
        .ok_or_eyre("No more colors to choose from? (This is should not happen as we should never be able to have more players than colors!)")
}
