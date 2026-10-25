# Glossary

只收录你已经能正确使用的术语。每条一两句话。

## `loco new`
Loco 生成器（`cargo install loco` 装的独立命令）唯一的子命令，用来生成新项目。`--name --db --bg --assets` 全给就不再提问。
_Avoid_: “loco 模板”、“loco init”

## `cargo loco start`
在项目目录里启动应用。`cargo loco` 是项目 `.cargo/config.toml` 里的别名（`run --`），实际运行的是项目自己的二进制。
_Avoid_: “loco start”（少了 cargo 就是全局生成器，没有 start）

## `/_ping`
Loco 内置的存活检查接口，返回 `{"ok":true}`，不用写代码就有。用来确认服务已经起来。

## `AppRoutes` / `add_route`
`AppRoutes` 是整个应用的路由表，在 `src/app.rs` 的 `Hooks::routes` 里构造；`.add_route(controllers::xxx::routes())` 把一个 controller 的路由接进来，漏了就 404。
_Avoid_: “Router”（axum 的 `Router` 是框架最后生成的东西，不是你在 app.rs 里写的）

## `Routes::new().prefix().add()`
每个 controller 的 `pub fn routes()` 返回的一组路由：`prefix` 定公共前缀，`add(路径, get(handler))` 逐条登记，`get(...)` 就是 axum 的 `MethodRouter`。

## `AppContext`
Loco 预先定义好的 axum State 结构体，handler 里写 `State(ctx): State<AppContext>` 拿到，里面有 `db`、`config` 等。
_Avoid_: “AppState”（那是你在 axum 里自己起的名字）

## `cargo loco routes`
列出应用全部路由的命令，按前缀折叠成树，右侧是完整路径。用来确认路由有没有注册上。
