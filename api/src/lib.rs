//! API Backend
//! 
//! This is a placeholder for your backend API server.
//! You can implement your API using frameworks like:
//! - Axum
//! - Actix-web  
//! - Rocket
//! - Warp
//!
//! The api-wasm client will connect to this backend.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
}

// Add your API types and functions here
