#!/bin/bash
# Recursive Light Framework - Development Environment Setup
# This script sets up everything you need to start developing

set -e  # Exit on error

echo "🌟 Recursive Light Framework - Setup"
echo "=====================================\n"

# Check prerequisites
echo "📋 Checking prerequisites..."

# Check Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust not found. Install from https://rustup.rs/"
    exit 1
fi
echo "✓ Rust found: $(rustc --version)"

# Check Python (for pre-commit)
if ! command -v python3 &> /dev/null && ! command -v python &> /dev/null; then
    echo "❌ Python not found. Install Python 3.8+"
    exit 1
fi
echo "✓ Python found"

# Install pre-commit if not already installed
echo "\n📦 Installing pre-commit hooks..."
if ! command -v pre-commit &> /dev/null; then
    echo "Installing pre-commit via pipx..."
    # Check if pipx is installed
    if ! command -v pipx &> /dev/null; then
        echo "Installing pipx first..."
        sudo apt update && sudo apt install -y pipx || python3 -m pip install --user pipx
        pipx ensurepath
    fi
    pipx install pre-commit
    echo "✓ pre-commit installed via pipx"
else
    echo "✓ pre-commit already installed"
fi
pre-commit install
echo "✓ Pre-commit hooks installed"

# Install SQLx CLI
echo "\n📦 Installing SQLx CLI..."
if ! command -v sqlx &> /dev/null; then
    echo "Installing sqlx-cli (this may take a few minutes)..."
    cargo install sqlx-cli --no-default-features --features sqlite,rustls
else
    echo "✓ SQLx CLI already installed"
fi

# Set up environment
echo "\n⚙️  Setting up environment..."
cd api
if [ ! -f .env ]; then
    cp .env.example .env
    echo "✓ Created .env file (please add your API keys)"
else
    echo "✓ .env file already exists"
fi

# Create database
echo "\n🗄️  Setting up database..."
if [ ! -f dev.db ]; then
    sqlx database create
    echo "✓ Database created"
else
    echo "✓ Database already exists"
fi

# Run migrations
echo "\n📊 Running database migrations..."
sqlx migrate run
echo "✓ Migrations complete"

# Build project
echo "\n🔨 Building project..."
cargo build
echo "✓ Build successful"

# Run tests
echo "\n🧪 Running tests..."
cargo test
echo "✓ Tests passed"

echo "\n✅ Setup complete!"
echo "\n📚 Next steps:"
echo "  1. Edit api/.env and add your LLM API keys"
echo "  2. Read DEVELOPMENT.md for development workflow"
echo "  3. Start coding! (See memory-bank/ for specifications)"
echo "\n🚀 Happy coding!"
