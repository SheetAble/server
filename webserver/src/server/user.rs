use crate::database::{self, schema::users::email};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UsersResponse {
  users: Vec<database::models::User>,
}

pub async fn get_users() -> Json<UsersResponse> {
  let connection = &mut database::establish_connection();

  let results = database::get_users(connection);

  Json(UsersResponse { users: results })
}

#[derive(Serialize)]
pub struct UserResponse {
  user: database::models::User,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
  email: String,
  password: String,
  library: String,
}

pub async fn create_user(Json(payload): Json<CreateUserRequest>) -> Json<UserResponse> {
  let connection = &mut database::establish_connection();

  println!("Creating user with email: {}", payload.email);

  let user = database::create_user(
    connection,
    &payload.email,
    &payload.password,
    &payload.library,
  );

  Json(UserResponse { user })
}
