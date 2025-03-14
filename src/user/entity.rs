use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub active: Option<bool>,
    pub email: String,
    pub encrypted_password: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUserPayload {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl UserResponse {
    pub fn from(user: &User) -> UserResponse {
        UserResponse {
            id: user.id.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
            active: user
                .active
                .clone()
                .expect("User should have an active status"),
            created_at: user
                .created_at
                .expect("User should have a created_at field")
                .to_string(),
            updated_at: user
                .updated_at
                .expect("User should have an updated_at field")
                .to_string(),
        }
    }
}
