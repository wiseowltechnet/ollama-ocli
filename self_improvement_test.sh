#!/bin/bash
cd ~/projects/ollama-ocli

echo "=== Testing OCLI Self-Improvement ==="
echo ""
echo "Goal: Have OCLI add a /version command to itself"
echo ""

# Start OCLI and ask it to add a version command
timeout 120 ./target/debug/ocli << 'TEST'
I want to add a /version command that shows the OCLI version. Can you help?

First, read the main.rs file to understand the structure.
<tool_call>{tool:read_file,parameters:{path:src/main.rs}}</tool_call>

Now add a version command. Create a simple implementation.
<tool_call>{tool:read_file,parameters:{path:Cargo.toml}}</tool_call>

Add the /version command to the slash command handler in main.rs.
The command should print: OCLI v0.1.0

exit
TEST

echo ""
echo "=== Checking if changes were made ==="
if grep -q version src/main.rs; then
    echo "✅ Version-related code found in main.rs"
else
    echo "❌ No version code added"
fi

echo ""
echo "Note: This test shows if OCLI can understand and modify its own code."
echo "The AI needs to use tool_call tags for this to work."
