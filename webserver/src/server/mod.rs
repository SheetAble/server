pub mod user;
use axum::{
  routing::{get, post},
  Json, Router,
};

#[tokio::main]
pub async fn run_server() {
  // build our application with a single route
  let app = Router::new()
    .route("/", get(root))
    .route("/users", get(user::get_users))
    .route("/user", post(user::create_user));

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}

async fn root() -> Json<serde_json::Value> {
  Json(serde_json::json!({ "main": "this is testing" })) // inline fast way
}
