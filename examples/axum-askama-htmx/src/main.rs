use askama::*;

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
    count: u64,
}

#[derive(Template)]
#[template(path = "counter_component.html")]
struct CounterTemplate {
    count: u64,
}

use axum::{extract::Path, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { HelloTemplate { count: 0 } }))
        .route(
            "/counter/:count",
            get(|Path(count): Path<u64>| async move { CounterTemplate { count } }),
        );

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
