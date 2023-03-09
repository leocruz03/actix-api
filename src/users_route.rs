use actix_web::{get, web, Responder};

use crate::user_model::{self, User};

#[get("/users")]
pub async fn get_users() -> impl Responder {
    let mut users: Vec<user_model::User> = Vec::new();

    users.push(
        User {
            id: "01".to_string(),
            name: "leonardo".to_string(),
            username: "delacruz".to_string(),
            email: "email@email.com".to_string(),
            password: "123456789".to_string(),
            description: None
        }
    );
    
    users.push(
        User {
            id: "01".to_string(),
            name: "john".to_string(),
            username: "doejohn".to_string(),
            email: "johndoe@email.com".to_string(),
            password: "987654321".to_string(),
            description: Some("this is description of john".to_string()),
        }
    );

    users.push(
        User {
            id: "01".to_string(),
            name: "leonardo".to_string(),
            username: "delacruz".to_string(),
            email: "email@email.com".to_string(),
            password: "123456789".to_string(),
            description: None
        }
    );

    return web::Json(users);
}