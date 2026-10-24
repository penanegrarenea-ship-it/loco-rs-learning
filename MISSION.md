# Mission: 用 Loco 写一个 RESTful 后台管理 API

## Why
用 Loco（Rust 的 Rails 风格框架）做一个后台管理用的 RESTful JSON API，尽量按 DDD 分层组织代码，借这个项目把框架摸熟。

## Success looks like
- 能用 `loco new` 起项目、`cargo loco start` 跑起来，并说清每个顶层目录是干什么的
- 能用生成器加资源（model / migration / controller / scaffold），写出符合 REST 语义的 CRUD 接口
- 后台常见需求能落地：JWT 登录、权限校验、分页、参数校验、统一错误返回
- 能说清 Loco 默认分层和 DDD（领域 / 应用 / 基础设施）怎么对应，哪些地方要自己再拆一层
- 每个接口有请求测试（request test）

## Constraints
- Rust 新手阶段已学完；axum 学过参数提取、返回值、测试、带状态路由、中间件
- 没用过 Rails，Rails 惯例需要一两句解释
- 中文，干脆；先命令和结果，再一两句为什么
- 本地默认 SQLite，避免依赖 Postgres

## Out of scope
- 服务端渲染页面（Tera 视图）、前端 SPA
- 部署、Redis 队列、多数据库、多租户（以后需要再说）
