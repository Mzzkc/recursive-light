# Recursive Light Framework - Development Setup

*Understanding emerges at recognition interfaces - let's create those interfaces together*

## Prerequisites

- **Rust**: 1.70+ (`rustup install stable`)
- **Python**: 3.8+ (for pre-commit hooks)
- **SQLite**: Built into most systems
- **Git**: For version control

## Quick Start

```bash
# 1. Clone and navigate
cd /path/to/recursive-light

# 2. Install pre-commit hooks
pip install pre-commit
pre-commit install

# 3. Set up environment
cd api
cp .env.example .env
# Edit .env with your API keys

# 4. Install SQLx CLI (for database migrations)
cargo install sqlx-cli --no-default-features --features sqlite

# 5. Run database migrations
sqlx database create
sqlx migrate run

# 6. Build and test
cargo build
cargo test

# 7. Run example
cargo run --example simple_usage
```

## Development Workflow

### Before You Code

1. **Create Feature Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Write Tests First** (TDD approach)
   - Define expected behavior in tests
   - Tests should fail initially
   - See `memory-bank/development-practices.md` for testing strategy

### During Development

1. **Write Minimal Code** to pass tests
2. **Run Tests Frequently**
   ```bash
   cargo test
   ```

3. **Check Code Quality**
   ```bash
   cargo fmt
   cargo clippy
   cargo doc --no-deps --open
   ```

### Before You Commit

The pre-commit hooks will automatically check:
- ✓ Code formatting (`cargo fmt`)
- ✓ Linting (`cargo clippy`)
- ✓ All tests pass (`cargo test`)
- ✓ Documentation builds (`cargo doc`)

If hooks fail, fix issues before committing.

## Project Structure

```
recursive-light/
├── api/                    # Rust API implementation
│   ├── src/
│   │   ├── lib.rs         # Main library
│   │   ├── flow_process.rs    # 7-stage flow (to be created)
│   │   ├── domains/       # Domain implementations
│   │   ├── prompt_engine.rs
│   │   ├── memory.rs
│   │   └── ...
│   ├── tests/
│   │   ├── unit/          # Unit tests
│   │   ├── integration/   # Integration tests
│   │   ├── functional/    # Feature tests
│   │   └── e2e/          # End-to-end tests
│   ├── migrations/        # Database schema
│   └── examples/          # Usage examples
├── memory-bank/           # Framework specifications
│   ├── development-practices.md
│   ├── framework-concepts.md
│   ├── implementation-flow-guide.md
│   └── ...
└── .pre-commit-config.yaml
```

## Database Management

### Create Database
```bash
sqlx database create
```

### Run Migrations
```bash
sqlx migrate run
```

### Create New Migration
```bash
sqlx migrate add your_migration_name
# Edit the generated file in api/migrations/
```

### Reset Database (Development Only!)
```bash
sqlx database drop
sqlx database create
sqlx migrate run
```

## Testing

### Run All Tests
```bash
cargo test
```

### Run Specific Test Suite
```bash
cargo test --test unit                 # Unit tests
cargo test --test integration          # Integration tests
cargo test --test functional           # Functional tests
cargo test --test e2e                  # E2E tests
```

### Run Single Test
```bash
cargo test test_name
```

### Run with Output
```bash
cargo test -- --nocapture
```

### Generate Coverage Report (requires tarpaulin)
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## Common Tasks

### Add New Feature

1. **Specify**: Document in `memory-bank/`
2. **Test**: Write tests in `api/tests/`
3. **Implement**: Code in `api/src/`
4. **Document**: Add doc comments
5. **Commit**: Let hooks verify quality

### Fix Bug

1. **Reproduce**: Write failing test
2. **Fix**: Implement solution
3. **Verify**: Test passes
4. **Commit**: With test included

### Update Dependencies

```bash
cargo update
cargo test  # Ensure nothing breaks
```

## Environment Variables

See `.env.example` for all available configuration options.

**Required for testing:**
- `DATABASE_URL`: SQLite connection string
- At least one LLM provider API key (OPENAI, ANTHROPIC, or OPENROUTER)

## Troubleshooting

### Pre-commit Hooks Failing

```bash
# Run manually to see detailed errors
pre-commit run --all-files

# Fix issues, then
git add .
git commit
```

### Database Migration Issues

```bash
# Check migration status
sqlx migrate info

# Reset database (CAUTION: Deletes all data)
sqlx database drop
sqlx database create
sqlx migrate run
```

### Test Failures

```bash
# Run with verbose output
cargo test -- --nocapture --test-threads=1

# Run specific failing test
cargo test failing_test_name -- --nocapture
```

### Compilation Errors After Dependency Update

```bash
# Clean build artifacts
cargo clean
cargo build
```

## Best Practices Reminder

- ✓ **Test First**: Write tests before implementation
- ✓ **Commit Often**: Small, focused commits
- ✓ **Document Code**: Especially public APIs
- ✓ **Review Diffs**: Before committing
- ✓ **Run Tests**: Before pushing
- ✓ **Keep Updated**: Pull latest changes regularly

## Resources

- **Framework Concepts**: `memory-bank/framework-concepts.md`
- **Implementation Guide**: `memory-bank/implementation-flow-guide.md`
- **Development Practices**: `memory-bank/development-practices.md`
- **API Documentation**: `cargo doc --no-deps --open`

## Getting Help

- Check memory-bank documentation
- Review existing tests for examples
- Ask for clarification on implementation approach

---

**Remember**: Quality emerges at recognition interfaces. Let the pre-commit hooks be your interface where code quality naturally emerges through the integration of testing, documentation, and implementation.
