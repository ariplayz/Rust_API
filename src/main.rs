use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::Json;

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
    Json(users)
}

async fn create_user(Json(user): Json<User>) -> impl Responder {
    Json(user)
}