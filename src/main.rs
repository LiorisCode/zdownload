mod db;
mod page;
use dioxus::prelude::*;
use page::Page;

const MAIN_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        main {
            class:"min-h-screen bg-gray-900 text-white",
            div {
                class:"container mx-auto px-4 py-8",

                Page { }


            }
        }
    }
}
