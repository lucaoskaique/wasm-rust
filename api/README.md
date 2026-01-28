# API - Backend Server (Placeholder)

A placeholder backend API server for use with the WASM client.

## Overview

This is a template/placeholder directory for building a backend API server in Rust. The `api-wasm` client can connect to any HTTP API - this directory provides a starting point for your backend implementation.

## Getting Started

### Choose Your Framework

Popular Rust web frameworks to consider:

#### Axum (Recommended)
- Modern, ergonomic, built on Tokio
- Great async support
- Easy integration with Tower middleware

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower = "0.4"
serde = { version = "1.0", features = ["derive"] }
```

#### Actix Web
- High performance
- Mature ecosystem
- Actor-based architecture

```toml
[dependencies]
actix-web = "4"
serde = { version = "1.0", features = ["derive"] }
```

#### Rocket
- Developer-friendly
- Type-safe routing
- Great documentation

```toml
[dependencies]
rocket = "0.5"
serde = { version = "1.0", features = ["derive"] }
```

## Example: Basic Axum Server

### Update Cargo.toml

```toml
[package]
name = "api"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
```

### Update src/lib.rs or create src/main.rs

```rust
use axum::{
    routing::{get, post},
    Json, Router,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

async fn health_check() -> &'static str {
    "OK"
}

async fn get_users() -> Json<ApiResponse<Vec<User>>> {
    // TODO: Replace with database query
    let users = vec![
        User {
            id: "1".to_string(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: "2".to_string(),
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ];

    Json(ApiResponse::success(users))
}

async fn create_user(
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<ApiResponse<User>>) {
    // TODO: Validate and save to database
    let user = User {
        id: uuid::Uuid::new_v4().to_string(),
        name: payload.name,
        email: payload.email,
    };

    (StatusCode::CREATED, Json(ApiResponse::success(user)))
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .layer(CorsLayer::permissive()); // Configure CORS for WASM client

    // Run server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## Running the Server

```bash
# Development
cargo run

# With auto-reload (requires cargo-watch)
cargo watch -x run

# Production build
cargo build --release
./target/release/api
```

## Database Integration

### With SQLx (PostgreSQL)

```toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres"] }
```

### With Diesel (PostgreSQL)

```toml
[dependencies]
diesel = { version = "2.1", features = ["postgres", "r2d2"] }
```

### With MongoDB

```toml
[dependencies]
mongodb = "2.8"
```

## Project Structure

```
api/
├── src/
│   ├── main.rs           # Server entry point
│   ├── routes/           # Route handlers
│   │   ├── mod.rs
│   │   ├── users.rs
│   │   └── auth.rs
│   ├── models/           # Data models
│   │   ├── mod.rs
│   │   └── user.rs
│   ├── db/               # Database layer
│   │   ├── mod.rs
│   │   └── pool.rs
│   └── middleware/       # Custom middleware
│       └── mod.rs
├── migrations/           # Database migrations
├── Cargo.toml
└── README.md
```

## Authentication

Example JWT authentication with `jsonwebtoken`:

```toml
[dependencies]
jsonwebtoken = "9"
```

```rust
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}
```

## Environment Configuration

Use `.env` file with `dotenvy`:

```toml
[dependencies]
dotenvy = "0.15"
```

```rust
use dotenvy::dotenv;

fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
}
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        let app = Router::new().route("/health", get(health_check));

        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
```

Run tests:

```bash
cargo test
```

## Deployment

### Docker

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/api /usr/local/bin/api
CMD ["api"]
```

### Shuttle.rs

```toml
[dependencies]
shuttle-runtime = "0.38"
shuttle-axum = "0.38"
```

### Fly.io, Railway, Render

All support Rust applications with simple deployment.

## Resources

- [Axum Documentation](https://docs.rs/axum)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [SQLx Documentation](https://docs.rs/sqlx)
- [Diesel Getting Started](https://diesel.rs/guides/getting-started)

## License

MIT License - see LICENSE file for details
