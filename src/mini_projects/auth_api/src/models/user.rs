use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct RegisterUser {

    #[validate(length(min=2, max=20, message="Username must be in between 2-20 char"))]
    pub username: String,

    #[validate(email(message="Invalid input"))]
    pub email: String,

    #[validate(length(min=8, message="Password must be 8 of 8 char"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginUser {

    #[validate(email(message="Invalid user input"))]
    pub email: String,

    #[validate(length(min=8, message="Invalid user input"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatedUser {

    #[validate(length(min=2, max=20, message="Username must be in between 2-20 char"))]
    pub username: String,

    #[validate(length(min=8, message="Password must be 8 char long"))]
    pub password: String,
}


