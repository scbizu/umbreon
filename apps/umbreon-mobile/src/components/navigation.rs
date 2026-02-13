use crate::state::{use_app_context, NavSection};
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn NavigationBar() -> Element {
    let ctx = use_app_context();
    let nav = ctx.nav;
    let active = *nav.read();

    rsx! {
        nav { class: "bottom-nav",
            for section in NavSection::ALL.into_iter() {
                NavButton { section, active, nav }
            }
        }
    }
}

#[component]
fn NavButton(section: NavSection, active: NavSection, mut nav: Signal<NavSection>) -> Element {
    let is_active = section == active;
    let button_class = if is_active { "bottom-nav-btn active" } else { "bottom-nav-btn" };
    let label = section.label();
    let icon = section.icon();

    rsx! {
        button {
            class: button_class,
            onclick: move |_| {
                *nav.write() = section;
            },
            span { class: "material-icons", "{icon}" }
            span { "{label}" }
        }
    }
}
