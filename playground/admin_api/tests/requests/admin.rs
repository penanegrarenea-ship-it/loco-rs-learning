use admin_api::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_get_admins() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/admin").await;
        assert_eq!(res.status_code(), 200);

        // you can assert content like this:
        // assert_eq!(res.text(), "content");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_call_ping() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/admin/ping").await;
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.text(), r#"{"pong":true}"#);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_stats_with_path_and_query() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/admin/stats/users?limit=2").await;
        assert_eq!(res.status_code(), 200);

        let body: serde_json::Value = res.json();
        assert_eq!(body["section"], "users");
        assert_eq!(body["limit"], 2);
        assert_eq!(body["items"].as_array().unwrap().len(), 2);
    })
    .await;
}
