use dioxus::prelude::*;

const COMPLEXITY_SCSS: Asset = asset!("/assets/styling/components/complexity.scss");

const MAX_COMPLEXITY: u8 = 3;

pub fn complexity_symbols(complexity: u8) -> String {
    (1..=MAX_COMPLEXITY)
        .map(|slot| if slot <= complexity { '▰' } else { '▱' })
        .collect()
}

#[component]
pub fn Complexity(complexity: u8) -> Element {
    rsx! {
        document::Stylesheet { href: COMPLEXITY_SCSS }

        span {
            class: "complexity complexity-{complexity}",
            title: "Complexity {complexity}",
            "aria-label": "Complexity {complexity}",
            for slot in 1..=MAX_COMPLEXITY {
                span {
                    class: if slot <= complexity { "complexity-slot filled" } else { "complexity-slot" },
                }
            }
        }
    }
}
