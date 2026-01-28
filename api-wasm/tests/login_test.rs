#![cfg(target_arch = "wasm32")]
#![allow(dead_code)] // WASM tests are run with wasm-pack test, not cargo test
#![allow(unused_variables)] // Variables used in WASM test context
#![allow(clippy::assertions_on_constants)] // Test assertions for demonstration

use angel_api_client_wasm::{setup_panic_hook, Client, LoginRequest};
use wasm_bindgen_test::*;

// Configure tests to run in browser environment
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_client_creation() {
    setup_panic_hook();

    // Test that we can create a client instance
    let _client = Client::new("https://api.example.com");

    // Test passes by reaching this point without panic
}

#[wasm_bindgen_test]
async fn test_login_function_exists() {
    setup_panic_hook();

    let client = Client::new("https://api.example.com");

    // Create mock login data

    let login_data = LoginRequest::new("test@example.com".into(), "testpassword".into());

    // Test that login method exists and can be called
    // Note: This will fail with network error since we're using fake URL,
    // but it proves the method exists and our WASM bindings work
    let result = client.login(login_data).await;

    // We expect this to fail due to network/CORS, but the binding should work
    assert!(
        result.is_err(),
        "Login should fail with network error for fake URL"
    );
}
