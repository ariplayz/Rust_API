use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Deserialize, Serialize, Clone, utoipa::ToSchema)]
struct User {
    id: i32,
    name: String,
}

/// Retrieve all users
#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "List of users", body = Vec<User>)
    )
)]
async fn get_users() -> impl Responder {
    let users = vec![User { id: 1, name: "Bob".to_string() }];
    web::Json(users)
}

/// Create a new user
#[utoipa::path(
    post,
    path = "/api/users",
    request_body = User,
    responses(
        (status = 200, description = "User created", body = User)
    )
)]
async fn create_user(web::Json(user): web::Json<User>) -> impl Responder {
    web::Json(user)
}

#[derive(OpenApi)]
#[openapi(
    paths(get_users, create_user),
    components(schemas(User))
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://0.0.0.0:8081");
    println!("Swagger UI available at http://localhost:8081/swagger-ui/");

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .service(web::resource("/users").route(web::get().to(get_users)))
                    .service(web::resource("/users").route(web::post().to(create_user)))
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi())
            )
    })
        .bind("0.0.0.0:8081")?
        .run()
        .await
}
