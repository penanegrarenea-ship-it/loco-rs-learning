# Notes

## 学习者偏好
- 中文，干脆，少废话。
- 先给命令和真实结果，再用一两句说为什么。
- 有 axum 基础（参数提取、返回值、测试、带状态路由、中间件）；没用过 Rails。
  - 讲 Rails 惯例时用一两句解释，不要假设他懂。
  - 用 axum → Loco 对照，不用 Rails → Loco 对照。
- **用户用 Windows 11 + PowerShell。** 课里给的命令一律写 PowerShell 版：`curl.exe` 或 `Invoke-RestMethod`/`Invoke-WebRequest`，环境变量 `$env:PORT=5151`（用完 `Remove-Item Env:PORT`），路径反斜杠，查文件用 `Select-String`/`Get-ChildItem`。教练在 Linux（含 `~/.local/pwsh/pwsh` 7.6.8）真跑验证，输出照实贴。
  - Windows 专属的坑只写官方文档/源码能查到的：`--os` 在 Windows 默认 `windows`（多 `src\bin\tool.rs` + `cargo loco-tool` 别名）；`.cargo/config.toml` 对 MSVC 目标用 `rust-lld` 链接。MSVC 构建工具、长路径等官方没写，不写。

## 环境（教练这边，2026-10-24 核实）
- rustc / cargo 1.99.1；Loco MSRV 1.94。
- `loco` CLI 1.2.0（`cargo install loco`），框架 `loco-rs` 1.2.0（2026-09-22 发布）。
- 练习项目：`playground/admin_api`（`loco new --name admin_api --db sqlite --bg async --assets none`）。

## 教练草稿
- 文档 CLI reference 写 `LOCO_VERSION` “currently 1.1”，源码 `loco-new/src/lib.rs` 已是 `"1.2"`；生成的 Cargo.toml 实际是 `loco-rs = { version = "1.2" }`。以源码为准。
- CHANGELOG 顶部 `Unreleased` 段（只编译自己的数据库驱动、`testing-core` 等）尚未发布；1.2.0 生成的 Cargo.toml 里 sea-orm 仍同时带 sqlx-sqlite 和 sqlx-postgres。
- `loco new` 的模板（Rest API / SaaS 等）只能交互选，没有 `--template` 参数；用 `--db/--bg/--assets` 全部给出即可跳过所有提问。`--db sqlite --assets none` 实际效果就是带 DB + 用户鉴权的 REST API。
- 下一课候选：目录结构（src/app.rs 的 Hooks、controllers、models、dtos、config）。
- 第 2 课发现：`cargo loco routes` 实际是树形输出，add-controller 文档示例是扁平 `[GET] /_ping`；Hooks 参考页行号过时（1.2.0 `trait Hooks` 在 src/app.rs:422），且说 impl Hooks 由 `cargo loco generate` 生成（实为 `loco new`）；`--assets none` 项目里 `src/views/` 放 JSON 响应结构体，另有 `src/dtos/`。
- 官方 doctrine 反对 services/repositories 层（Active Record，胖 model）。DDD 拆层与此冲突，讲到时要明说是取舍。
- 下一课：第 3 课 路由和 controller（`cargo loco generate controller`、Routes API、extractor、format::json）。
