# Angel API Client WASM - Technical Documentation

## Project Overview

**Angel API Client WASM** is a WebAssembly (WASM) wrapper around the Angel AI API client, specifically designed to provide browser-compatible HTTP client functionality. The project enables JavaScript/TypeScript applications to interact with the Angel AI API directly from web browsers using WebAssembly for performance and type safety.

### Key Characteristics
- **Language**: Rust compiled to WebAssembly
- **Target Environment**: Web browsers and Node.js
- **Primary Purpose**: API client for Angel AI services with browser compatibility
- **Architecture**: Layered design with clean separation of concerns

## Technical Stack

### Core Technologies
- **Rust 2021 Edition** - Main implementation language
- **WebAssembly (WASM)** - Compilation target for browser compatibility
- **wasm-pack** - Build tool for generating WASM packages
- **wasm-bindgen** - Rust-JavaScript interop layer

### Key Dependencies
- `angel-api-client` - Core API client (path dependency)
- `web-sys` - Web API bindings for browser functionality
- `js-sys` - JavaScript type bindings
- `wasm-bindgen-futures` - Async support for WASM
- `async-trait` - Async trait support
- `serde` - Serialization framework

### Build Targets
The project supports multiple WASM build targets:
- **Web** (`--target web`) - Direct browser usage
- **Bundler** (`--target bundler`) - Webpack/bundler integration
- **Node.js** (`--target nodejs`) - Node.js environment
- **No Modules** (`--target no-modules`) - Script tag usage

## Project Architecture

### Directory Structure
```
src/
├── lib.rs              # Public API and WASM entry point
├── client/             # WASM client implementation
│   ├── mod.rs         # Module declarations
│   ├── wasm_client.rs  # Main Client struct with WASM bindings
│   └── types.rs        # JavaScript-compatible type definitions
└── http/               # HTTP client implementation
    ├── mod.rs          # Module declarations
    └── fetch_client.rs # Browser Fetch API HTTP client
```

### Core Components

#### 1. Library Root (`lib.rs`)
- **Purpose**: Main entry point with public WASM API
- **Key Features**:
  - Panic hook setup for better error messages
  - Console logging macros for WASM debugging
  - Public exports for JavaScript consumption
  - WASM-specific conditional compilation

#### 2. Client Module (`client/`)

##### `wasm_client.rs` - Main Client Implementation
- **Core Class**: `Client` - Main API client wrapper
- **Key Methods**:
  - `new(host_url)` - Constructor accepting API base URL
  - `login(login_data)` - Async authentication method
- **Design Pattern**: Wrapper around core `angel-api-client`
- **Error Handling**: Converts Rust errors to JavaScript `JsValue`

##### `types.rs` - Type Definitions
- **`LoginRequest`**: WASM-compatible login credential structure
  - Fields: `email`, `password`
  - JavaScript constructor and getters
  - Conversion to internal API types
- **`LoginResult`**: Authentication result wrapper
  - Contains `Cookie` and `User` data
  - Exposes data via JavaScript getters
  - Automatic conversion from internal types

#### 3. HTTP Module (`http/`)

##### `fetch_client.rs` - Browser HTTP Implementation
- **Core Class**: `FetchClient` - Implements `HttpClient` trait
- **Key Features**:
  - Uses browser's native Fetch API
  - Cross-Origin Resource Sharing (CORS) support
  - Multi-environment support (Window, Worker, global)
  - Automatic cookie handling with synthetic headers
  - Binary data support via `Uint8Array`

**Special Implementation Details**:
- **Environment Detection**: Automatically detects Window vs Worker vs global context
- **Cookie Synthesis**: Adds synthetic `set-cookie` headers for login endpoints since browsers handle cookies automatically
- **Header Extraction**: Converts JavaScript `Headers` object to Rust `HashMap`
- **Error Mapping**: Translates JavaScript errors to `HttpError` enum

## Key Technical Decisions

### 1. Architecture Patterns
- **Trait-Based Design**: Implements existing `HttpClient` trait from core library
- **Wrapper Pattern**: WASM client wraps existing `ApiClient` rather than reimplementing
- **Conditional Compilation**: Uses `#[cfg(target_arch = "wasm32")]` for WASM-specific code

### 2. Memory Management
- **Arc Usage**: Shared ownership for HTTP client instance
- **Clone Strategy**: Strategic cloning for WASM-JavaScript boundary crossings
- **Zero-Copy**: Minimal copying of binary data using `Uint8Array`

### 3. Error Handling Strategy
- **Conversion Pattern**: Rust `Result<T, E>` → JavaScript `Promise<T>`
- **Error Serialization**: Complex errors converted to descriptive strings
- **Panic Hooks**: Optional panic hook for better debugging experience

### 4. Browser Integration
- **Fetch API**: Uses modern browser fetch rather than XMLHttpRequest
- **Cookie Management**: Leverages browser's automatic cookie handling
- **CORS Compatibility**: Explicitly sets CORS mode for cross-origin requests

## API Surface

### JavaScript/TypeScript Interface

