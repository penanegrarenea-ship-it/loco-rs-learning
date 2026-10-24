# 能独立用非交互参数起 Loco 项目并验证

能在自己机器上独立执行 `loco new --name admin_api --db sqlite --bg async --assets none`、`cargo loco start`，并用 `curl /_ping` 拿到 `{"ok":true}` 来确认服务在跑。隔了一段时间仍能凭记忆写出这四个参数，也记得选了数据库后项目自带 9 个 `/api/auth/*` 路由。下一课不再讲安装和启动，直接讲目录结构和 `src/app.rs`。

Evidence: 2026-10-24 完成第 1 课练习，回忆题答对，自报 `/_ping` 输出和 auth 路由数（9）。
