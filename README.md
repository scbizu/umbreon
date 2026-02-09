# Umbreon Monorepo

Umbreon 是一个完全自定义的移动应用与服务体系，围绕 Rust / Dioxus 技术栈构建。该仓库采用 monorepo 方式，集中管理移动端 App、后端服务以及云端 Worker/配置等代码。

## 目录结构

```
.
├── apps/
│   └── umbreon-mobile/        # Dioxus 安卓 App（待接入 feed、播放器、记忆系统）
├── services/
│   └── feed-aggregator/       # 聚合 ATOM/RSS/RSSHub/自定义源的后端服务
├── workers/
│   └── cloud-worker/          # Cloud/Edge worker，占位
├── crates/
│   └── umbreon-core/          # 共享模型、配置、客户端 SDK
├── docs/
│   └── umbreon-prd.md         # 产品需求文档
├── Cargo.toml                 # Rust workspace 配置
└── README.md
```

## 快速开始

1. 安装 Rust 工具链（nightly 推荐）与 Android 开发依赖。
2. 在仓库根目录运行：
   ```bash
   cargo check
   ```
   确认 workspace 可以正常编译。
3. 后续将依次完善：
   - `apps/umbreon-mobile`: Dioxus Mobile 应用（Feed、播放器、弹幕、supermemory 对接）。
   - `services/feed-aggregator`: 聚合服务，提供统一 API。
   - `workers/cloud-worker`: 处理定时任务/远端抓取等。
   - `crates/umbreon-core`: 统一数据模型、配置管理、客户端工具。

更多背景和详细需求见 [docs/umbreon-prd.md](docs/umbreon-prd.md)。
