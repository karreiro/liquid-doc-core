#!/bin/bash

# Test all fixtures with different iteration counts
echo "🚀 Starting comprehensive fixture testing..."

# Default iterations
ITERATIONS=${1:-100}

echo "Using $ITERATIONS iterations per test"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Run the Node.js test script
node test-fixtures.js $ITERATIONS

echo ""
echo "✅ All tests completed!"
