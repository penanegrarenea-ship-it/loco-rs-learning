use admin_api::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

use super::prepare_data;

#[tokio::test]
#[serial]
async fn list_without_token_is_401() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/categories").await;
        assert_eq!(res.status_code(), 401);

        let body: serde_json::Value = res.json();
        assert_eq!(body["error"], "unauthorized");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn list_with_token_is_200() {
    request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (key, value) = prepare_data::auth_header(&user.token);

        let res = request.get("/api/categories").add_header(key, value).await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn create_rejects_empty_name_with_field_errors() {
    request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (key, value) = prepare_data::auth_header(&user.token);

        let res = request
            .post("/api/categories")
            .add_header(key, value)
            .json(&serde_json::json!({ "name": "" }))
            .await;
        assert_eq!(res.status_code(), 400);

        let body: serde_json::Value = res.json();
        assert_eq!(body["errors"]["name"][0]["code"], "length");
        assert_eq!(
            body["errors"]["name"][0]["message"],
            "name must be 1-50 characters"
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn create_rejects_duplicate_name_with_409() {
    request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (key, value) = prepare_data::auth_header(&user.token);

        let first = request
            .post("/api/categories")
            .add_header(key.clone(), value.clone())
            .json(&serde_json::json!({ "name": "Books" }))
            .await;
        assert_eq!(first.status_code(), 201);

        let second = request
            .post("/api/categories")
            .add_header(key, value)
            .json(&serde_json::json!({ "name": "Books" }))
            .await;
        assert_eq!(second.status_code(), 409);
        let body: serde_json::Value = second.json();
        assert_eq!(body["error"], "conflict");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn remove_with_token_deletes() {
    request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (key, value) = prepare_data::auth_header(&user.token);

        let created = request
            .post("/api/categories")
            .add_header(key.clone(), value.clone())
            .json(&serde_json::json!({ "name": "Temp" }))
            .await;
        assert_eq!(created.status_code(), 201);
        let id = created.json::<serde_json::Value>()["id"].clone();

        let removed = request
            .delete(&format!("/api/categories/{id}"))
            .add_header(key.clone(), value.clone())
            .await;
        assert_eq!(removed.status_code(), 204);

        let again = request
            .get(&format!("/api/categories/{id}"))
            .add_header(key, value)
            .await;
        assert_eq!(again.status_code(), 404);
    })
    .await;
}
