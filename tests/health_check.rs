// tests/health_check.rs
use axum::{
    body::{Body, to_bytes}, // <-- add to_bytes from axum::body
    http::{Request, StatusCode},
};
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn health_returns_ok_true() {
    let app = axum_template::app(); // crate name with '-' → '_'

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/health")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);

    // axum 0.8 to_bytes requires a body size limit; health response is tiny
    let bytes = to_bytes(response.into_body(), 1024)
        .await
        .expect("read body");
    let json: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(json, serde_json::json!({ "ok": true }));
}
