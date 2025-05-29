mod home;
mod setting;
use dioxus::prelude::*;
use home::Home;
use tokio::time::{sleep, Duration};

use setting::Setting;
#[derive(Clone, Copy, PartialEq)]
enum PageState {
    HomePage,
    SettingPage,
}

#[component]
pub fn Page() -> Element {
    let mut current_page = use_signal(|| PageState::HomePage);
    let current = *current_page.read();
    let isAnimating: Signal<bool> = use_signal(|| true);

    use_effect(move || {
        // 只在 isAnimating 变为 true 时触发
        if *isAnimating.read() {
            let mut isAnimating = isAnimating.clone();
            spawn(async move {
                sleep(Duration::from_millis(60)).await;
                isAnimating.set(false);
            });
        }
    });

    rsx! {
        nav {
            class:"flex justify-between items-center mb-8",
            h1 {
                class:"text-2xl font-bold bg-gradient-to-r from-blue-500 to-purple-500 bg-clip-text text-transparent",
                match current {
                     PageState::HomePage =>  "知视频下载助手",
                     PageState::SettingPage  => "设置"
                }

            },
            button {
                class:"text-gray-300 hover:text-white transition-colors duration-200",
                onclick: move |_| {

                    current_page.set(match &current {
                        PageState::HomePage => PageState::SettingPage,
                        PageState::SettingPage => PageState::HomePage,
                    });



                },
                match current {
                         PageState::SettingPage => rsx! {
                             // This SVG will be shown when currently on the SettingPage,
                             // and clicking it will navigate to HomePage. So it represents "go to Home".
                             svg {
                                 xmlns: "http://www.w3.org/2000/svg",
                                 class: "h-6 w-6",
                                 fill: "none",
                                 view_box: "0 0 24 24",
                                 stroke: "currentColor",
                                 path {
                                     stroke_linecap: "round",
                                     stroke_linejoin: "round",
                                     stroke_width: "2",
                                     d: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
                                 }
                             }
                         },
                         PageState::HomePage => rsx! {
                             // This SVG will be shown when currently on the HomePage,
                             // and clicking it will navigate to SettingPage. So it represents "go to Settings".
                             svg {
                                 xmlns: "http://www.w3.org/2000/svg",
                                 class: "h-6 w-6",
                                 fill: "none",
                                 view_box: "0 0 24 24",
                                 stroke: "currentColor",
                                 path {
                                     stroke_linecap: "round",
                                     stroke_linejoin: "round",
                                     stroke_width: "2",
                                     d: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                                 }
                                 path {
                                     stroke_linecap: "round",
                                     stroke_linejoin: "round",
                                     stroke_width: "2",
                                     d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                                 }
                             }
                         },
                }
            }
        }

        div {
            class: format!(
                   " inset-0 transition-opacity duration-300 {} ",
                    if *isAnimating.read() { " opacity-0" } else { "opacity-100" }
                ),
            match *current_page.read() {
                PageState::HomePage => rsx! { Home {} },
                PageState::SettingPage => rsx! { Setting {}},
            }


        }
    }
}
