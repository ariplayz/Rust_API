# Swagger/OpenAPI Implementation Guide

## Overview
Your Rust API now has Swagger/OpenAPI documentation integrated using **utoipa** and **utoipa-swagger-ui**.

## What Was Added

### 1. Dependencies (Cargo.toml)
```toml
utoipa = "4"                                           # OpenAPI spec generation
utoipa-swagger-ui = { version = "6", features = ["actix-web"] }  # Swagger UI
actix-files = "0.6"                                    # For file serving
```

### 2. Code Changes (src/main.rs)

#### A. Derive Macros on Structs
```rust
#[derive(Deserialize, Serialize, Clone, utoipa::ToSchema)]
struct User {
    id: i32,
    name: String,
}
```
- `utoipa::ToSchema` generates JSON schema for the User type in OpenAPI spec

#### B. OpenAPI Documentation on Endpoints
```rust
/// Retrieve all users
#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List of users", body = Vec<User>)
    )
)]
async fn get_users() -> impl Responder { ... }

/// Create a new user
#[utoipa::path(
    post,
    path = "/users",
    request_body = User,
    responses(
        (status = 200, description = "User created", body = User)
    )
)]
async fn create_user(web::Json(user): web::Json<User>) -> impl Responder { ... }
```

#### C. OpenAPI Schema Definition
```rust
#[derive(OpenApi)]
#[openapi(
    paths(get_users, create_user),
    components(schemas(User))
)]
struct ApiDoc;
```
- Lists all your API endpoints and data models

#### D. Swagger UI Integration
```rust
.service(
    SwaggerUi::new("/swagger-ui/{_:.*}")
        .url("/api-doc/openapi.json", ApiDoc.openapi())
)
```

## How to Use

### 1. Run Your Application
```bash
cargo run
```

Server starts at `http://0.0.0.0:8080`

### 2. Access Swagger UI
Open your browser and navigate to:
```
http://localhost:8080/swagger-ui/
```

You'll see an interactive API documentation interface where you can:
- View all available endpoints
- See request/response schemas
- Try out API calls directly

### 3. Access Raw OpenAPI JSON
The OpenAPI specification in JSON format is available at:
```
http://localhost:8080/api-doc/openapi.json
```

## Adding New Endpoints

When you add new endpoints, follow this pattern:

```rust
/// Brief description of what the endpoint does
#[utoipa::path(
    post,  // or get, put, delete, patch
    path = "/users/{id}",
    params(
        ("id" = u32, Path, description = "User ID")
    ),
    request_body = User,
    responses(
        (status = 200, description = "Success", body = User),
        (status = 404, description = "User not found")
    )
)]
async fn update_user(
    path: web::Path<u32>,
    web::Json(user): web::Json<User>
) -> impl Responder {
    // Implementation
}
```

Then add the function to the `ApiDoc` struct:
```rust
#[derive(OpenApi)]
#[openapi(
    paths(get_users, create_user, update_user),  // Add here
    components(schemas(User))
)]
struct ApiDoc;
```

## Adding New Data Models

When adding new structures, always include `utoipa::ToSchema`:

```rust
#[derive(Deserialize, Serialize, Clone, utoipa::ToSchema)]
struct Product {
    id: u32,
    name: String,
    price: f64,
}
```

Then add it to the `ApiDoc`:
```rust
#[derive(OpenApi)]
#[openapi(
    paths(get_users, create_user),
    components(schemas(User, Product))  // Add here
)]
struct ApiDoc;
```

## Documentation Attributes

### Common utoipa::path attributes:
- `get`, `post`, `put`, `delete`, `patch` - HTTP method
- `path = "/endpoint/{id}"` - Endpoint path with parameters
- `params(...)` - Path/query parameters
- `request_body = StructName` - Request body type
- `responses(...)` - Possible responses with status codes

### Response format:
```rust
responses(
    (status = 200, description = "Success", body = ReturnType),
    (status = 400, description = "Bad request"),
    (status = 404, description = "Not found")
)
```

## Benefits

✅ **Automatic Documentation** - No manual OpenAPI file needed
✅ **Type-Safe** - Documentation stays in sync with code
✅ **Interactive Testing** - Test endpoints directly from Swagger UI
✅ **API Client Generation** - Generate client libraries from the spec
✅ **Developer Friendly** - Clear, standardized API documentation

## Useful Links

- [utoipa Documentation](https://docs.rs/utoipa/)
- [OpenAPI 3.0 Specification](https://spec.openapis.org/oas/v3.0.3)
- [Swagger UI Guide](https://swagger.io/tools/swagger-ui/)

## Troubleshooting

**Q: Swagger UI not showing my new endpoint?**
A: Make sure you added it to the `paths(...)` in the `#[openapi]` macro on `ApiDoc`.

**Q: Response body not showing in documentation?**
A: Ensure your struct derives `utoipa::ToSchema` and is listed in `components(schemas(...))`.

**Q: Getting compile errors?**
A: Run `cargo check` to see detailed error messages. Make sure all endpoint functions are listed in `ApiDoc`.

