use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub fname: String,
    pub lname: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub image: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewUser {
    pub fname: String,
    pub lname: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUser {
    #[validate(url)]
    pub image: Option<String>,
    #[validate(phone)]
    pub phone_number: Option<String>,
}
