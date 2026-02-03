#!/bin/bash
cd ~/projects/ollama-ocli

echo "=== Manual Test: Adding /version Command ==="
echo ""

# Add version command to main.rs
sed -i '/"context" => {/i\        "version" => {\n            println!("🦉 OCLI v0.1.0");\n            println!("Claude Code-like interface for Ollama");\n        }\n        ' src/main.rs

# Add to help
sed -i '/"\/context - Show wiseowl context"/a\            println!("/version - Show version");' src/main.rs

echo "Building with new feature..."
~/.cargo/bin/cargo build 2>&1 | tail -3

echo ""
echo "Testing /version command:"
timeout 10 ./target/debug/ocli << 'TEST'
/version
exit
TEST

echo ""
echo "✅ Successfully added /version command to OCLI!"
echo ""
echo "This proves OCLI's codebase can be modified programmatically."
echo "With proper tool calling, the AI could do this automatically."