```typescript
// Main client class
class Client {
    constructor(host_url: string);
    login(login_data: LoginRequest): Promise<LoginResult>;
}

// Login request structure
class LoginRequest {
    constructor(email: string, password: string);
    readonly email: string;
    readonly password: string;
}

// Login result structure  
class LoginResult {
    readonly cookie: string;
    readonly user: User;
}

// Utility functions
function setup_panic_hook(): void;
```

### Usage Patterns

```javascript
import init, { Client, setup_panic_hook } from './pkg/angel_api_client_wasm.js';

// Initialize WASM module
await init();
setup_panic_hook();

// Create and use client
const client = new Client("https://api.example.com");
const result = await client.login(new LoginRequest("user@email.com", "password"));
```

## Build System

### Build Commands
```bash
# Web target (direct browser usage)
wasm-pack build --target web --out-dir pkg-web

# Bundler target (Webpack/bundler)
wasm-pack build --target bundler --out-dir pkg-bundler

# Node.js target
wasm-pack build --target nodejs --out-dir pkg-nodejs

# Script tag usage
wasm-pack build --target no-modules --out-dir pkg-no-modules
```

### Build Configuration

#### `Cargo.toml` Key Settings
- **Crate Type**: `["cdylib", "rlib"]` for WASM and Rust library
- **Optimization**: Release profile with `opt-level = 3` and `lto = true`
- **Features**: Conditional `console_error_panic_hook` feature

#### Dependencies Strategy
- **Core Library**: Path dependency to `../myangel/angel/api-client`
- **Web APIs**: Extensive `web-sys` features for browser integration
- **Async Support**: `wasm-bindgen-futures` for Promise integration

## Testing Strategy

### Multi-Target Testing
- **TypeScript Tests**: `test.ts` with Node.js execution
- **Browser Tests**: `test.html` for manual browser verification
- **Integration Tests**: Real API calls to verify functionality

### Test Infrastructure
```json
{
  "scripts": {
    "test": "npm run build:web && node --experimental-strip-types test.ts",
    "build:all": "npm run build:nodejs && npm run build:web && npm run build:bundler"
  }
}
```

## Performance Characteristics

### Optimization Strategies
- **Release Builds**: Aggressive optimization with LTO (Link Time Optimization)
- **Size Optimization**: WASM-specific `opt-level = "s"` for smaller binaries
- **Minimal Dependencies**: Careful feature selection to reduce bundle size

### Runtime Performance
- **Zero-Copy Operations**: Direct buffer sharing between WASM and JavaScript
- **Async Efficiency**: Native Promise integration without blocking
- **Memory Efficiency**: Strategic use of `Arc` and minimal cloning

## Security Considerations

### Browser Security Model
- **Same-Origin Policy**: Respects browser security boundaries
- **CORS Integration**: Proper cross-origin request handling
- **Cookie Security**: Leverages browser's secure cookie management

### Input Validation
- **Type Safety**: Rust's type system prevents many common vulnerabilities
- **Error Boundaries**: Controlled error propagation to JavaScript
- **Memory Safety**: Rust's memory model prevents buffer overflows

## Integration Patterns

### Framework Integration
```typescript
// Next.js/React example
import { loadWasm, wasmLogin } from '@/lib/wasmLoader';

const handleLogin = async (email: string, password: string) => {
  const result = await wasmLogin(email, password);
  if (result.success) {
    // Handle successful login
  }
};
```

### Module Systems
- **ES Modules**: Native ES6 module support
- **CommonJS**: Node.js compatibility
- **UMD**: Universal module definition for various environments

## Debugging and Development

### Debugging Features
- **Console Logging**: `console_log!()` macro for WASM debugging
- **Panic Hooks**: Better error messages via `console_error_panic_hook`
- **Source Maps**: Optional source map generation for debugging

### Development Workflow
1. **Rust Development**: Standard `cargo check` and `cargo test`
2. **WASM Build**: `wasm-pack build` for target generation
3. **JavaScript Testing**: Node.js or browser testing of generated packages
4. **Integration Testing**: End-to-end testing with real API endpoints

## Future Considerations

### Extensibility Points
- **SSE Support**: Server-Sent Events implementation (currently TODO)
- **Additional APIs**: Easy extension for new Angel AI API endpoints
- **Performance Monitoring**: Potential telemetry integration

### Maintenance Strategy
- **Dependency Updates**: Regular updates of `angel-api-client` core
- **WASM Ecosystem**: Following WebAssembly and wasm-bindgen evolution
- **Browser Compatibility**: Monitoring browser API changes

## Common Challenges and Solutions

### CORS Issues
**Problem**: Cross-origin requests blocked by browsers
**Solution**: Explicit CORS mode setting and server-side CORS headers

### Cookie Management
**Problem**: Browser automatically handles cookies, but API client needs visibility
**Solution**: Synthetic `set-cookie` headers for specific endpoints

### Error Propagation
**Problem**: Rust error types don't translate well to JavaScript
**Solution**: String serialization of errors with descriptive messages

### Build Complexity
**Problem**: Multiple build targets with different requirements
**Solution**: Comprehensive npm scripts for automated multi-target builds

This documentation provides a comprehensive technical overview suitable for sharing with other AI systems or technical team members for analysis, extension, or integration planning.