use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod auth;
pub mod user;

use axum::{
  routing::{get, post},
  Json, Router,
};
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
pub async fn run_server() {
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

  let app = Router::new()
    .nest_service("/data", ServeDir::new("data"))
    .route("/", get(root))
    .route("/users", get(user::get_users))
    .route("/user", post(user::create_user))
    .route("/protected", get(auth::protected))
    .route("/authorize", post(auth::authorize));

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
  tracing::debug!("listening on {}", listener.local_addr().unwrap());
  axum::serve(listener, app.layer(TraceLayer::new_for_http()))
    .await
    .unwrap();
}

async fn root() -> Json<serde_json::Value> {
  Json(serde_json::json!({ "main": "this is testing" })) // inline fast way
}
