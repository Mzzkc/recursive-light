# Development Practices: Recursive Light Framework
*Understanding emerges at recognition interfaces between code and quality*

## Core Development Philosophy

The Recursive Light Framework embodies its own principles in development: **quality emerges at the interface between implementation and verification**. We don't build code then test it - we create conditions where quality naturally emerges through integrated test-driven development.

## Testing Strategy

### Test-Driven Development (TDD)

1. **Write Tests First**: Before implementing any feature, write tests that define expected behavior
2. **Red-Green-Refactor**:
   - **Red**: Write failing test
   - **Green**: Write minimal code to pass
   - **Refactor**: Improve code while maintaining passing tests
3. **Test Layers**:
   - **Unit Tests**: Individual functions and methods
   - **Integration Tests**: Module interactions
   - **Functional Tests**: Feature-level behavior
   - **E2E Tests**: Complete user workflows
   - **Edge Case Tests**: Boundary conditions and error scenarios

### Test Coverage Goals

- **Minimum**: 80% code coverage
- **Critical Paths**: 100% coverage for:
  - Flow Process stages
  - Boundary dynamics
  - Pattern lifecycle transitions
  - Memory state management
  - API request/response handling

### Test Organization

```
api/
├── tests/
│   ├── unit/               # Unit tests per module
│   │   ├── flow_process_test.rs
│   │   ├── boundary_test.rs
│   │   ├── domain_test.rs
│   │   └── ...
│   ├── integration/        # Integration tests
│   │   ├── api_integration_test.rs
│   │   ├── memory_integration_test.rs
│   │   └── ...
│   ├── functional/         # Feature-level tests
│   │   ├── conversation_flow_test.rs
│   │   ├── state_persistence_test.rs
│   │   └── ...
│   └── e2e/               # End-to-end scenarios
│       ├── complete_conversation_test.rs
│       ├── multi_session_test.rs
│       └── ...
```

## Pre-Commit Hooks

### Purpose

Pre-commit hooks serve as **recognition interfaces** where code quality emerges before entering the codebase. They are:
- **Guardrails**: Prevent broken code from being committed
- **Reminders**: Reinforce best practices automatically
- **Quality Gates**: Ensure standards are met consistently

### What They Check

1. **Formatting**: `cargo fmt --check`
2. **Linting**: `cargo clippy -- -D warnings`
3. **Tests**: `cargo test --all-features` (MUST PASS)
4. **Documentation**: `cargo doc --no-deps`
5. **File Quality**: Trailing whitespace, file endings, etc.

### Installation

```bash
# Install pre-commit framework
pip install pre-commit

# Install hooks
pre-commit install

# Test hooks manually
pre-commit run --all-files
```

### Bypassing (Emergency Only)

```bash
# Only when absolutely necessary and with good reason
git commit --no-verify -m "Emergency fix: reason here"
```

## Database Strategy

### Development (Current)

- **SQLite**: Local, file-based, zero-config
- **Location**: `api/dev.db`
- **Migrations**: Use `sqlx migrate` for schema versioning
- **Benefits**: Fast iteration, no external dependencies

### Production (Future)

- **Supabase**: Managed PostgreSQL with pgvector
- **Migration Path**: SQLx migrations work across SQLite and PostgreSQL
- **Benefits**: Scalability, vector search, managed backups

### Database Best Practices

1. **Always use migrations**: Never modify schema directly
2. **Version control**: All migrations in `api/migrations/`
3. **Test migrations**: Both up and down
4. **Seed data**: Maintain test fixtures for development

## Code Quality Standards

### Rust Best Practices

1. **Error Handling**: Use `Result<T, E>` and `?` operator, never `unwrap()` in production code
2. **Type Safety**: Leverage Rust's type system, use newtype pattern for domain concepts
3. **Documentation**: Document all public APIs with examples
4. **Async/Await**: Use `tokio` runtime consistently
5. **Dependencies**: Minimize external dependencies, prefer battle-tested crates

### Framework-Specific Practices

1. **Domain Fluidity**: Allow domains to emerge based on context, not hardcoded activation
2. **Boundary Dynamics**: Implement full oscillatory parameters (P, F, A, φ)
3. **Interface Focus**: Create conditions for emergence, not mechanical construction
4. **State Compression**: Optimize token usage from the start
5. **Pattern Recognition**: Track patterns through lifecycle stages

## Development Workflow

### Feature Implementation Process

1. **Specify**: Write feature specification in memory-bank
2. **Design**: Define interfaces and data structures
3. **Test First**: Write comprehensive tests
4. **Implement**: Write minimal code to pass tests
5. **Refactor**: Improve code quality
6. **Document**: Update memory-bank and code docs
7. **Commit**: Let pre-commit hooks verify quality

### Commit Message Format

```
type(scope): brief description

Detailed explanation of what changed and why.

Tests:
- Added unit tests for X
- Added integration tests for Y
- All tests passing

Closes #issue_number
```

Types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`

### Branch Strategy

- `main`: Production-ready code, protected
- `develop`: Integration branch
- `feature/name`: Feature development
- `fix/name`: Bug fixes
- `test/name`: Test improvements

## Continuous Integration (Future)

### GitHub Actions Pipeline

1. **On Push**: Run all tests, linting, formatting checks
2. **On PR**: Above + code coverage report
3. **On Merge to Main**: Above + deploy to staging
4. **On Release**: Deploy to production

## Recognition Interfaces

<div style="border-left: 3px solid #2ecc71; padding-left: 10px;">
Development practices transform into software quality at the boundary where code meets verification. Notice how your confidence in the codebase emerges not from any single practice, but at the interfaces between testing, documentation, and implementation.

**Transcendence Trigger**: The pre-commit hook is not a barrier - it's a recognition interface where code quality emerges through the integration of multiple validation domains.
</div>

## Living Document

This document evolves as we discover better practices. Update it when you find improved approaches.

**Last Updated**: 2025-10-24
**Version**: 1.0
