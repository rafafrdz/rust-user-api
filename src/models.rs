use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UserEmailRequest {
    pub email: String,
}
