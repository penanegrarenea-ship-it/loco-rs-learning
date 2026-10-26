use admin_api::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn create_rejects_empty_name_with_field_errors() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request
            .post("/api/categories")
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
    request::<App, _, _>(|request, _ctx| async move {
        let first = request
            .post("/api/categories")
            .json(&serde_json::json!({ "name": "Books" }))
            .await;
        assert_eq!(first.status_code(), 201);

        let second = request
            .post("/api/categories")
            .json(&serde_json::json!({ "name": "Books" }))
            .await;
        assert_eq!(second.status_code(), 409);
        let body: serde_json::Value = second.json();
        assert_eq!(body["error"], "conflict");
    })
    .await;
}
