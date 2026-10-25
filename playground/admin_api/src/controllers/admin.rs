#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[debug_handler]
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn ping(State(_ctx): State<AppContext>) -> Result<Response> {
    format::json(serde_json::json!({ "pong": true }))
}

/// `?limit=2` —— 可选的查询参数
#[derive(Debug, Deserialize)]
pub struct StatsQuery {
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct StatItem {
    pub name: &'static str,
    pub value: u64,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub section: String,
    pub limit: usize,
    pub items: Vec<StatItem>,
}

/// GET /api/admin/stats/{section}?limit=2 —— 先返回写死的数据，下一课换成数据库
#[debug_handler]
pub async fn stats(
    Path(section): Path<String>,
    Query(q): Query<StatsQuery>,
) -> Result<Response> {
    let all = [
        ("total", 42),
        ("active_today", 7),
        ("pending_review", 3),
    ];
    let limit = q.limit.unwrap_or(all.len()).min(all.len());
    let items = all
        .iter()
        .take(limit)
        .map(|(name, value)| StatItem { name, value: *value })
        .collect();
    format::json(StatsResponse {
        section,
        limit,
        items,
    })
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/admin/")
        .add("/", get(index))
        .add("ping", get(ping))
        .add("stats/{section}", get(stats))
}
