use crate::state::{use_app_context, NavSection};
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn NavigationBar() -> Element {
    let ctx = use_app_context();
    let nav = ctx.nav;
    let active = *nav.read();

    let mut sidebar = ctx.sidebar_collapsed;
    let collapsed = *sidebar.read();
    let sidebar_class = if collapsed { "umbreon-sidebar collapsed" } else { "umbreon-sidebar" };

    rsx! {
        aside { class: "{sidebar_class}",
            div { class: "umbreon-brand-row",
                div { class: "umbreon-brand",
                    span { class: "umbreon-brand-text", "Umbreon" }
                }
                button {
                    class: "collapse-toggle",
                    onclick: move |_| {
                        let current = *sidebar.read();
                        *sidebar.write() = !current;
                    },
                    span { class: "material-icons", "menu" }
                }
            }
            nav { class: "umbreon-nav",
                for section in NavSection::ALL.into_iter().filter(|section| *section != NavSection::Settings) {
                    NavButton { section, active, nav }
                }
            }
            div { class: "sidebar-footer",
                NavButton { section: NavSection::Settings, active, nav }
            }
        }
    }
}

#[component]
fn NavButton(section: NavSection, active: NavSection, mut nav: Signal<NavSection>) -> Element {
    let is_active = section == active;
    let button_class = if is_active { "nav-btn active" } else { "nav-btn" };
    let label = section.label();
    let icon = section.icon();

    rsx! {
        button {
            class: button_class,
            onclick: move |_| {
                *nav.write() = section;
            },
            span { class: "nav-icon material-icons", "{icon}" }
            span { class: "nav-label", "{label}" }
        }
    }
}
