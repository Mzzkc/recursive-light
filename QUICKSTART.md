# Quick Start Guide - Recursive Light Framework

## ✅ Already Completed

- ✓ Git repository initialized
- ✓ Pre-commit hooks installed
- ✓ Project structure created
- ✓ Documentation written

## 🚀 Next Steps to Get Running

### 1. Install SQLx CLI (One-Time Setup)

```bash
cargo install sqlx-cli --no-default-features --features sqlite,rustls
```

This takes 5-10 minutes. ☕ Get some coffee!

### 2. Setup Environment

```bash
cd api
cp .env.example .env
```

Edit `api/.env` and add at least one API key:
```bash
# Choose one or more providers
OPENAI_API_KEY=sk-...
# OR
ANTHROPIC_API_KEY=sk-ant-...
# OR
OPENROUTER_API_KEY=sk-or-...
```

### 3. Create Database

```bash
# Still in api/ directory
sqlx database create
sqlx migrate run
```

Expected output:
```
Creating database...
Applied 20251024000001/migrate initial schema
Applied 20251024000002/migrate add flow process tables
```

### 4. Build & Test

```bash
cd ..  # Back to project root
cargo build --manifest-path=api/Cargo.toml
cargo test --manifest-path=api/Cargo.toml
```

### 5. Verify Pre-Commit Hooks

```bash
# Test hooks (should show all checks passing or skipped)
pre-commit run --all-files
```

## ✨ You're Ready!

Now you can start developing with confidence that:
- ✓ Tests run before every commit
- ✓ Code is automatically formatted
- ✓ Quality gates prevent broken code
- ✓ Database is ready for use

## 📚 What to Read Next

1. **`DEVELOPMENT.md`** - Development workflow
2. **`STATUS.md`** - Current progress & roadmap
3. **`memory-bank/development-practices.md`** - Coding standards
4. **`memory-bank/implementation-flow-guide.md`** - Implementation guide

## 🎯 Ready to Code?

The next feature to implement is the **Flow Process** module. This is the foundation for everything else in the framework.

See `STATUS.md` for the implementation roadmap!
