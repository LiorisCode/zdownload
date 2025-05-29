// use crate::db::SettingsDatabase;
use crate::db::Settings;
use dioxus::prelude::*;
use regex::Regex;
use std::env::temp_dir;
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    sync::mpsc,
    thread,
};
use tokio;
use tokio::time::{self, Duration};
#[component]
pub fn Home() -> Element {
    let mut video_text = use_signal(|| String::new());
    let mut is_downloading = use_signal(|| false);
    let mut download_status = use_signal(|| String::new());
    let settings = use_signal(Settings::load);

    let instructions = vec![
        Instruction::new(
            1,
            "核心技术：基于业界领先的 yt-dlp 引擎和 ffmpeg 多媒体处理框架构建".into(),
        ),
        Instruction::new(2, "自定义配置：支持部分yt-dlp的配置".into()),
        Instruction::new(
            3,
            "广泛兼容：支持 YouTube、Bilibili 等 1000+ 个视频平台的下载解析".into(),
        ),
        Instruction::new(
            4,
            "极简操作：在输入框粘贴视频链接后，一键即可开始智能下载".into(),
        ),
        Instruction::new(5, "首次下载记得设置下载路径".into()),
    ];

    let mut download_video = move || {
        is_downloading.set(true);
        download_status.set(String::from("开始下载..."));
        let url = video_text.read().to_string();
        if url.is_empty() {
            is_downloading.set(false);
            download_status.set(String::from("请输入有效的视频链接"));
            ()
        }

        // 1. 创建临时文件路径
        let temp_path = temp_dir().join("yt-dlp-temp");
        let ffmpeg_path = temp_dir().join("ffmpeg-temp");
        #[cfg(not(windows))]
        {
            // 2. 写入 yt-dlp 二进制
            let yt_dlp_exe = include_bytes!("./bin/yt-dlp");
            std::fs::write(&temp_path, yt_dlp_exe)
                .map_err(|e| format!("写入临时文件失败: {}", e))
                .unwrap();

            let ffmpeg_exe = include_bytes!("./bin/ffmpeg");
            std::fs::write(&ffmpeg_path, ffmpeg_exe)
                .map_err(|e| format!("写入临时文件失败: {}", e))
                .unwrap();
        }

        #[cfg(windows)]
        {
            yt_dlp_bytes = include_bytes!("./bin/yt-dlp.exe");
            ffmpeg_bytes = include_bytes!("./bin/ffmpeg.exe");
            yt_dlp_name = "yt-dlp-temp.exe";
            ffmpeg_name = "ffmpeg-temp.exe";
        }

        // 3. 在 Unix 系统上设置可执行权限
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&temp_path, std::fs::Permissions::from_mode(0o755))
                .map_err(|e| format!("设置可执行权限失败: {}", e))
                .unwrap();

            std::fs::set_permissions(&ffmpeg_path, std::fs::Permissions::from_mode(0o755))
                .map_err(|e| format!("设置可执行权限失败: {}", e))
                .unwrap();
        }

        let (tx, rs) = mpsc::channel::<String>();
        let dp = settings.read().download_path.clone();
        let output_template = format!("{}/%(title)s.%(ext)s", dp);
        let is_down_list = settings.read().down_video_list.clone();
        let qua = settings.read().quality.clone();

        thread::spawn({
            move || {
                let mut cmd = Command::new(&temp_path); // 先创建基础命令

                // 根据URL类型添加不同参数
                if url.contains("youtube.com") || url.contains("youtu.be") {
                    // YouTube 专用下载逻辑
                    cmd.arg(&url)
                        .arg("--newline")
                        .arg("--no-check-certificate")
                        .arg("--ffmpeg-location")
                        .arg(&ffmpeg_path)
                        .arg("--retries")
                        .arg("10")
                        .arg("-o")
                        .arg(&output_template)
                        .args(if is_down_list.to_string() == "no" {
                            vec!["--no-playlist"]
                        } else {
                            vec![]
                        });
                } else {
                    // 其他网站下载逻辑
                    cmd.arg(&url)
                        .arg("--newline")
                        .arg("--no-check-certificate")
                        .arg("--ffmpeg-location")
                        .arg(&ffmpeg_path)
                        .args(match qua.as_str() {
                            "best" => vec![
                                "-f",
                                "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best",
                                "--merge-output-format",
                                "mp4",
                            ],
                            _ => vec!["-f", "best[ext=mp4]"],
                        })
                        .arg("-o")
                        .arg(&output_template)
                        .args(if is_down_list.to_string() == "no" {
                            vec!["--no-playlist"]
                        } else {
                            vec![]
                        });
                }

                let mut child = cmd
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("无法启动进程");

                // 获取输出流
                let stdout = child.stdout.take().unwrap();
                let stderr = child.stderr.take().unwrap();

                // 创建读取线程
                let tx_stdout = tx.clone();
                let stdout_thread = thread::spawn(move || {
                    let reader = BufReader::new(stdout);
                    for line in reader.lines() {
                        tx_stdout.send(line.unwrap_or_default()).unwrap();
                    }
                });

                let tx_stderr = tx.clone();
                let stderr_thread = thread::spawn(move || {
                    let reader = BufReader::new(stderr);
                    for line in reader.lines() {
                        tx_stderr.send(line.unwrap_or_default()).unwrap();
                    }
                });

                let status = child.wait().unwrap();

                // 加入线程
                stdout_thread.join().unwrap();
                stderr_thread.join().unwrap();

                if status.success() {
                    let _ = std::fs::remove_file(&temp_path);
                    let _ = std::fs::remove_file(&ffmpeg_path);
                    tx.send("下载完成".to_string()).unwrap();
                } else {
                    tx.send(format!("线程退出状态：{}", status)).unwrap();
                }
            }
        });

        spawn(async move {
            while let Ok(line) = rs.recv() {
                download_status.with_mut(|t| {
                    t.push_str(&line);
                    t.push('\n');
                });
                let _ = document::eval(
                    r#"
                                const container = document.getElementById("output-container");
                                if (container) {
                                    container.scrollTop = container.scrollHeight+10;
                                }
                                "#,
                );

                time::sleep(Duration::from_millis(500)).await;
            }
        });

        is_downloading.set(false);
    };

    // end

    let mut handle_clear = move || {
        // 实现清除输入的逻辑
        video_text.set(String::new());
        download_status.set(String::new());
    };

    let extract_url = move |text: &str| -> Option<String> {
        let url_regex = Regex::new(
            r#"(?xi)
                \b                            # 单词边界
                (                             # 开始捕获组
                    https?://[^\s<>"]+        # http:// 或 https:// 开头的 URL
                    |                        # 或
                    ftp://[^\s<>"]+          # ftp:// 开头的 URL
                    |                        # 或
                    www\.[^\s<>"]+           # www. 开头的 URL（不带协议）
                    |                        # 或
                    [a-z0-9.-]+\.[a-z]{2,}/[^\s<>"]*  # 类似 "example.com/path" 的格式
                )
            "#,
        )
        .ok()?;

        url_regex.find(text).map(|m| m.as_str().to_string())
    };

    rsx! {
        div {
            class: "space-y-6",

            // 下载输入区域
            div {
                class: "bg-gray-800 rounded-lg p-6 mb-6 shadow-lg hover:shadow-xl transition-shadow duration-200",

                div {
                    class: "flex gap-4",

                    input {

                        r#type: "text",
                        value: "{video_text}",
                        placeholder: "请输入下载链接...",
                        class: "flex-1 bg-gray-700 text-white rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all duration-200",
                        oninput: move |e| {
                                       if let Some(url) = extract_url(&e.value()) {
                                           video_text.set(url); // 仅设置解析后的 URL
                                       }
                                   }
                    }

                    button {
                        disabled: "{is_downloading}",
                        onclick: move |_| download_video()
                        ,
                        class: "bg-blue-600 hover:bg-blue-700 text-white px-6 py-2 rounded-lg transition-all duration-200 flex items-center gap-2 hover:scale-105 active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed",

                        if *is_downloading.read() {

                                svg {
                                    class: "h-5 w-5 mr-3 -ml-1 size-5 animate-spin text-white",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    view_box: "0 0 24 24",

                                    circle {
                                        class: "opacity-25",
                                        cx: "12",
                                        cy: "12",
                                        r: "10",
                                        stroke: "currentColor",
                                        stroke_width: "4"
                                    }
                                    path {
                                        class: "opacity-75",
                                        fill: "currentColor",
                                        d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                                    }
                                }
                                "下载中..."

                        } else {

                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    class: "h-5 w-5",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke: "currentColor",

                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
                                    }
                                }
                                "下载"

                        }
                    }

                    button {
                        disabled: "{is_downloading}",
                        onclick: move |_| handle_clear(),
                        class: "bg-gray-700 hover:bg-gray-600 text-white px-4 py-2 rounded-lg transition-all duration-200 hover:scale-105 active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed",

                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "h-5 w-5",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",

                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M6 18L18 6M6 6l12 12"
                            }
                        }
                    }
                }
            }

            // 输出区域
            div {
                class: "bg-gray-800 rounded-lg p-6 shadow-lg hover:shadow-xl transition-shadow duration-200",

                h2 {
                    class: "text-xl font-semibold mb-4 bg-gradient-to-r from-blue-500 to-purple-500 bg-clip-text text-transparent",
                    "下载状态"
                }

                div {
                    id:"output-container",

                    class: "bg-gray-700 rounded-lg p-4 min-h-[200px] whitespace-pre-line overflow-y-auto max-h-[400px]",

                    p {

                        class: "text-gray-300",
                        "{download_status}"
                    }
                }
            }

            // 使用说明
            div {
                class: "mt-8 bg-gray-800 rounded-lg p-6 shadow-lg hover:shadow-xl transition-shadow duration-200",

                h2 {
                    class: "text-xl font-semibold mb-4 bg-gradient-to-r from-blue-500 to-purple-500 bg-clip-text text-transparent",
                    "使用说明"
                }

                ul {
                    class: "space-y-2 text-gray-300",

                    for instruction_item in instructions {
                        li {
                            class: "flex items-center gap-2",
                            key: "{instruction_item.id}",

                            span {
                                class: "w-6 h-6 bg-blue-600 rounded-full flex items-center justify-center text-sm",
                                "{instruction_item.id}"
                            }
                            "{instruction_item.text}"
                        }
                    }
                }
            }

            // 作者信息
            div {
                class: "mt-8 text-center text-gray-400",

                p {
                    class: "flex items-center justify-center gap-2",

                    "Created with"
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-5 w-5 text-red-500",
                        view_box: "0 0 20 20",
                        fill: "currentColor",

                        path {
                            fill_rule: "evenodd",
                            d: "M3.172 5.172a4 4 0 015.656 0L10 6.343l1.172-1.171a4 4 0 115.656 5.656L10 17.657l-6.828-6.829a4 4 0 010-5.656z",
                            clip_rule: "evenodd"
                        }
                    }
                    "by 魔法师"
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    id: i8,
    text: String,
}

impl Instruction {
    fn new(id: i8, text: String) -> Self {
        Self { id, text }
    }
}
