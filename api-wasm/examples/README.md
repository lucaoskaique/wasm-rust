# Usage Examples

This directory contains example code showing how to use the API WASM client in different environments.

## Basic Example (HTML + ES Modules)

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>API WASM Demo</title>
</head>
<body>
    <h1>WASM Demo</h1>
    <button id="testBtn">Test WASM Functions</button>
    <pre id="output"></pre>

    <script type="module">
        import init, { greet, add_numbers } from './pkg/api_wasm.js';
        
        async function main() {
            // Initialize WASM
            await init();
            
            document.getElementById('testBtn').addEventListener('click', () => {
                const output = document.getElementById('output');
                
                // Call WASM functions
                greet("WASM World");
                const sum = add_numbers(42, 8);
                
                output.textContent = `Sum: ${sum}`;
            });
        }
        
        main();
    </script>
</body>
</html>
```

## TypeScript Example

```typescript
import init, { greet, add_numbers } from 'api-wasm';

async function main() {
    // Initialize WASM module
    await init();
    
    // Use demo functions
    greet("TypeScript");
    
    const result = add_numbers(10, 20);
    console.log('Result:', result);  // 30
}

main();
```

## Node.js Example

```javascript
const init = require('./pkg/api_wasm.js');

async function main() {
    // Initialize WASM
    await init();
    
    const { greet, add_numbers } = require('./pkg/api_wasm.js');
    
    greet("Node.js");
    const sum = add_numbers(100, 200);
    console.log('Sum:', sum);  // 300
}

main().catch(console.error);
```

## React Example

```typescript
import { useEffect, useState } from 'react';
import init, { greet, add_numbers } from 'api-wasm';

function WasmComponent() {
    const [wasmReady, setWasmReady] = useState(false);
    const [result, setResult] = useState<number | null>(null);

    useEffect(() => {
        init().then(() => {
            setWasmReady(true);
            greet("React");
        });
    }, []);

    const handleCalculate = () => {
        if (wasmReady) {
            const sum = add_numbers(15, 25);
            setResult(sum);
        }
    };

    return (
        <div>
            <h2>WASM Demo</h2>
            {wasmReady ? (
                <>
                    <button onClick={handleCalculate}>Calculate</button>
                    {result !== null && <p>Result: {result}</p>}
                </>
            ) : (
                <p>Loading WASM...</p>
            )}
        </div>
    );
}
```

## Vite Configuration

```javascript
// vite.config.js
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';

export default defineConfig({
    plugins: [wasm()],
    build: {
        target: 'esnext',
    },
});
```

## Webpack Configuration

```javascript
// webpack.config.js
module.exports = {
    experiments: {
        asyncWebAssembly: true,
    },
    module: {
        rules: [
            {
                test: /\.wasm$/,
                type: 'webassembly/async',
            },
        ],
    },
};
```

## Bundle Targets

Build for different environments:

```bash
# Web (default)
wasm-pack build --target web

# Bundler (webpack, rollup, vite)
wasm-pack build --target bundler

# Node.js
wasm-pack build --target nodejs

# No modules (UMD-like)
wasm-pack build --target no-modules
```

## Performance Tips

- Initialize WASM once at app startup
- Reuse WASM functions instead of recreating
- Use TypedArrays for bulk data transfer
- Minimize string conversions between JS and Rust
- Consider using `wasm-opt` for production builds

## Browser Compatibility

Works in all modern browsers with WebAssembly support:
- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+
