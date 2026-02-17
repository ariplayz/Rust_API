use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct User {
    id: i32,
    name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("")
                    .service(web::resource("/users").route(web::get().to(get_users)))
                    .service(web::resource("/users").route(web::post().to(create_user)))
            )
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

async fn get_users() -> impl Responder {
    let users = vec![User { id: 1, name: "Bob".to_string() }];
    web::Json(users)
}

async fn create_user(web::Json(user): web::Json<User>) -> impl Responder {
    web::Json(user)
}