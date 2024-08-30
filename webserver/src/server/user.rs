use crate::database;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct UsersResponse {
    users: Vec<database::models::User>,
}

pub async fn get_users() -> Json<UsersResponse> {
    let connection = &mut database::establish_connection();

    let results = database::get_users(connection);

    Json(UsersResponse { users: results })
}
