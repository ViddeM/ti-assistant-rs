use super::{expansions::Expansion, game_settings::Expansions};

/// A kind of component whose printed versions replace each other as expansions are added.
pub trait Versioned: PartialEq + Sized + 'static {
    /// An expansion every component of this kind needs on top of its own.
    const REQUIRES: Expansion = Expansion::Base;
    /// The versions of each component that has more than one, oldest first.
    const VERSIONS: &'static [&'static [Self]];
    /// Components that an expansion removes without a replacement.
    const REMOVED: &'static [(Self, Expansion)] = &[];

    /// The expansion that introduced this version.
    fn expansion(&self) -> Expansion;

    /// Whether this version is in play with the provided expansions.
    fn is_enabled_in(&self, expansions: &Expansions) -> bool {
        expansions.is_enabled(&Self::REQUIRES)
            && expansions.is_enabled(&self.expansion())
            && !Self::REMOVED
                .iter()
                .any(|(component, expansion)| component == self && expansions.is_enabled(expansion))
            && Self::VERSIONS
                .iter()
                .find(|versions| versions.contains(self))
                .is_none_or(|versions| {
                    expansions.is_enabled(&versions[0].expansion())
                        && versions
                            .iter()
                            .rfind(|version| expansions.is_enabled(&version.expansion()))
                            == Some(self)
                })
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::components::{action_card::ActionCard, tech::Technology};

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

    fn check_versions<T: Versioned + Debug>() {
        for (i, versions) in T::VERSIONS.iter().enumerate() {
            for (j, version) in versions.iter().enumerate() {
                for other in &versions[j + 1..] {
                    assert_ne!(version, other, "{versions:?}");
                    assert_ne!(version.expansion(), other.expansion(), "{versions:?}");
                }
                for other in &T::VERSIONS[i + 1..] {
                    assert!(!other.contains(version), "{version:?} is in two chains");
                }
            }
        }

        for expansions in all_expansions() {
            for versions in T::VERSIONS {
                let in_play = versions
                    .iter()
                    .filter(|version| version.is_enabled_in(&expansions))
                    .count();
                assert!(in_play <= 1, "{versions:?} with {expansions:?}");

                let removed = T::REMOVED.iter().any(|(component, expansion)| {
                    versions.contains(component) && expansions.is_enabled(expansion)
                });
                if expansions.is_enabled(&T::REQUIRES)
                    && expansions.is_enabled(&versions[0].expansion())
                    && !removed
                {
                    assert_eq!(in_play, 1, "{versions:?} with {expansions:?}");
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
}
