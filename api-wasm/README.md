# API WASM - WebAssembly HTTP Client

A high-performance WebAssembly HTTP client for browser applications, written in Rust.

## Features

- 🌐 **Fetch API Integration**: Native browser HTTP requests
- ⚡ **High Performance**: Compiled to WebAssembly for speed
- 🔒 **Type Safe**: Rust types with automatic TypeScript bindings
- 🍪 **Cookie Management**: Automatic cookie handling
- 📦 **Zero Dependencies**: Minimal JavaScript runtime
- 🎯 **Async/Await**: Modern JavaScript async API

## Installation

### Via NPM (when published)

```bash
npm install api-wasm
```

### Build from Source

```bash
# Install wasm-pack
cargo install wasm-pack

# Build the package
wasm-pack build --target web --out-dir pkg

# For production
wasm-pack build --target web --release
```

## Quick Start

### HTML + JavaScript

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>API WASM Demo</title>
</head>
<body>
    <script type="module">
        import init, { add_numbers, greet } from './pkg/api_wasm.js';
        
        async function main() {
            // Initialize WASM
            await init();
            
            // Call WASM functions
            console.log(add_numbers(5, 3));  // 8
            greet("World");  // Logs to console
        }
        
        main();
    </script>
</body>
</html>
```

### TypeScript

```typescript
import init, { greet, add_numbers } from 'api-wasm';

async function main() {
    await init();
    
    greet("TypeScript");
    const result = add_numbers(10, 20);
    console.log('Result:', result);
}

main();
```

## API Reference

### Demo Functions

```typescript
// Simple greeting function
function greet(name: string): void

// Add two numbers
function add_numbers(a: number, b: number): number
```

## Building

### Development Build

```bash
wasm-pack build --target web --dev
```

### Production Build

```bash
wasm-pack build --target web --release
```

### Different Targets

```bash
# For bundlers (webpack, rollup, etc.)
wasm-pack build --target bundler

# For Node.js
wasm-pack build --target nodejs

# No modules (UMD-like)
wasm-pack build --target no-modules
```

## Testing

```bash
# Run tests in headless browser
wasm-pack test --headless --firefox
wasm-pack test --headless --chrome

# Run tests with browser UI
wasm-pack test --firefox
```

## File Structure

```
api-wasm/
├── src/
│   ├── lib.rs              # Main entry point
│   ├── client/             # HTTP client implementation
│   │   ├── mod.rs
│   │   ├── wasm_client.rs  # WASM client with bindings
│   │   └── types.rs        # TypeScript-compatible types
│   ├── http/               # HTTP layer
│   │   ├── mod.rs
│   │   └── fetch_client.rs # Fetch API implementation
│   ├── utils/              # Utilities
│   │   ├── mod.rs
│   │   └── logging.rs      # Console logging
│   └── demo/               # Example functions
│       └── mod.rs
├── examples/               # Usage examples
├── tests/                  # WASM tests
├── Cargo.toml             # Rust dependencies
├── wasm-pack.toml         # WASM build config
└── README.md              # This file
```

## Development

### Watch Mode

```bash
# Requires cargo-watch
cargo install cargo-watch

# Watch and rebuild
cargo watch -i pkg -s "wasm-pack build --target web"
```

### Debugging

Enable panic hooks for better error messages:

```rust
use console_error_panic_hook;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}
```

View logs in browser console:

```rust
use web_sys::console;

console::log_1(&"Debug message".into());
```

## Performance Optimization

### Build Size

Reduce WASM size:

```toml
# Cargo.toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Enable link-time optimization
codegen-units = 1   # Better optimization
panic = "abort"     # Smaller binary
```

Post-processing:

```bash
# Install wasm-opt (from binaryen)
wasm-opt -Oz -o output.wasm input.wasm
```

### Runtime Performance

- Use `web-sys` directly for hot paths
- Minimize string allocations
- Batch operations when possible
- Use TypedArrays for bulk data

## Browser Compatibility

Works in all modern browsers with WebAssembly support:

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## License

MIT License - see LICENSE file for details

## Contributing

This is a demo/template project. Contributions and improvements welcome!

## Resources

- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [web-sys Documentation](https://rustwasm.github.io/wasm-bindgen/web-sys/)
- [MDN WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly)
