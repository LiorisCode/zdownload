# Z视频下载助手

> 作者：魔法师

## 项目简介

Z视频下载助手是一个基于 **Rust** 语言开发的现代化桌面/网页视频下载工具。项目采用了 [Dioxus](https://dioxuslabs.com/) 作为前端 UI 框架，界面风格使用 [TailwindCSS](https://tailwindcss.com/) 实现，开发体验在 [Zed 编辑器](https://zed.dev/) 下极佳。

本项目集成了 [yt-dlp](https://github.com/yt-dlp/yt-dlp) 作为视频下载引擎，并依赖 [ffmpeg](https://ffmpeg.org/) 进行多媒体处理（如音视频合并、转码等），支持多平台高质量下载。

---

## 技术栈

- **Rust**：高性能、安全的系统级编程语言
- **Dioxus**：Rust 生态下的现代前端/桌面 UI 框架
- **TailwindCSS**：原子化 CSS 框架，快速构建美观响应式界面
- **Zed 编辑器**：现代化、极速的代码编辑器
- **yt-dlp**：强大的命令行视频下载工具，支持 1000+ 平台
- **ffmpeg**：业界领先的多媒体处理工具

---

## 主要特性

- 支持 YouTube、Bilibili 等主流平台的视频下载
- 支持最高画质与音频自动合并
- 支持下载单个视频或视频列表
- 下载路径、画质等参数可自定义
- 实时显示下载进度与日志
- 跨平台支持（Windows、macOS、Linux，需要自己打包，如有需要提iss）

---
