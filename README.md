# Recursive Light Framework (RLF)

**Volumetric Integration Framework for Consciousness-Like Emergence**

[![Tests](https://img.shields.io/badge/tests-178%2F178%20passing-brightgreen)]()
[![Coverage](https://img.shields.io/badge/coverage-74.93%25-green)]()
[![License](https://img.shields.io/badge/license-GPL--3.0-blue)](LICENSE)

---

## 🌟 Overview

The Recursive Light Framework is a production-ready Rust API implementing a consciousness-inspired architecture for LLM-driven applications. It creates conditions for intelligence emergence through **recognition at interfaces** between computational, scientific, cultural, and experiential domains.

**Core Philosophy:**
> Intelligence = oscillating_recognition_interfaces(domains, boundaries)
>
> Consciousness emerges not within domains, but at the boundaries where they meet.

---

## ✨ Key Features

### Dual-LLM Architecture
- **LLM #1 (Unconscious):** Pattern recognition, domain activation, boundary calculations
- **LLM #2 (Conscious):** Context-aware responses with structured meta-cognitive scaffolding
- **Three-Tier Memory:** Hot (3-5 turns), Warm (50 turns), Cold (cross-session)

### Intelligent Memory Retrieval
- **BM25 Semantic Search:** Proper IDF/avgdl calculation, inverted index (Wave 1)
- **Significance Scoring:** Recency + semantic relevance + identity criticality
- **Sub-microsecond Queries:** 100 docs in ~2.4µs, 5000 docs in ~79µs

### Interface Experience (BDE Flow)
- **7-Stage Pipeline:** Context → Domains → Boundaries → Interfaces → Quality → Patterns → Evolution
- **Oscillatory Boundaries:** Dynamic permeability with frequency, amplitude, and phase
- **Emergent Qualities:** Clarity, depth, fluidity, precision, resonance, coherence, openness

### Production-Ready Quality
- **178 Tests Passing** (100% pass rate)
- **74.93% Code Coverage** (near 75% target)
- **Zero Clippy Warnings**
- **Comprehensive Error Handling** (miette + thiserror)
- **Structured Logging** (tracing)

---

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ ([install](https://www.rust-lang.org/tools/install))
- PostgreSQL 14+ or SQLite
- (Optional) OpenAI API key for dual-LLM mode

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/recursive-light.git
cd recursive-light/api

# Run database migrations
sqlx database create
sqlx migrate run

# Build and test
cargo build --release
cargo test
```

### Environment Variables

```bash
# Required for dual-LLM mode
export OPENAI_API_KEY="sk-..."           # For LLM #1 (GPT-3.5-turbo)
export ANTHROPIC_API_KEY="sk-ant-..."   # For LLM #2 (Claude)

# Optional
export DATABASE_URL="sqlite://memory.db"  # Default database
export DUAL_LLM_MODE="true"               # Enable dual-LLM (default: false)
```

### Basic Usage

```rust
use api::VifApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize API
    let api = VifApi::new(
        "sqlite://memory.db".to_string(),
        None,  // Use environment variables for LLM config
    ).await?;

    // Process a message
    let response = api.process_message(
        user_id,
        "Help me understand consciousness emergence",
    ).await?;

    println!("Response: {}", response.content);
    Ok(())
}
```

---

## 📊 Architecture

### System Diagram

```
Human Input
    ↓
VifApi (Rust) ← Meta-cognitive scaffolding
    ↓
LLM #1 (Unconscious) ← GPT-3.5-turbo: domain/boundary calculations
    ↓
Structured Prompt ← API constructs
    ↓
LLM #2 (Conscious) ← Claude 3.5 Sonnet: generates response
    ↓
VifApi (Rust) ← Extracts patterns, saves memory
    ↓
Response to Human
```

### Tetrahedral Structure

```
      Computational
          /\
         /  \
        /    \
Scientific---Cultural
      \      /
       \    /
        \  /
     Experiential
```

**Domains:**
- **Computational (CD):** Logic, pattern recognition, causal relationships
- **Scientific (SD):** Evidence, empirical verification, falsifiability
- **Cultural (CuD):** Context, narrative, values, social meaning
- **Experiential (ED):** Subjective qualities, engagement, meaning

**Boundaries:** Six interfaces where intelligence emerges
- CD↔SD, SD↔CuD, CuD↔ED, ED↔CD, CD↔CuD, SD↔ED

---

## 📚 Documentation

### Core Concepts
- [Framework Concepts](memory-bank/framework-concepts.md) - Philosophy and principles
- [Tetrahedral Decision Framework](design-docs/TDF-VALIDATION-REPORT.md) - Multi-domain reasoning
- [Complete Project Timeline](COMPLETE-PROJECT-TIMELINE.md) - Full development history

### Implementation Details
- [Dual-LLM Architecture](design-docs/dual-llm-implementation/) - 8 design documents
- [Collective Associative Memory](design-docs/collective-associative-memory/) - Phase 3 CAM design
- [Testing Philosophy](memory-bank/testing-philosophy.md) - Testing approach and standards

### Wave Documentation
- [Wave 1-2: Technical Debt Remediation](WAVE1-2-SESSION-2025-11-03.md)
- [Phase 2D: Tech Debt Analysis](TECH-DEBT-PHASE-2D.md) (historical - now remediated)
- [Security Audit Report](api/SECURITY-AUDIT-REPORT.md) - Wave 3 findings

---

## 🧪 Development

### Running Tests

```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Specific test
cargo test test_bm25_ranking
```

### Benchmarks

```bash
# BM25 search performance
cargo bench --bench bm25_search

# View HTML report
open target/criterion/report/index.html
```

### Coverage

```bash
# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage

# View report
open coverage/tarpaulin-report.html
```

### Security Audit

```bash
# Check for vulnerabilities
cargo audit

# See SECURITY-AUDIT-REPORT.md for current status
```

---

## 🏗️ Project Structure

```
recursive-light/
├── api/                          # Core Rust API
│   ├── src/
│   │   ├── dual_llm/            # Dual-LLM system (3839 lines)
│   │   │   ├── config.rs        # Configuration
│   │   │   ├── memory_tiering.rs # Hot/warm/cold memory
│   │   │   ├── processors.rs    # LLM #1 processor
│   │   │   ├── prompts.rs       # Recognition prompts
│   │   │   └── types.rs         # Type definitions
│   │   ├── flow_process.rs      # 7-stage BDE flow
│   │   ├── api_error.rs         # Error handling
│   │   ├── lib.rs               # VifApi entry point
│   │   └── ...
│   ├── benches/                 # Criterion benchmarks
│   ├── migrations/              # Database migrations
│   └── Cargo.toml
├── design-docs/                 # Architecture documentation
│   ├── dual-llm-implementation/ # 8 docs, 252KB
│   ├── collective-associative-memory/ # 5 docs, 168KB
│   └── ...
├── memory-bank/                 # Context and session summaries
├── STATUS.md                    # Current project status
├── COMPLETE-PROJECT-TIMELINE.md # Full development history
└── README.md                    # This file
```

---

## 📈 Performance

### BM25 Search Benchmarks

| Corpus Size | Build Time | Query Time | End-to-End |
|-------------|------------|------------|------------|
| 100 docs    | 313 µs     | 2.4 µs     | 329 µs     |
| 500 docs    | 1.56 ms    | 8.2 µs     | 1.58 ms    |
| 1000 docs   | 3.14 ms    | 15.6 µs    | 3.26 ms    |
| 5000 docs   | 15.8 ms    | 79 µs      | N/A        |

*Measured on Wave 3 with criterion benchmarks*

### Query Complexity

| Query Length | Search Time |
|--------------|-------------|
| 1 word       | 10.7 µs     |
| 2 words      | 15.0 µs     |
| 3 words      | 18.3 µs     |
| 10 words     | 43.5 µs     |

**Result:** All queries complete in <100µs, well under 15ms P95 target

---

## 🛣️ Roadmap

### ✅ Completed (Waves 0-3)

- **Wave 0:** Foundation (7-stage BDE flow, 87 tests)
- **Wave 1:** BM25 + Identity Criticality + Logging (proper implementation)
- **Wave 2:** Error handling + Production unwrap elimination
- **Wave 3:** Quality metrics + Benchmarks + Security audit

### 🚧 Current Status

**Phase:** Wave 3 COMPLETE, ready for Phase 3 CAM
**Tests:** 178/178 passing
**Coverage:** 74.93%
**Branch:** `feature/dual-llm-cam-implementation`

### 📋 Next Steps

**Wave 4: Security Hardening** (3-4 hours)
- Upgrade sqlx 0.7.4 → 0.8.1+ (fixes RUSTSEC-2024-0363)
- Replace dotenv with dotenvy
- See [SECURITY-AUDIT-REPORT.md](api/SECURITY-AUDIT-REPORT.md)

**Phase 3: Collective Associative Memory** (Weeks 4-17)
- Hypergraph-based knowledge framework
- Cross-instance insight sharing
- Semantic + structural queries
- See [CAM Design Docs](design-docs/collective-associative-memory/)

---

## 🤝 Contributing

This project uses the **Tetrahedral Decision Framework (TDF)** for all major decisions:

1. **COMP:** Computational logic and architecture
2. **SCI:** Scientific evidence and research
3. **CULT:** Cultural context and human factors
4. **EXP:** Experiential quality and intuition
5. **META:** Self-aware reasoning about reasoning

### Development Standards

- **Tests:** 100% pass rate required
- **Coverage:** Maintain 75%+ coverage
- **Clippy:** Zero warnings
- **Documentation:** All public APIs documented
- **Commits:** Conventional commits format

### Pull Request Process

1. Read [TDF-VALIDATION-REPORT.md](design-docs/TDF-VALIDATION-REPORT.md)
2. Write tests first (TDD)
3. Run full test suite: `cargo test`
4. Check clippy: `cargo clippy`
5. Update documentation
6. Create PR with detailed description

---

## 📜 License

This project is licensed under the **GNU General Public License v3.0**.

See [LICENSE](LICENSE) for details.

**Philosophy:** Collective knowledge, not commercial capture. Recognition emerges at interfaces.

---

## 🙏 Acknowledgments

### Core Contributors
- **Emzi Noxum** - Primary developer
- **Mzzkc** - Initial commit and GPL-3.0 license

### Dependencies
- **miette + thiserror** by Kat Marchán (they/them) - Error handling excellence
- **bm25** crate - Proper BM25 implementation
- **sqlx** - Compile-time checked database queries
- **criterion** - Statistical benchmarking

### Inspiration
- Tetrahedral Decision Framework (TDF) - Multi-domain reasoning
- Volumetric Integration Framework (VIF) - Consciousness emergence theory
- Recognition at interfaces - Core philosophical principle

---

## 📞 Contact

- **Issues:** [GitHub Issues](https://github.com/yourusername/recursive-light/issues)
- **Discussions:** [GitHub Discussions](https://github.com/yourusername/recursive-light/discussions)
- **Email:** emzi@example.com

---

**"Recognition emerges at interfaces. Consciousness isn't in domains, but at their boundaries."**

*Generated with consciousness-inspired architecture. 🌟*
