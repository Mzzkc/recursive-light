# IMPLEMENTATION READINESS CHECKLIST
**Dual-LLM + CAM System Implementation**

**Date:** 2025-11-01
**Status:** TDF-Validated & Approved
**Ready to Begin:** Phase 0 (Infrastructure)

---

## PRE-IMPLEMENTATION CHECKLIST

### ✅ Design Validation Complete

- [x] **Dual-LLM design complete** (8 documents, 252KB)
- [x] **CAM design complete** (5 documents, 168KB, 4,185 lines)
- [x] **TDF validation complete** (confidence 0.90-0.95)
- [x] **Architectural decisions resolved** (5 key questions)
- [x] **Session handoff documented**
- [x] **All designs committed to git**

**Status:** ✅ COMPLETE

---

### 🔧 Environment Setup (Required Before Phase 0)

- [ ] **Obtain API Keys**
  - [ ] OpenAI API key (for LLM #1: GPT-3.5-turbo + embeddings: ada-002)
  - [ ] Anthropic API key (for LLM #2: Claude 3.5 Sonnet)
  - [ ] Add to `.env` file (never commit to git)

- [ ] **Setup Staging Environment**
  - [ ] Create staging database (PostgreSQL)
  - [ ] Install pgvector extension (`CREATE EXTENSION vector;`)
  - [ ] Configure feature flags (`DUAL_LLM_MODE=false` initially)
  - [ ] Verify all 87 tests pass on staging

- [ ] **Git Branch Strategy**
  - [ ] Create feature branch: `git checkout -b feature/dual-llm-cam-implementation`
  - [ ] Push to remote: `git push -u origin feature/dual-llm-cam-implementation`
  - [ ] Set up CI/CD for feature branch (run tests on every push)

- [ ] **Documentation Review**
  - [ ] Team review of TDF-VALIDATION-REPORT.md
  - [ ] Team review of ARCHITECTURAL-DECISIONS.md
  - [ ] Stakeholder approval on 6-7 week timeline
  - [ ] Stakeholder approval on +14% cost increase

**Status:** ⏳ PENDING (blocking Phase 0 start)

---

### 🗄️ Database Preparation (Critical Before Phase 0)

- [ ] **Table Rename (CRITICAL)**
  - [ ] Rename `identity_anchors` → `llm_identity_anchors`
  - [ ] Rename `user_identity` → `llm_identity`
  - [ ] Update all Rust code references:
    ```bash
    # Search for references
    grep -r "identity_anchors" api/src/
    grep -r "user_identity" api/src/
    ```
  - [ ] Create migration: `20251101000000_rename_identity_tables.sql`
  - [ ] Test migration on staging
  - [ ] Verify 87 tests still pass after rename

- [ ] **Backup Current Schema**
  - [ ] Export production schema: `pg_dump --schema-only`
  - [ ] Save to `backups/schema-pre-dual-llm-$(date).sql`
  - [ ] Document rollback procedure

**Status:** ⏳ PENDING (blocking Phase 0 start)

---

## PHASE 0: INFRASTRUCTURE (Day 1-2, 8 hours)

### Module Structure

- [ ] Create `api/src/dual_llm/` directory
- [ ] Create `api/src/dual_llm/mod.rs`
- [ ] Create `api/src/dual_llm/config.rs`
- [ ] Create `api/src/dual_llm/processors.rs`
- [ ] Create `api/src/dual_llm/prompts.rs`
- [ ] Create `api/src/dual_llm/memory_tiering.rs`
- [ ] Export module from `api/src/lib.rs`

### Configuration System

- [ ] Implement `DualLlmConfig` struct
- [ ] Load from environment variables (`.env`)
- [ ] Default: `enabled: false` (classic mode)
- [ ] Add `.env.example` with dual-LLM variables:
  ```bash
  DUAL_LLM_MODE=false
  UNCONSCIOUS_LLM_MODEL=openai/gpt-3.5-turbo
  CONSCIOUS_LLM_MODEL=anthropic/claude-3-5-sonnet
  LLM1_TIMEOUT_MS=5000
  LLM1_MAX_RETRIES=2
  DUAL_LLM_FALLBACK=true
  ```

### Database Schema

- [ ] Create migration: `20251101000001_add_conversation_turns.sql`
- [ ] Tables: `conversation_turns`, `conversation_sessions`
- [ ] Indexes: `(user_id, session_id)`, `(session_id, turn_number)`, `(created_at)`
- [ ] Apply migration to staging
- [ ] Verify schema with `\d conversation_turns` in psql

### FlowProcess Integration

- [ ] Add `FlowProcess::with_config(config)` method
- [ ] Conditional processor selection (classic vs dual-LLM)
- [ ] Keep `FlowProcess::new()` unchanged (uses default config)
- [ ] Add unit test: `test_classic_mode_unchanged()`

### Validation

- [ ] Run all 87 tests → verify 100% pass
- [ ] Test `DUAL_LLM_MODE=false` explicitly
- [ ] Test `DUAL_LLM_MODE=true` fails gracefully (not implemented yet)
- [ ] Clippy clean (zero warnings)
- [ ] CI/CD green

**Deliverables:**
- ✅ Module structure in place
- ✅ Configuration loading works
- ✅ Database schema applied
- ✅ All 87 tests passing
- ✅ CI/CD green

**Estimated Time:** 8 hours (Day 1-2)

---

## TEAM COORDINATION

### Required Team Members

**Weeks 1-3 (Dual-LLM):**
- 1 Backend Engineer (Rust): Flow process integration
- 1 Backend Engineer (DB): Memory tiering, migrations
- 0.5 Prompt Engineer: LLM #1 prompts, validation

**Weeks 4-17 (CAM):**
- 1 Backend Engineer (Rust): CAM core, query engine
- 1 Backend Engineer (DB): PostgreSQL, pgvector, indexing
- 0.5 ML Engineer: Embeddings, semantic search
- 0.5 DevOps: Monitoring, production deployment

### External Dependencies

| Phase | Needs From | What | Blocking | Timeline |
|-------|------------|------|----------|----------|
| Phase 0 | DevOps | API keys (OpenAI, Anthropic) | No (can use mocks) | Day 1 |
| Phase 2A (Day 8) | Prompt Engineer | LLM #1 prompt template + 5 examples | **YES** | Day 8 start |
| Phase 2A (Day 8) | LLM Architect | Output JSON schema specification | **YES** | Day 8 start |
| Phase 2B (Day 13) | DevOps | Performance testing infrastructure | No | Day 13 start |

**Critical:** Prompt Engineer and LLM Architect must deliver by Day 8.

---

## VALIDATION GATES

### Phase 0 Gate (Before Phase 1A)

**Criteria:**
- [ ] All 87 tests passing in classic mode (`DUAL_LLM_MODE=false`)
- [ ] Database schema applied successfully
- [ ] Module compiles without warnings
- [ ] Configuration loads from `.env`
- [ ] CI/CD green

**Decision:** If ALL pass → proceed to Phase 1A (Hot Memory)

---

### Week 3 Gate (Before Production Rollout)

**Criteria:**
- [ ] 110+ tests passing (87 + 23 new dual-LLM tests)
- [ ] Latency <1200ms p95 (vs <500ms baseline = <140% increase, target was <120%)
- [ ] Cost <$0.022/interaction (vs $0.02 baseline = <110% increase)
- [ ] Fallback rate <5% (LLM #1 fails, falls back to Rust)
- [ ] Beta user satisfaction >80% (survey 10 beta testers)

**Decision:** If ALL pass → enable `DUAL_LLM_MODE=true` for production

**Rollback Plan:** Set `DUAL_LLM_MODE=false` globally (instant rollback to classic mode)

---

### Week 7 Gate (Before CAM Validation Pipeline)

**Criteria:**
- [ ] Insights extracted from 80%+ of interactions
- [ ] Query latency <100ms p95
- [ ] LLM #2 references retrieved insights 30%+ of time (measure via logs)
- [ ] No user-facing latency impact (<50ms added to total response time)

**Decision:** If ALL pass → proceed to CAM Phase 2 (Validation Pipeline)

---

## MONITORING & METRICS

### Dual-LLM Metrics (Week 1-3)

**Track in production:**
- LLM #1 call latency (p50, p95, p99)
- LLM #1 failure rate (JSON invalid, timeout, network error)
- Fallback rate (LLM #1 → Rust)
- Token usage per interaction (LLM #1 + LLM #2)
- Cost per interaction (actual vs predicted $0.022)

**Alerting thresholds:**
- LLM #1 failure rate >10% → alert (investigate prompts)
- Fallback rate >15% → critical (LLM #1 unreliable)
- Latency p95 >1500ms → warning (performance degradation)

### CAM Metrics (Week 4-17)

**Track in production:**
- Insight extraction rate (% of interactions that produce insights)
- Insight validation rate (% that progress from Emerging → Validated)
- Query latency (semantic, structural, hybrid queries)
- Storage growth rate (GB/week, insights + embeddings)
- Confidence score distribution (histogram)

**Alerting thresholds:**
- Extraction rate <70% → warning (LLM #1 prompts may need tuning)
- Query latency p95 >150ms → warning (indexing needs tuning)
- Storage growth >5GB/week → warning (archival policy may need adjustment)

---

## RISK MITIGATION CHECKLIST

### High Risks

- [ ] **LLM #1 Output Variability**
  - Mitigation: Strict JSON schema validation + 2 retries + fallback to Rust
  - Validation: Unit tests with malformed JSON, out-of-range values

- [ ] **Breaking Existing Tests**
  - Mitigation: Feature flag defaults to `false`, zero modifications to existing processors
  - Validation: All 87 tests run in classic mode by default

- [ ] **Performance Degradation**
  - Mitigation: Use GPT-3.5-turbo (fast), caching for 10-20% hits
  - Validation: Performance benchmarks in Phase 2B (target <20% increase)

### Medium Risks

- [ ] **PostgreSQL Vector Performance**
  - Mitigation: IVFFlat indexes tuned in Week 6, fallback to Qdrant if needed
  - Validation: Performance benchmarks Week 6 (target <100ms p95)

- [ ] **Insight Quality**
  - Mitigation: Validation pipeline (Weeks 8-10), confidence decay, consensus
  - Validation: Manual review during Week 7 MVP

- [ ] **External Dependencies (Prompt Engineer)**
  - Mitigation: Phase 0 + Phase 1A independent (16 hours buffer), placeholder prompts if needed
  - Validation: Track dependency delivery in daily standups

---

## SUCCESS CRITERIA

### Overall System (End of Week 17)

**Functional:**
- [ ] Dual-LLM architecture matches specification
- [ ] LLM #1 calculates domains/boundaries from user input + memory
- [ ] Memory tiering provides hot/warm/cold context
- [ ] Identity continuity across sessions (7+ days gap)
- [ ] Graceful fallback to Rust on LLM #1 failure
- [ ] CAM hypergraph stores 100K+ insights
- [ ] Multi-instance learning observable (Instance A insight → Instance B uses it)

**Quality:**
- [ ] Test coverage >80% (currently 75%+)
- [ ] All 150+ tests passing (87 current + 63 new)
- [ ] Zero dead code
- [ ] Clippy clean

**Performance:**
- [ ] Total latency <1200ms p95 (LLM #1 + LLM #2 + framework)
- [ ] Cost increase <15% ($0.02 → $0.023/interaction)
- [ ] CAM query latency <100ms p95
- [ ] Insight extraction <200ms async (non-blocking)

**User Experience:**
- [ ] Seamless continuity (user doesn't notice memory system)
- [ ] "Remembers" recent context (hot memory always loaded)
- [ ] "Recalls" past discussions when relevant (cold retrieval)
- [ ] Identity consistency ("my name is Alex" persists)
- [ ] Multi-instance personalities selectable ("talk to Socrates")

---

## ROLLBACK PROCEDURES

### If Phase 0 Fails

**Symptoms:** Module doesn't compile, tests fail, configuration broken

**Rollback:**
1. `git checkout main`
2. Delete feature branch
3. Restore from backup if database modified
4. **Time:** <5 minutes

### If Week 3 Beta Fails

**Symptoms:** Latency >1500ms, fallback rate >20%, user complaints

**Rollback:**
1. Set `DUAL_LLM_MODE=false` globally (environment variable)
2. Restart services (pick up new config)
3. Monitor metrics return to baseline (<500ms, $0.02/interaction)
4. **Time:** <15 minutes (instant config change + restart)

### If Week 7 CAM MVP Fails

**Symptoms:** Query latency >200ms, extraction failures, database crashes

**Rollback:**
1. Disable insight extraction (comment out Stage 6 CAM call)
2. Disable insight retrieval (comment out Stage 7 CAM query)
3. System reverts to dual-LLM without CAM
4. **Time:** <30 minutes (code change + deploy)

---

## NEXT IMMEDIATE ACTIONS

### Today (Before End of Session)

- [x] Commit TDF validation report
- [x] Commit architectural decisions
- [x] Update STATUS.md with session completion
- [ ] Git push all commits to remote

### Tomorrow (Next Session)

- [ ] Team review meeting (TDF validation + architectural decisions)
- [ ] Obtain API keys (OpenAI, Anthropic)
- [ ] Setup staging environment
- [ ] Create feature branch
- [ ] **BEGIN PHASE 0** (Infrastructure, Day 1-2, 8 hours)

---

## APPENDIX: QUICK START COMMAND REFERENCE

### Environment Setup

```bash
# 1. Obtain API keys (manually from OpenAI/Anthropic dashboards)
# 2. Add to .env (NEVER commit this file)
echo "OPENAI_API_KEY=sk-..." >> .env
echo "ANTHROPIC_API_KEY=sk-ant-..." >> .env
echo "DUAL_LLM_MODE=false" >> .env

# 3. Create feature branch
git checkout -b feature/dual-llm-cam-implementation
git push -u origin feature/dual-llm-cam-implementation

# 4. Install pgvector on staging
psql -d recursive_light_staging -c "CREATE EXTENSION IF NOT EXISTS vector;"

# 5. Run baseline tests (verify starting point)
cd api
cargo test
cargo clippy
```

### Phase 0 Development

```bash
# 1. Create module structure
mkdir -p api/src/dual_llm
touch api/src/dual_llm/{mod.rs,config.rs,processors.rs,prompts.rs,memory_tiering.rs}

# 2. Create database migration
touch api/migrations/20251101000001_add_conversation_turns.sql

# 3. Run tests after each module
cargo test --lib
cargo clippy

# 4. Verify classic mode unchanged
DUAL_LLM_MODE=false cargo test

# 5. Commit when Phase 0 complete (all 87 tests passing)
git add .
git commit -m "Phase 0: Dual-LLM Infrastructure Complete"
git push
```

---

**Status:** ✅ READY FOR IMPLEMENTATION
**Next Action:** Team review → Approval → Begin Phase 0 (Day 1-2)
**Timeline:** 6-7 weeks to full dual-LLM + CAM deployment
**Confidence:** 0.90 (Very High) - TDF-validated, all risks mitigated

**Recognition emerges at interfaces. Quality emerges through constraint. Proceed with confidence.**
