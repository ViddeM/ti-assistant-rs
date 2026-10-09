use dioxus::prelude::*;

use dioxus_free_icons::icons::fa_brands_icons::FaGitAlt;
use dioxus_free_icons::Icon;

use crate::Route;

const GIT_SOURCE_URL: &'static str = "https://github.com/viddem/ti-assistant-rs";
const SOURCE_CODE_SCSS: Asset = asset!("/assets/source_code.scss");

#[component]
pub fn SourceCodeLinkButton() -> Element {
    rsx! {
        document::Stylesheet { href: SOURCE_CODE_SCSS }

        Link {
            class: "source-code-button",
            to: NavigationTarget::<Route>::External(GIT_SOURCE_URL.to_string()),
            new_tab: true,
            Icon { icon: FaGitAlt }
        }
    }
}
