use super::{expansions::Expansion, game_settings::Expansions};

/// A game component that comes with an expansion, which later expansions can replace or remove.
pub trait Component: PartialEq + Sized + 'static {
    /// An expansion every component of this kind needs on top of its own.
    const REQUIRES: Expansion = Expansion::Base;
    /// The versions of each component that newer versions replace, written with `replacements!`.
    const REPLACEMENTS: &'static [Chain<Self>] = &[];
    /// Components that an expansion removes without a replacement.
    const REMOVED: &'static [(Self, Expansion)] = &[];

    /// The expansion that introduced this component.
    fn expansion(&self) -> Expansion;

    /// Whether this version is in play with the provided expansions.
    fn is_enabled_in(&self, expansions: &Expansions) -> bool {
        if !expansions.is_enabled(&Self::REQUIRES) {
            // The component hasn't been introduced in these expansions.
            return false;
        }

        if !expansions.is_enabled(&self.expansion()) {
            // The component hasn't been introduced in these expansions.
            return false;
        }

        if Self::REMOVED
            .iter()
            .filter(|(component, _)| component == self)
            .any(|(_, expansion)| expansions.is_enabled(expansion))
        {
            // The component has been removed in one of the enabled expansions.
            return false;
        }

        if let Some(chain) = Self::REPLACEMENTS.iter().find(|chain| chain.contains(self)) {
            // The component is part of a replacement chain; check that this is the version that is in play given the current expansions.
            return chain.in_play(expansions) == Some(self);
        }

        true
    }
}

/// The versions of a component, from the oldest to the newest.
#[derive(Debug)]
pub struct Chain<T: 'static>(&'static [T]);

impl<T: 'static> Chain<T> {
    pub(crate) const fn new(versions: &'static [T]) -> Self {
        Self(versions)
    }
}

impl<T: Component> Chain<T> {
    /// The first version of the component.
    pub fn oldest(&self) -> &T {
        &self.0[0]
    }

    /// The versions of the component, from the newest to the oldest.
    pub fn newest_first(&self) -> impl Iterator<Item = &T> {
        self.0.iter().rev()
    }

    /// Whether the provided component is one of these versions.
    fn contains(&self, component: &T) -> bool {
        self.0.contains(component)
    }

    /// The newest version that comes with the provided expansions, or none if the component hasn't been introduced in any of them.
    fn in_play(&self, expansions: &Expansions) -> Option<&T> {
        if !expansions.is_enabled(&self.oldest().expansion()) {
            return None;
        }
        self.newest_first()
            .find(|version| expansions.is_enabled(&version.expansion()))
    }
}

/// Defines `REPLACEMENTS` from chains of versions written `Old -> New`.
macro_rules! replacements {
    ($(
        $first:ident $(($($first_args:tt)*))? $(-> $next:ident $(($($next_args:tt)*))?)+
    ),* $(,)?) => {
        const REPLACEMENTS: &'static [$crate::common::component::Chain<Self>] = &[$(
            $crate::common::component::Chain::new(&[
                Self::$first $(($($first_args)*))? $(, Self::$next $(($($next_args)*))?)+
            ])
        ),*];
    };
}
pub(crate) use replacements;

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::components::{
        action_card::ActionCard, agenda::Agenda, frontier_card::FrontierCard, leaders::Leader,
        planet::Planet, planet_attachment::PlanetAttachment, relic::Relic,
        strategy_card::StrategyCardVersion, tech::Technology,
    };

    use super::*;

    fn all_expansions() -> impl Iterator<Item = Expansions> {
        (0..64).map(|bits: u32| Expansions {
            prophecy_of_kings: bits & 1 != 0,
            codex_1: bits & 2 != 0,
            codex_2: bits & 4 != 0,
            codex_3: bits & 8 != 0,
            codex_4: bits & 16 != 0,
            thunders_edge: bits & 32 != 0,
        })
    }

    fn check_versions<T: Component + Debug>() {
        for (i, chain) in T::REPLACEMENTS.iter().enumerate() {
            let versions: Vec<&T> = chain.newest_first().collect();
            for (j, version) in versions.iter().enumerate() {
                for other in &versions[j + 1..] {
                    assert_ne!(version, other, "{chain:?}");
                    assert_ne!(version.expansion(), other.expansion(), "{chain:?}");
                }
                for other in &T::REPLACEMENTS[i + 1..] {
                    assert!(!other.contains(version), "{version:?} is in two chains");
                }
            }
        }

        for expansions in all_expansions() {
            for chain in T::REPLACEMENTS {
                let in_play = chain
                    .newest_first()
                    .filter(|version| version.is_enabled_in(&expansions))
                    .count();
                assert!(in_play <= 1, "{chain:?} with {expansions:?}");

                let removed = T::REMOVED.iter().any(|(component, expansion)| {
                    chain.contains(component) && expansions.is_enabled(expansion)
                });
                if expansions.is_enabled(&T::REQUIRES)
                    && expansions.is_enabled(&chain.oldest().expansion())
                    && !removed
                {
                    assert_eq!(in_play, 1, "{chain:?} with {expansions:?}");
                }
            }

            for (component, expansion) in T::REMOVED {
                if expansions.is_enabled(expansion) {
                    assert!(
                        !component.is_enabled_in(&expansions),
                        "{component:?} with {expansions:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn technology_versions_are_consistent() {
        check_versions::<Technology>();
    }

    #[test]
    fn action_card_versions_are_consistent() {
        check_versions::<ActionCard>();
    }

    #[test]
    fn leader_versions_are_consistent() {
        check_versions::<Leader>();
    }

    #[test]
    fn strategy_card_versions_are_consistent() {
        check_versions::<StrategyCardVersion>();
    }

    #[test]
    fn planet_versions_are_consistent() {
        check_versions::<Planet>();
    }

    #[test]
    fn agenda_versions_are_consistent() {
        check_versions::<Agenda>();
    }

    #[test]
    fn relic_versions_are_consistent() {
        check_versions::<Relic>();
    }

    #[test]
    fn frontier_card_versions_are_consistent() {
        check_versions::<FrontierCard>();
    }

    #[test]
    fn planet_attachment_versions_are_consistent() {
        check_versions::<PlanetAttachment>();
    }
}
