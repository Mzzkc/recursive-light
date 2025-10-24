# Installation Guide - Recursive Light Framework

## Quick Start (Automated)

```bash
./setup.sh
```

If you encounter Python environment errors, see the manual installation below.

## Manual Installation

### 1. Install Pre-Commit (Choose One Method)

#### Method A: Using pipx (Recommended for Modern Linux)
```bash
# Install pipx if not already installed
sudo apt update && sudo apt install pipx
pipx ensurepath

# Install pre-commit
pipx install pre-commit

# Install hooks in repository
pre-commit install
```

#### Method B: Using apt (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install pre-commit
pre-commit install
```

#### Method C: Using Virtual Environment
```bash
# Create virtual environment
python3 -m venv .venv
source .venv/bin/activate

# Install pre-commit
pip install pre-commit
pre-commit install
```

### 2. Install SQLx CLI

```bash
cargo install sqlx-cli --no-default-features --features sqlite,rustls
```

This may take 5-10 minutes to compile.

### 3. Setup Environment

```bash
cd api
cp .env.example .env
```

Edit `.env` and add your LLM API keys:
```bash
# Required: At least one provider
OPENAI_API_KEY=sk-...
# Or
ANTHROPIC_API_KEY=sk-ant-...
# Or
OPENROUTER_API_KEY=sk-or-...
```

### 4. Create Database

```bash
# From the api/ directory
sqlx database create
sqlx migrate run
```

This creates `dev.db` and runs all migrations.

### 5. Build & Test

```bash
cargo build
cargo test
```

## Troubleshooting

### Python Package Installation Errors

If you see `externally-managed-environment` errors:

**Solution 1 - Use pipx (Recommended)**:
```bash
sudo apt install pipx
pipx install pre-commit
```

**Solution 2 - Use apt**:
```bash
sudo apt install pre-commit
```

**Solution 3 - Virtual Environment**:
```bash
python3 -m venv .venv
source .venv/bin/activate
pip install pre-commit
```

### SQLx CLI Installation Takes Forever

This is normal - it's compiling from source. Grab a coffee! ☕

Alternative: Use pre-built binary (if available for your platform):
```bash
# Check releases: https://github.com/launchbadge/sqlx/releases
```

### Database Migration Errors

```bash
# Reset database (CAUTION: Deletes all data)
sqlx database drop
sqlx database create
sqlx migrate run
```

### Cargo Build Errors

```bash
# Clean and rebuild
cargo clean
cargo build
```

### Pre-commit Hook Failures

```bash
# See what's failing
pre-commit run --all-files

# Fix issues shown, then retry
cargo fmt
cargo clippy --fix --allow-dirty
cargo test
```

## Verifying Installation

After installation, verify everything works:

```bash
# Should show no errors
pre-commit run --all-files

# Should compile without errors
cargo build

# Should pass (or show 0 tests if none written yet)
cargo test

# Should show database tables
sqlite3 dev.db ".tables"
```

Expected output from `.tables`:
```
collective_insights         patterns
flow_process_executions     phenomenological_qualities
interface_experiences       state_snapshots
user_profiles              users
```

## Environment Variables Reference

Required in `api/.env`:

```bash
# Database (SQLite for development)
DATABASE_URL=sqlite:dev.db

# LLM Provider (at least one required)
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...
OPENROUTER_API_KEY=sk-or-...

# Optional: Default provider configuration
DEFAULT_PROVIDER=openai
DEFAULT_MODEL=gpt-4
DEFAULT_TOKEN_BUDGET=2048
```

## Next Steps

Once installed:

1. Read `DEVELOPMENT.md` for development workflow
2. Check `STATUS.md` for current progress
3. Review `memory-bank/development-practices.md` for coding standards
4. Start implementing features with TDD approach

## Getting Help

- Setup issues: Check this INSTALL.md
- Development workflow: See DEVELOPMENT.md
- Framework concepts: See memory-bank/framework-concepts.md
- Implementation guidance: See memory-bank/implementation-flow-guide.md
