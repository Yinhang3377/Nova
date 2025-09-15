#!/bin/bash
# Nova Project Setup Script
# This script initializes the Nova project environment

set -e  # Exit on any error

echo "🌟 Nova Project Setup"
echo "===================="

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check for common development tools
echo "Checking development environment..."

if command_exists git; then
    echo "✅ Git is installed"
else
    echo "❌ Git is not installed. Please install Git first."
    exit 1
fi

if command_exists python3; then
    echo "✅ Python 3 is available"
fi

if command_exists node; then
    echo "✅ Node.js is available"
fi

if command_exists go; then
    echo "✅ Go is available"
fi

if command_exists rustc; then
    echo "✅ Rust is available"
fi

echo ""
echo "📁 Project structure:"
echo "src/        - Source code"
echo "tests/      - Test files"
echo "docs/       - Documentation"
echo "examples/   - Example implementations"
echo "scripts/    - Build and utility scripts"

echo ""
echo "🚀 Nova is ready for development!"
echo "Check out the examples/ directory to get started."
echo "Read CONTRIBUTING.md for development guidelines."

echo ""
echo "Next steps:"
echo "1. Add your source code to src/"
echo "2. Write tests in tests/"
echo "3. Update documentation in docs/"
echo "4. Run ./scripts/test.sh to verify everything works"