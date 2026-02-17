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
                web::scope("/api/v1")
                    .service(web::resource("/users").to(|| {
                        get_users()
                    }))
                    .service(web::resource("/users").route(web::post().to(create_user)))
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn get_users() -> impl Responder {
    let users = vec![User { id: 1, name: "Alice".to_string() }];
    web::Json(users)
}

async fn create_user(web::Json(user): web::Json<User>) -> impl Responder {
    web::Json(user)
}