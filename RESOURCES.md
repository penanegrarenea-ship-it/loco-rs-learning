# Resources

只认三个官方来源。文档和源码不一致时，以源码 / 当前发布版为准。

## Knowledge
- [loco.rs](https://loco.rs/) — 官网首页，版本与整体介绍。
- [loco.rs/docs](https://loco.rs/docs/) — 官方文档入口（教程 / How-to / Reference / Explanation 四类）。
- [github.com/loco-rs/loco](https://github.com/loco-rs/loco) — 源码。版本看根 `Cargo.toml`，改动看 `CHANGELOG.md`；`loco` CLI 在 `loco-new/`。
  - [Cargo.toml](https://github.com/loco-rs/loco/blob/master/Cargo.toml) — `loco-rs` 当前版本和 feature 列表。
  - [CHANGELOG.md](https://github.com/loco-rs/loco/blob/master/CHANGELOG.md) — 每版改了什么、怎么升级；注意顶部 `Unreleased` 段还没发布。
- [Your First App](https://loco.rs/docs/tutorials/your-first-app/) — 安装、`loco new`、`cargo loco start`、第一个 scaffold。第 1 课主读。
- [CLI reference](https://loco.rs/docs/reference/cli/) — `loco new` 全部参数和 `cargo loco` 全部子命令。查命令时用。
- [Coming from Axum](https://loco.rs/docs/explanation/coming-from-axum/) — axum 概念到 Loco 的对照。你有 axum 底子，这页最省力。
- [Architecture: the request lifecycle](https://loco.rs/docs/explanation/architecture/) — 启动流程、AppContext、请求怎么走、中间件为什么是 LIFO。
- [Add a controller](https://loco.rs/docs/how-to/add-controller/) — 写路由和 handler。
- [Configuration](https://loco.rs/docs/reference/configuration/) — `config/*.yaml` 每个键。
- [Generators & field types](https://loco.rs/docs/reference/generators/) — `cargo loco generate` 全部生成器和字段类型写法。
- [Protect a Route with JWT](https://loco.rs/docs/how-to/jwt-auth/) — 后台登录鉴权。
- [Paginate query results](https://loco.rs/docs/how-to/paginate/) — 列表分页。
- [Validate requests](https://loco.rs/docs/how-to/validate-requests/) — 参数校验。
- [Handle errors](https://loco.rs/docs/how-to/handle-errors/) — 错误和 HTTP 状态码。
- [Write request (controller) tests](https://loco.rs/docs/how-to/request-tests/) — 接口测试。

## Wisdom (Communities)
- [GitHub Discussions](https://github.com/loco-rs/loco/discussions) — 官方仓库的讨论区。适合问“这样设计对不对”、DDD 分层怎么放这类判断题，回答可被搜索。
- [官方 Discord](https://discord.gg/fTvyBzwKS8) — 链接来自官方 README。适合快速问答、卡住时求助。

## Gaps
- 官方文档没有专门讲 DDD 分层的页面。DDD 部分只能从 Loco 的默认结构（controllers / models / dtos / workers）出发自己推，需要判断时去 Discussions 问。
