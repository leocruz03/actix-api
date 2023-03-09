use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub description: Option<String>
}