//! Many components implement `Ord` by comparing display names. Those values are used as
//! `BTreeSet` elements and `BTreeMap` keys in the game state, so two distinct values must
//! never compare as equal, otherwise one silently replaces the other.

use std::{cmp::Ordering, fmt::Debug};

use strum::IntoEnumIterator;
use ti_helper_game_data::{
    common::{color::Color, faction::Faction},
    components::{
        agenda::Agenda,
        frontier_card::FrontierCard,
        objectives::{Objective, public::PublicObjective, secret::SecretObjective},
        planet::Planet,
        relic::Relic,
        tech::Technology,
    },
};

fn assert_ord_consistent_with_eq<T: Ord + Debug>(values: impl Iterator<Item = T>) {
    let mut values: Vec<T> = values.collect();
    values.sort();
    for pair in values.windows(2) {
        assert_ne!(
            pair[0].cmp(&pair[1]),
            Ordering::Equal,
            "{:?} and {:?} compare as equal",
            pair[0],
            pair[1],
        );
    }
}

#[test]
fn ord_is_consistent_with_eq() {
    assert_ord_consistent_with_eq(Planet::iter());
    assert_ord_consistent_with_eq(Technology::iter());
    assert_ord_consistent_with_eq(Relic::iter());
    assert_ord_consistent_with_eq(SecretObjective::iter());
    assert_ord_consistent_with_eq(PublicObjective::iter());
    assert_ord_consistent_with_eq(
        PublicObjective::iter()
            .map(Objective::from)
            .chain(SecretObjective::iter().map(Objective::from)),
    );
    assert_ord_consistent_with_eq(Agenda::iter());
    assert_ord_consistent_with_eq(Faction::iter());
    assert_ord_consistent_with_eq(Color::iter());
    assert_ord_consistent_with_eq(FrontierCard::iter());
}
