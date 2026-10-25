# 能从路由一路找到 handler，认得出 axum extractor

能在 Windows PowerShell 上用 `Select-String` 找到 `app.rs` 里 `with_default_routes().add_route(controllers::auth::routes())`，能用 `cargo loco routes` 列出 12 条路由，能从一条路由（`/current`、`/login`）定位到它的注册行和 handler 行。在 handler 签名里认出 `State(ctx): State<AppContext>` 是 State、`Json(params): Json<LoginParams>` 是请求体，回忆题答对（漏了 `add_route` 路由就不注册）。所以下一课可以直接让他自己写 controller、注册路由，不用再讲 `Hooks::routes` 是什么。

Evidence: 2026-10-25 第 2 课练习，在 Windows 上自己跑出的输出（auth.rs:269/162，266/137）。
