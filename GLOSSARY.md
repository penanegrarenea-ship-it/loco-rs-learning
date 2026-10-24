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
