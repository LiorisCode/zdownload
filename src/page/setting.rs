use crate::db::Settings;
use dioxus::prelude::*;
use rfd::AsyncFileDialog;

#[component]
pub fn Setting() -> Element {
    // 使用 Settings 结构体来管理设置
    let settings = use_signal(Settings::load);

    // 选项配置
    let quality_options = vec![
        ("best".to_string(), "最佳画质".to_string()),
        ("normal".to_string(), "普通画质".to_string()),
    ];

    let downvideo_options = vec![
        ("yes".to_string(), "下载视频列表".to_string()),
        ("no".to_string(), "不下载视频列表".to_string()),
    ];

    // 选择文件夹的函数
    let select_folder = {
        let mut settings = settings.clone();

        move |_| async move {
            if let Some(folder) = AsyncFileDialog::new()
                .set_title("选择下载文件夹")
                .pick_folder()
                .await
            {
                let path = folder.path().to_string_lossy().to_string();
                settings.write().download_path = path;
                settings.read().save();
            }
        }
    };

    rsx! {
        div { class: "p-4 max-w-md mx-auto",
            h1 { class: "text-2xl font-bold mb-6 text-center text-white",
                "设置选项"
            }

            // 视频质量选择
            Dropdown {
                title: "视频质量".to_string(),
                options: quality_options,
                selected_value: Signal::new(settings.read().quality.clone()),
                on_change: {
                    let mut settings = settings.clone();
                    Callback::new(move |value: String| {
                        settings.write().quality = value;
                        settings.read().save();
                    })
                },
            }

            // 视频列表下载选择
            Dropdown {
                title: "视频列表".to_string(),
                options: downvideo_options,
                selected_value: Signal::new(settings.read().down_video_list.clone()),
                on_change: {
                    let mut settings = settings.clone();
                    Callback::new(move |value: String| {
                        settings.write().down_video_list = value;
                        settings.read().save();
                    })
                },
            }

            // 下载路径选择
             div {
                 class: "bg-gray-800 rounded-xl p-4 mb-4 shadow-lg border border-gray-700",
                 h2 {
                     class: "text-lg font-semibold mb-3 text-white",
                     "下载路径"
                 }
                 div {
                     class: "flex items-center space-x-2",
                     input {
                         class: "flex-1 bg-gray-700 text-white rounded-lg px-4 py-2 border border-gray-600 focus:border-blue-500 focus:ring-2 focus:ring-blue-500/50",
                         readonly: true,
                         value: "{settings.read().download_path}",
                     }
                     button {
                         class: "bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg transition-colors",
                         onclick: select_folder,
                         "选择文件夹"
                     }
                 }
                 p {
                     class: "mt-2 text-sm text-gray-400",
                     "当前选择的下载文件夹将用于保存所有下载内容"
                 }
             }

            // 状态显示区域
            div { class: "mt-6 p-4 bg-gray-700 rounded-lg",
                h2 { class: "text-lg font-semibold mb-2 text-white", "当前设置" }
                div { class: "space-y-2",
                    p { class: "text-gray-300",
                        "视频质量: " span { class: "font-medium text-white",
                            {settings.read().quality.clone()}
                        }
                    }
                    p { class: "text-gray-300",
                        "视频列表下载: " span { class: "font-medium text-white",
                            {settings.read().down_video_list.clone()}
                        }
                    }
                    p { class: "text-gray-300",
                        "下载路径: " span { class: "font-medium text-white",
                            {settings.read().download_path.clone()}
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct DropdownProps {
    title: String,
    options: Vec<(String, String)>,
    selected_value: Signal<String>,
    on_change: Callback<String>,
}

#[component]
pub fn Dropdown(mut props: DropdownProps) -> Element {
    rsx! {
        div {
            class: "bg-gray-800 rounded-xl p-4 mb-4 shadow-lg border border-gray-700 transition-all hover:shadow-xl",
            h2 {
                class: "text-lg font-semibold mb-3 text-white",
                {props.title}
            }
            div {
                class: "relative",
                select {
                    class: "w-full bg-gray-700 text-white rounded-lg px-4 py-2 pr-8 border border-gray-600 focus:border-blue-500 focus:ring-2 focus:ring-blue-500/50 transition-all cursor-pointer appearance-none",
                    onchange: move |e| {
                        let value = e.value();
                        props.selected_value.set(value.to_string());
                        props.on_change.call(value);
                    },

                    for (value, text) in props.options.iter() {
                        option {
                            class: "bg-gray-800",
                            value: "{value}",
                            selected: *props.selected_value.read() == *value,
                            "{text}"
                        }
                    }
                }
                div {
                    class: "pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-400",
                    svg {
                        class: "w-5 h-5",
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M19 9l-7 7-7-7"
                        }
                    }
                }
            }
        }
    }
}
