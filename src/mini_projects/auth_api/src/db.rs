use mongodb::{Client, Collection};
use crate::models::user::User;

pub async fn connect_db() -> Collection<User> {
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    client.database("mydb").collection::<User>("users")
}