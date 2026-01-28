#!/bin/bash

echo "🧪 Verifying WASM build..."

# Check if pkg directory exists
if [ ! -d "pkg" ]; then
    echo "❌ pkg directory not found. Run 'wasm-pack build --target web' first."
    exit 1
fi

# Check required files
files=(
    "package.json"
    "angel_api_client_wasm.js"
    "angel_api_client_wasm.d.ts"
    "angel_api_client_wasm_bg.wasm"
)

for file in "${files[@]}"; do
    if [ ! -f "pkg/$file" ]; then
        echo "❌ Missing file: pkg/$file"
        exit 1
    else
        echo "✅ Found: pkg/$file"
    fi
done

# Check TypeScript definitions contain our exports
if grep -q "class Client" pkg/angel_api_client_wasm.d.ts; then
    echo "✅ Client class exported in TypeScript definitions"
else
    echo "❌ Client class not found in TypeScript definitions"
    exit 1
fi

# Check package.json is valid JSON
if python3 -m json.tool pkg/package.json > /dev/null 2>&1; then
    echo "✅ package.json is valid JSON"
else
    echo "❌ package.json is not valid JSON"
    exit 1
fi

# Check WASM file size (should be reasonable)
wasm_size=$(stat -c%s pkg/angel_api_client_wasm_bg.wasm)
if [ $wasm_size -gt 100000 ] && [ $wasm_size -lt 10000000 ]; then
    echo "✅ WASM file size reasonable: $wasm_size bytes"
else
    echo "⚠️  WASM file size unusual: $wasm_size bytes"
fi

echo ""
echo "🎉 All verifications passed! WASM package is ready to use."
echo ""
echo "📦 Package contents:"
echo "   - TypeScript definitions: pkg/angel_api_client_wasm.d.ts"
echo "   - JavaScript bindings: pkg/angel_api_client_wasm.js"
echo "   - WebAssembly binary: pkg/angel_api_client_wasm_bg.wasm"
echo "   - Package metadata: pkg/package.json"
echo ""
echo "💡 Usage example:"
echo "   import init, { Client } from './pkg/angel_api_client_wasm.js';"
echo "   await init();"
echo "   const client = new Client('https://dev-api.angelq.ai');"
echo "   const result = await client.login({ email: 'user@example.com', password: 'password' });"
echo "   console.log('Login result:', JSON.parse(result));"
