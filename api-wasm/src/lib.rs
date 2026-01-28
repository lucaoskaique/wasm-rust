//! API Client for WebAssembly
//!
//! This crate provides WASM bindings for HTTP API clients, allowing
//! JavaScript applications to interact with APIs directly from
//! the browser using WebAssembly.
//!
//! # Features
//!
//! - Full HTTP client implementation using browser Fetch API
//! - Demo functions for testing WASM capabilities
//! - WASM-optimized with conditional compilation
//! - Type-safe bindings with automatic TypeScript generation
//!
//! # Usage
//!
//! ```javascript
//! import init, { greet, add_numbers } from './pkg/api_wasm.js';
//!
//! // Initialize WASM
//! await init();
//!
//! // Use WASM functions
//! greet("World");  // Logs to console
//! console.log(add_numbers(5, 3));  // 8
//! ```

#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;

// Web API bindings
#[wasm_bindgen]
extern "C" {
    /// Browser alert function
    fn alert(s: &str);

    /// Console log function
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

/// Macro for console logging in WASM environment
///
/// Usage: `console_log!("Hello {}", "world");`
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => ($crate::log(&format_args!($($t)*).to_string()))
}

/// Setup panic hook for better error messages in WASM
///
/// This should be called once during initialization to get better
/// error messages when panics occur in the WASM code.
#[wasm_bindgen]
pub fn setup_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook\#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Demo function: Greet with a name
#[wasm_bindgen]
pub fn greet(name: &str) {
    console_log!("Hello, {}!", name);
}

/// Demo function: Add two numbers
#[wasm_bindgen]
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
