use std::io::Result;

use actix_web::{App, HttpServer};

// importación de módulos
mod user_model;
mod users_route;

/*
    permitir que la función main sea async
*/
#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(users_route::get_users)
        }
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
