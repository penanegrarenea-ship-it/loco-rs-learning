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
- 第 3 课发现：`generate controller admin` 前缀变复数 `api/admins/`；add-controller 文档的 `curl .../api/notes/`（尾斜杠）实测 404；文档的 `cargo test --test requests_notes` 不存在，只有 `--test mod`；未匹配路由回 404 + “Welcome to Loco!” HTML（fallback 中间件）。playground 里已有 `controllers/admin.rs`（ping、stats/{section}?limit）和 3 个 request 测试。
- 第 3 课练习待交（用户在外面，回来补；收到后再写 learning-records/0003 和 GLOSSARY）。
- 第 4 课发现：`generate model` 写 migration → 注入 `migration/src/lib.rs` 两处 → 加 tests/models → 自动 `db migrate` + `db entities`；结尾提示 “You can now apply it with `cargo loco db migrate && cargo loco db entities`” 是多余的（Your First App 也说已应用）。`db entities` 依赖外部 sea-orm-cli（add-model 页没提），缺它时停在一半（migration 已执行）。`db entities` 整个重写 `_entities/`（连 users.rs 头注释和 derive 顺序也变）；`models/<name>.rs` 仅不存在时创建（`fix_entities` 的 `if !new_file.exists()`）。生成后对整个项目跑 `cargo fmt`（`src/cli.rs` `format_generated_code`，改了第 3 课的 admin.rs 格式）。`int` → `BigIntegerNull`/`Option<i64>`。seed 需手动在 `Hooks::seed/truncate` 加行；fixture 缺 NOT NULL 列报 `missing field`；不带 `--reset` 重复 seed 报 `UNIQUE constraint failed: users.id`。开发环境 `auto_migrate: true`：`db down` 后 `cargo loco start` 会自动重新建表（第 4 课练习 3 的答案）。
- fixture 用英文商品名：避免 Windows 控制台/PS 5.1 对中文 JSON 的编码问题（官方没写，不在课里讲）。
- 教练机已装 sea-orm-cli 2.0.4（`--no-default-features --features codegen,sqlx-sqlite,sqlx-postgres,runtime-tokio-rustls`，因缺 OpenSSL 头文件）。
- playground 现有：`products` 表 + fixture 3 行，`products::Model::list_on_sale`，`GET /api/admin/products`。
- 下一课：第 5 课 scaffold 脚手架。
- 第 4 课答疑（迁移/entities/seed）核实：`db down [STEPS]` 默认 1；无 `db up`/`db fresh`；`db reset` = `M::fresh`+migrate（不 seed）；`db truncate` = `Hooks::truncate`（`delete_many`，只清列出的表）；`seed --reset` 先 `db::reset` 再 `Hooks::seed`；`auto_migrate` 只在 `create_app`（start / 测试 boot）里经 `converge` 生效，`db` 子命令不经过。`generate migration` 不自动 migrate/entities，但也跑 cargo fmt。手动 ALTER 加列 → entities 会带上 → reset 后列没了 → seed 报 `has no column named note`、接口 500 `no such column`。CLI reference 写 DbCommands 在 `cli.rs:505-520`，1.2.0 实为 505–541。sku 演示列已回滚干净。
