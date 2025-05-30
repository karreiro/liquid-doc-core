#!/bin/bash

echo "🚀 Running WebAssembly LiquidDoc Parser Project"
echo ""

echo "1️⃣  Installing Rust tools..."
echo "----------------------------------------"
cargo insta --version || cargo install cargo-insta

echo ""
echo "2️⃣  Installing Ruby dependencies..."
echo "----------------------------------------"
if command -v bundle &> /dev/null; then
    bundle install
else
    echo "📦 Install bundler to run ruby app" 
fi

echo ""
echo "3️⃣  Building WebAssembly version..."
echo "----------------------------------------"
wasm-pack build wasm --target web --out-dir web/pkg

echo ""
echo "4️⃣  Starting Sinatra server..."
echo "----------------------------------------"
echo "🌐 Starting Sinatra server at http://localhost:4567"
echo "📱 Open your browser and navigate to the URL above"
echo "⏹️  Press Ctrl+C to stop the server"
echo ""

ruby app.rb
