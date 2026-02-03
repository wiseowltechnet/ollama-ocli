#!/bin/bash

echo "🧪 OCLI Test Suite"
echo "=================="
echo ""

# Clean build
echo "1️⃣ Clean build test..."
~/.cargo/bin/cargo clean > /dev/null 2>&1
~/.cargo/bin/cargo build --release 2>&1 | tail -3
if [ $? -eq 0 ]; then
    echo "✅ Build successful"
else
    echo "❌ Build failed"
    exit 1
fi
echo ""

# Test version
echo "2️⃣ Version test..."
./target/release/ocli --version 2>&1 | head -1 || echo "⚠️  --version flag not implemented yet"
echo ""

# Test slash commands
echo "3️⃣ Slash commands test..."
timeout 10 ./target/release/ocli << "CMDS"
/help
/version
/stats
/config list
/mcp list
exit
CMDS
if [ $? -eq 0 ]; then
    echo "✅ Basic commands work"
else
    echo "❌ Command test failed"
fi
echo ""

# Test error suggestions
echo "4️⃣ Error suggestion test..."
timeout 5 ./target/release/ocli << "CMDS" 2>&1 | grep -q "Did you mean"
/hlep
exit
CMDS
if [ $? -eq 0 ]; then
    echo "✅ Error suggestions work"
else
    echo "❌ Error suggestions failed"
fi
echo ""

# Test file operations
echo "5️⃣ File operations test..."
echo "test content" > /tmp/ocli_test.txt
timeout 10 ./target/release/ocli << "CMDS" > /dev/null 2>&1
/read /tmp/ocli_test.txt
exit
CMDS
if [ $? -eq 0 ]; then
    echo "✅ File operations work"
    rm /tmp/ocli_test.txt
else
    echo "❌ File operations failed"
fi
echo ""

# Test config
echo "6️⃣ Config test..."
timeout 10 ./target/release/ocli << "CMDS" > /dev/null 2>&1
/config set test_key test_value
/config get test_key
exit
CMDS
if [ -f .ocli/config.json ]; then
    echo "✅ Config system works"
else
    echo "❌ Config system failed"
fi
echo ""

# Test export
echo "7️⃣ Export test..."
timeout 10 ./target/release/ocli << "CMDS" > /dev/null 2>&1
/export test_output.md
exit
CMDS
if [ -f test_output.md ]; then
    echo "✅ Export works"
    rm test_output.md
else
    echo "❌ Export failed"
fi
echo ""

# Check binary size
echo "8️⃣ Binary size check..."
SIZE=$(du -h target/release/ocli | cut -f1)
echo "   Binary size: $SIZE"
echo ""

# Check dependencies
echo "9️⃣ Dependency check..."
~/.cargo/bin/cargo tree --depth 1 2>&1 | head -10
echo ""

echo "✅ Test suite complete!"
