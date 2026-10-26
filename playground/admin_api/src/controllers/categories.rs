#![allow(clippy::unused_async)]
use axum::http::StatusCode;
use loco_rs::prelude::*;

use crate::{
    dtos::{
        categories::{CategoryDto, CreateCategory, UpdateCategory},
        common::{ApiError, Page},
    },
    models::{
        _entities::categories::{ActiveModel, Entity},
        users,
    },
};

/// Build a 404 response shaped as the [`ApiError`] envelope.
fn not_found(message: &str) -> Response {
    let error = ApiError {
        code: "not_found".to_string(),
        message: message.to_string(),
        details: None,
    };
    (StatusCode::NOT_FOUND, Json(error)).into_response()
}

/// `?q=book` —— 可选的名称关键字
#[derive(Debug, serde::Deserialize)]
pub struct ListFilter {
    pub q: Option<String>,
}

#[debug_handler]
async fn list(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Query(pagination): Query<query::PaginationQuery>,
    Query(filter): Query<ListFilter>,
) -> Result<Json<Page<CategoryDto>>> {
    let res = query::paginate(
        &ctx.db,
        Entity::search(filter.q.as_deref()),
        None,
        &pagination,
    )
    .await?;

    Ok(Json(Page::from_query(res)))
}

#[debug_handler]
async fn get_one(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Response> {
    let Some(model) = Entity::find_by_id(id).one(&ctx.db).await? else {
        return Ok(not_found("category not found"));
    };
    Ok(Json(CategoryDto::from(model)).into_response())
}

#[debug_handler]
async fn create(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    JsonValidateWithMessage(params): JsonValidateWithMessage<CreateCategory>,
) -> Result<Response> {
    Entity::ensure_name_unique(&ctx.db, &params.name, None).await?;
    let item = ActiveModel {
        name: Set(params.name),
        description: Set(params.description),
        ..Default::default()
    };
    let item = item.insert(&ctx.db).await?;

    Ok((StatusCode::CREATED, Json(CategoryDto::from(item))).into_response())
}

#[debug_handler]
async fn update(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<i64>,
    JsonValidateWithMessage(params): JsonValidateWithMessage<UpdateCategory>,
) -> Result<Response> {
    let Some(model) = Entity::find_by_id(id).one(&ctx.db).await? else {
        return Ok(not_found("category not found"));
    };
    Entity::ensure_name_unique(&ctx.db, &params.name, Some(id)).await?;

    let mut item = model.into_active_model();
    item.name = Set(params.name);
    item.description = Set(params.description);
    let item = item.update(&ctx.db).await?;

    Ok(Json(CategoryDto::from(item)).into_response())
}

#[debug_handler]
async fn remove(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Response> {
    // token 里只有 pid；要知道“是谁”，用 pid 查 users 表
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let Some(model) = Entity::find_by_id(id).one(&ctx.db).await? else {
        return Ok(not_found("category not found"));
    };
    tracing::info!(category_id = id, by = %user.email, "category deleted");

    model.into_active_model().delete(&ctx.db).await?;

    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/categories")
        .add("/", get(list))
        .add("/", post(create))
        .add("/{id}", get(get_one))
        .add("/{id}", put(update))
        .add("/{id}", delete(remove))
}
