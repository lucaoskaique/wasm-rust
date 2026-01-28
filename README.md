# WASM Rust - Full-Stack WebAssembly Project

A complete full-stack Rust project demonstrating WebAssembly (WASM) in the browser with a Rust backend API.

## Project Structure

```
wasm-rust/
├── api-wasm/          # WebAssembly client for browser
│   ├── src/           # Rust source compiled to WASM
│   ├── examples/      # Usage examples
│   └── tests/         # WASM tests
├── api/               # Backend API server (placeholder)
│   └── src/           # API server implementation
└── README.md          # This file
```

## Components

### 🌐 api-wasm (WebAssembly Client)

A high-performance WASM client library that runs in the browser. Features:

- **Browser-Native**: Uses Fetch API for HTTP requests
- **Type-Safe**: Rust types with TypeScript bindings
- **Zero-Copy**: Efficient data transfer between JS and WASM
- **Async/Await**: Modern async JavaScript API
- **Cookie Support**: Automatic cookie management

**Build:**
```bash
cd api-wasm
wasm-pack build --target web --out-dir pkg
```

### 🚀 api (Backend Server)

A placeholder for your Rust backend API. Choose your framework:

- **Axum**: Modern, ergonomic web framework
- **Actix-web**: High-performance actor-based framework
- **Rocket**: Type-safe, async framework
- **Warp**: Lightweight, composable framework

**Example (Axum):**
```bash
cd api
# Add to Cargo.toml:
# axum = "0.7"
# tokio = { version = "1", features = ["full"] }
```

## Getting Started

### Prerequisites

- **Rust** (latest stable): https://rustup.rs/
- **wasm-pack**: `cargo install wasm-pack`
- **Node.js** (for testing): https://nodejs.org/

### Quick Start

1. **Build the WASM client:**
   ```bash
   cd api-wasm
   wasm-pack build --target web
   ```

2. **Create a simple HTML page:**
   ```html
   <!DOCTYPE html>
   <html>
   <head>
       <meta charset="utf-8">
       <title>WASM Rust Demo</title>
   </head>
   <body>
       <h1>WASM Rust Client</h1>
       <script type="module">
           import init, { greet } from './api-wasm/pkg/api_wasm.js';
           
           async function run() {
               await init();
               greet("World");
           }
           
           run();
       </script>
   </body>
   </html>
   ```

3. **Serve with a local server:**
   ```bash
   # Python
   python3 -m http.server 8000
   
   # Node.js
   npx http-server -p 8000
   ```

4. **Open in browser:** http://localhost:8000

## Development Workflow

### WASM Client Development

```bash
cd api-wasm

# Development build
wasm-pack build --target web --dev

# Production build
wasm-pack build --target web --release

# Run tests
wasm-pack test --headless --firefox

# Watch mode (requires cargo-watch)
cargo watch -i pkg -s "wasm-pack build --target web"
```

### API Server Development

```bash
cd api

# Run server (example with cargo-watch)
cargo watch -x run

# Run tests
cargo test

# Build release
cargo build --release
```

## Architecture

### Data Flow

```
┌─────────────┐     HTTP/JSON     ┌─────────────┐
│   Browser   │ ←───────────────→ │  API Server │
│  (WASM+JS)  │   Fetch API       │    (Rust)   │
└─────────────┘                   └─────────────┘
      │
      └─→ api-wasm.wasm (compiled Rust)
```

### WASM Client Features

- **HTTP Client**: Fetch API wrapper
- **Serialization**: JSON via serde
- **Error Handling**: Comprehensive error types
- **Logging**: Browser console integration
- **Type Bindings**: Automatic TypeScript definitions

## Example Usage

### TypeScript/JavaScript

```typescript
import init, { ApiClient } from './pkg/api_wasm.js';

async function main() {
    // Initialize WASM
    await init();
    
    // Create client
    const client = new ApiClient('https://api.example.com');
    
    // Make authenticated request
    const response = await client.login('user@example.com', 'password');
    console.log('User:', response);
}

main();
```

### Rust (WASM)

```rust
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
}

#[wasm_bindgen]
pub async fn fetch_user(id: String) -> Result<JsValue, JsValue> {
    let url = format!("https://api.example.com/users/{}", id);
    let user: User = http_get(&url).await?;
    Ok(serde_wasm_bindgen::to_value(&user)?)
}
```

## Building for Different Targets

```bash
# Web (ES modules)
wasm-pack build --target web

# Bundler (webpack, rollup)
wasm-pack build --target bundler

# Node.js
wasm-pack build --target nodejs

# No modules
wasm-pack build --target no-modules
```

## Testing

### WASM Tests

```bash
cd api-wasm

# Headless browser tests
wasm-pack test --headless --chrome
wasm-pack test --headless --firefox

# With browser UI
wasm-pack test --chrome
```

### Integration Tests

```bash
# Run both WASM and API tests
cargo test --workspace
```

## Performance Tips

1. **Enable optimizations:**
   - Use `--release` flag
   - Enable LTO in Cargo.toml
   - Use `opt-level = "z"` for size

2. **Reduce WASM size:**
   - Run `wasm-opt` (from binaryen)
   - Use `wasm-snip` to remove unused code
   - Enable `panic = "abort"`

3. **Lazy loading:**
   - Split large WASM into modules
   - Load on-demand

## Deployment

### WASM Client

1. Build production bundle:
   ```bash
   wasm-pack build --target web --release
   ```

2. Deploy `pkg/` directory to CDN or static host

3. Set correct MIME types:
   - `.wasm` → `application/wasm`
   - `.js` → `application/javascript`

### API Server

1. Build release binary:
   ```bash
   cd api
   cargo build --release
   ```

2. Deploy binary to server

3. Configure CORS for WASM client

## Resources

- **Rust and WebAssembly Book**: https://rustwasm.github.io/docs/book/
- **wasm-bindgen Guide**: https://rustwasm.github.io/wasm-bindgen/
- **MDN Web Docs**: https://developer.mozilla.org/en-US/docs/WebAssembly

## License

MIT License - see LICENSE file for details

## Contributing

Contributions welcome! This is a template/demo project showcasing Rust WASM development patterns.
