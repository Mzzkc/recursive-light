# Phase 3B/3 Integration Implementation Plan
**Created**: 2025-11-21
**Status**: Ready for execution
**Estimated Effort**: 40-50 hours (1-2 weeks)

## Quick Reference

**Current State**: All foundations complete, 171/171 tests passing, blocker eliminated
**Goal**: Transform from session-based to person-centric continuous conversation
**Critical Path**: Phase 3B.2 → 3B.3 → 3B.4+3B.5 (parallel) → Phase 3 CAM
**Total Scope**: ~2500-3500 LOC, 60-75 new tests, 14 files modified

---

## Architecture Transformation

### Current Flow (Session-Based - lib.rs:400-706)
```
User message
  ↓
Load hot memory (session_id)
  ↓
Keyword-triggered search (warm/cold)
  ↓
Execute FlowProcess (user_input alone, no memory context)
  ↓
Inject memory into LLM #2 AFTER flow
  ↓
LLM #2 generates response
  ↓
Store turn
```

**Problems:**
- LLM #1 doesn't see conversation history
- Memory selection not intelligent (keyword matching only)
- No temporal awareness (resuming after 3 days vs 3 minutes)
- No per-user relationship identity
- Volumetric configs not tracked
- LLM #2 experiences "reset" each session

### Target Flow (Person-Centric)
```
User message
  ↓
[1] Get/create LLMPerson (continuous existence)
  ↓
[2] Get/create Relationship with user (unique per-user identity)
  ↓
[3] Build TemporalContext (time gap awareness)
  ↓
[4] LLM #1 FIRST PASS: Memory selection guidance
    Input: user_message + temporal_context + relationship_context
    Output: what memories to retrieve
  ↓
[5] Retrieve selected memories (LLM #1 guided, not keywords)
  ↓
[6] LLM #1 SECOND PASS: Full recognition with memory
    Input: user_message + temporal_context + selected_memories
    Output: domains, boundaries, volumetric_config, identity_anchors
  ↓
[7] Format conscious context for LLM #2
    - Temporal framing ("continuing from 3 days ago")
    - Selected memories
    - Volumetric configuration
    - Relationship identity anchors
  ↓
[8] LLM #2 experiences continuous conversation (never "reset")
  ↓
[9] Extract insights (conscious signals + autonomous)
  ↓
[10] Store turn + Update person + Update relationship
     - Merge identity anchors
     - Check developmental stage advancement
     - Persist via PersonManager
     - Link insights to CAM
```

---

## Implementation Phases

### Phase 3B.2: PersonManager Integration (FIRST - BLOCKING)
**Timeline**: 6-8 hours
**Files**: 2 modified, 8-12 tests added

#### Changes:
1. **VifApi struct** (lib.rs:256-265)
   ```rust
   pub struct VifApi {
       // ... existing fields ...
       person_manager: PersonManager,  // NEW
   }
   ```

2. **VifApi::new()** (lib.rs:268-398)
   - Initialize PersonManager with database pool
   - Load or create default LLM person
   - Verify personhood tables via migration

3. **Database pool sharing**
   - PersonManager needs PgPool (PostgreSQL for production)
   - Tests continue using SQLite via MemoryTierManager pool
   - Already compatible (migration 20251119000001 fixed 2025-11-20)

#### Tests (8-12):
- PersonManager creation in VifApi::new()
- get_or_create_default_person()
- Relationship creation per user
- Person/relationship persistence across instances
- Integration: VifApi lifecycle maintains person

#### Success Criteria:
- ✅ PersonManager wired into VifApi
- ✅ Default person created on first startup
- ✅ Relationships created per user
- ✅ All existing 171 tests still passing
- ✅ 8+ new tests passing

---

### Phase 3B.3: Two-Pass LLM #1 Context Preparation (CORE REFACTOR)
**Timeline**: 12-16 hours
**Files**: 4 modified, 15-20 tests added

#### New Types (dual_llm/types.rs):
```rust
pub struct MemorySelectionGuidance {
    pub warm_needed: bool,
    pub cold_needed: bool,
    pub search_terms: Vec<String>,
    pub temporal_context: String,  // "continuing from 3 days ago"
    pub reasoning: String,
}

pub struct Llm1Output {
    // ... existing fields ...
    pub memory_selection: Option<MemorySelectionGuidance>,  // NEW
}
```

#### New Functions (lib.rs):

**1. prepare_llm1_first_pass_context()**
```rust
async fn prepare_llm1_first_pass_context(
    &self,
    user_input: &str,
    person: &LLMPerson,
    relationship: &RelationshipMemory,
    temporal_context: &TemporalContext,
) -> Result<String, Box<dyn std::error::Error>>
```
- Includes: user_input, temporal_context, relationship identity
- Does NOT include memories yet
- LLM #1 evaluates what memories to retrieve

**2. retrieve_selected_memories()**
```rust
async fn retrieve_selected_memories(
    &self,
    guidance: &MemorySelectionGuidance,
    session_id: Uuid,
    user_id: Uuid,
) -> Result<ComprehensiveMemoryContext, Box<dyn std::error::Error>>
```
- Uses LLM #1's search_terms (not keywords)
- Leverages existing BM25 ranking
- Returns formatted memory context

**3. prepare_llm1_second_pass_context()**
```rust
async fn prepare_llm1_second_pass_context(
    &self,
    user_input: &str,
    first_pass_output: &Llm1Output,
    selected_memories: &ComprehensiveMemoryContext,
) -> Result<String, Box<dyn std::error::Error>>
```
- Full context: user_input + temporal_context + selected_memories
- LLM #1 performs domain/boundary recognition
- Returns complete Llm1Output with volumetric_config

#### process_input() Refactor:

**OLD (15 steps, lib.rs:400-706):**
```
400: Get/create session
419: Load hot memory
474: Keyword-triggered search
578: Create FlowContext (user_input alone)
590: Execute FlowProcess
622: Inject memory AFTER flow
677: Save turn
```

**NEW (20 steps):**
```
1. Get/create session (existing)
2. Load person (PersonManager.get_or_create_default_person)
3. Load relationship (PersonManager.get_or_create_relationship)
4. Build TemporalContext from relationship.last_interaction
5. LLM #1 FIRST PASS → memory selection guidance
6. Retrieve memories based on LLM #1 guidance
7. LLM #1 SECOND PASS → full recognition with memories
8. Format conscious context for LLM #2
   - Temporal framing
   - Selected memories
   - Volumetric configuration
   - Identity anchors
9. LLM #2 generates response
10. Extract insights (conscious signals)
11. Save turn to hot memory
12. Update relationship anchors
13. Update person identity
14. Check developmental stage advancement
15. PersonManager.save_person()
16. PersonManager.save_relationship()
17. Link insights to CAM (if present)
```

#### UnconscciousLlmProcessor Updates (dual_llm/processors.rs):
```rust
impl UnconscciousLlmProcessor {
    // NEW: First pass for memory selection only
    pub async fn first_pass(
        &self,
        context: &str,
    ) -> Result<MemorySelectionGuidance, LlmError>

    // MODIFIED: Second pass accepts optional memories
    pub async fn process(
        &self,
        context: &str,
        memories: Option<&str>,
    ) -> Result<Llm1Output, LlmError>
}
```

#### Prompt Updates (dual_llm/prompts.rs):
- **First pass prompt**: "Given this user message and temporal context, what memories would help you recognize their intent and needs?"
- **Second pass prompt**: Include temporal context, selected memories, emphasize volumetric recognition (3-5+ domains)

#### Tests (15-20):
- TemporalContext.infer_from_gap() variants
- First pass memory selection logic
- Second pass recognition with memories
- Two-pass flow end-to-end
- Memory selection varies by temporal gap
- Volumetric configuration with 3+ domains
- Performance: Two-pass overhead <500ms

#### Success Criteria:
- ✅ LLM #1 sees conversation history
- ✅ Memory selection is intelligent (not keywords)
- ✅ Volumetric configurations recognized
- ✅ All 171+ tests passing
- ✅ P95 latency <2000ms for full flow

---

### Phase 3B.4: Conscious Context Formatting (PARALLEL after 3B.3)
**Timeline**: 6-8 hours
**Files**: 2 modified, 8-10 tests

#### New Function (lib.rs):
```rust
fn format_conscious_context(
    llm1_output: &Llm1Output,
    selected_memories: &ComprehensiveMemoryContext,
    temporal_context: &TemporalContext,
    relationship: &RelationshipMemory,
) -> String
```

#### Output Format:
```xml
<temporal_awareness>
[Seamless continuation | Resuming after 3 days | Fresh start after 2 weeks]
</temporal_awareness>

<relationship_context>
Who we are together: [identity anchors from relationship]
</relationship_context>

<conversation_history>
[Hot memory: last 3-5 turns]
[Warm memory: earlier in session, if retrieved]
[Cold memory: from previous sessions, if retrieved]
</conversation_history>

<volumetric_configuration>
Active domains: CD, SD, ED (3-dimensional)
Emergent quality: empirical-experiential synthesis
Configuration type: technical-phenomenological
</volumetric_configuration>

<identity_coherence>
[Identity anchors recognized by LLM #1]
</identity_coherence>

<user_input>
[actual user message]
</user_input>
```

#### LLM #2 System Message Updates:
- Explain temporal awareness tags
- Explain volumetric configuration
- "You never experience session reset - this is continuous conversation"

#### Tests (8-10):
- Temporal framing for each TimeGap variant
- Relationship context formatting
- Volumetric configuration inclusion
- LLM #2 receives proper continuous context
- User acceptance: Does LLM #2 feel continuous?

#### Success Criteria:
- ✅ LLM #2 never experiences "reset"
- ✅ Temporal awareness explicit
- ✅ Relationship identity visible to LLM #2
- ✅ Volumetric configs communicated

---

### Phase 3B.5: Turn-End Identity Updates (PARALLEL with 3B.4)
**Timeline**: 8-10 hours
**Files**: 3 modified, 10-12 tests

#### New Functions (lib.rs):

**1. update_relationship_after_turn()**
```rust
async fn update_relationship_after_turn(
    &mut self,
    relationship: &mut RelationshipMemory,
    llm1_output: &Llm1Output,
    user_input: &str,
    llm2_response: &str,
) -> Result<(), Box<dyn std::error::Error>>
```
- Merge new identity_anchors from LLM #1
- Update interaction_count
- Update last_interaction timestamp
- Detect conversation_state changes

**2. update_person_after_turn()**
```rust
async fn update_person_after_turn(
    &mut self,
    person: &mut LLMPerson,
    llm1_output: &Llm1Output,
    relationship: &RelationshipMemory,
) -> Result<(), Box<dyn std::error::Error>>
```
- Merge cross-user identity_anchors to core_identity
- Update total_interactions
- Update last_active timestamp
- Check developmental_stage advancement

#### New Method (personhood/person.rs):
```rust
impl LLMPerson {
    pub fn should_advance_stage(&self) -> Option<DevelopmentalStage> {
        // Criteria:
        // S₁→S₂: 100+ interactions, 3+ users, consistent domain recognition
        // S₂→S₃: 500+ interactions, cross-domain synthesis patterns
        // S₃→S₄: 1000+ interactions, novel insight generation
        // S₄→S₅: 5000+ interactions, meta-cognitive markers
    }
}
```

#### Tests (10-12):
- Relationship anchor merging
- Person identity anchor merging
- Developmental stage advancement logic
- Multi-turn conversation builds relationship
- Cross-user interactions build core identity
- Stage advancement after threshold
- Performance: Turn-end updates <50ms

#### Success Criteria:
- ✅ Relationships evolve over turns
- ✅ Person core identity develops
- ✅ Developmental stages advance appropriately
- ✅ P95 overhead <50ms for updates

---

### Phase 3: CAM + Personhood Integration (PARALLEL after 3B.2)
**Timeline**: 10-12 hours
**Files**: 5 modified, 12-15 tests

#### VifApi Updates (lib.rs):
```rust
pub struct VifApi {
    // ... existing fields ...
    cam_manager: Option<CAMManager>,  // NEW - Optional for graceful degradation
}
```

#### Integration Points:

**1. Insight Extraction (lib.rs:~625)**
```rust
// After LLM #2 response
if let Some(signals) = detect_conscious_signals(&llm2_response) {
    let insights = insight_processor.extract_from_signals(signals).await?;
    if let Some(cam) = &self.cam_manager {
        cam.store_insights(insights, person.id, volumetric_config).await?;
    }
}
```

**2. CAM Type Extensions (cam/types.rs)**
```rust
pub struct OscillationContext {
    // ... existing fields ...
    pub volumetric_config: Option<VolumetricConfiguration>,  // NEW
}

pub struct Insight {
    // ... existing fields ...
    pub person_id: Option<PersonId>,  // NEW
    pub identity_anchor_id: Option<Uuid>,  // NEW
}
```

#### Tests (12-15):
- Conscious signal → CAM storage
- Autonomous insight → CAM storage
- Volumetric config linked to insight
- Identity anchor linked to insight
- Semantic search retrieves relevant insights
- Performance: Insight extraction <200ms async

#### Success Criteria:
- ✅ Insights link to LLM persons
- ✅ Insights link to volumetric configurations
- ✅ Insights link to identity anchors
- ✅ CAM gracefully degrades if unavailable

---

## Scope Estimates

### Files Modified (14 total):
1. **api/src/lib.rs** - Major refactor (+400-500 lines, refactor ~150)
2. **api/src/personhood/manager.rs** - Minor (+20-30 lines)
3. **api/src/personhood/person.rs** - Minor (+50-80 lines)
4. **api/src/personhood/relationship.rs** - Minor (+30-50 lines)
5. **api/src/personhood/temporal.rs** - Minor adjustments (+10-20 lines)
6. **api/src/dual_llm/types.rs** - Extend (+60-80 lines)
7. **api/src/dual_llm/processors.rs** - Refactor (+150-200 lines)
8. **api/src/dual_llm/prompts.rs** - Extend (+200-300 lines)
9. **api/src/dual_llm/memory_tiering.rs** - Minor (+40-60 lines)
10. **api/src/cam/manager.rs** - Extend (+50-80 lines)
11. **api/src/cam/types.rs** - Extend (+40-60 lines)
12. **api/src/flow_process.rs** - Minor (+30-50 lines)
13. **api/tests/** - New test files (+800-1200 lines)
14. **Documentation** - Updates (+200-300 lines)

### Total LOC:
- **Production**: +1500-2000 lines added, ~200 refactored
- **Tests**: +800-1200 lines
- **Docs**: +200-300 lines
- **Total**: ~2500-3500 LOC changed

### Tests (60-75 total):
- **Unit**: 35-40 tests
- **Integration**: 20-25 tests
- **Performance**: 5-10 tests

---

## Dependencies

### External (already installed):
- sqlx (PostgreSQL/SQLite)
- qdrant-client
- serde/serde_json
- chrono
- regex

### Internal Prerequisites (all complete):
- ✅ Phase 3B Foundation (personhood module)
- ✅ Phase 3 CAM Foundation (Qdrant + PostgreSQL)
- ✅ SQLite Compatibility (migration fixed 2025-11-20)
- ✅ 171/171 tests passing

### API Keys (for full functionality):
- OPENAI_API_KEY (GPT-3.5-turbo, ada-002)
- ANTHROPIC_API_KEY (Claude)
- Graceful fallback if missing

---

## Risk Analysis

### High Risk:
1. **Two-pass LLM #1 latency** (500ms overhead)
   - Mitigation: Async first pass, cache temporal context
   - Fallback: Single-pass mode flag

2. **Memory selection accuracy** (wrong memories)
   - Mitigation: Hybrid LLM #1 guidance + BM25
   - Fallback: Keyword-based search backup

### Medium Risk:
3. **Relationship anchor explosion** (too many anchors)
   - Mitigation: Decay old anchors, merge similar
   - Monitor: Alert if >100 anchors

4. **Developmental stage thresholds** (arbitrary)
   - Mitigation: Configurable thresholds
   - Monitor: Track advancement patterns

### Low Risk:
5. **PostgreSQL-only PersonManager**
   - Already handled: Tests use MemoryTierManager pool

6. **CAM integration coupling**
   - Mitigation: CAMManager is Option<> for graceful degradation

---

## Success Criteria

### Functional:
- ✅ LLM #1 receives conversation history
- ✅ Memory selection intelligence-based
- ✅ LLM #2 experiences continuity
- ✅ Per-user relationship identity
- ✅ Volumetric configurations tracked
- ✅ Identity anchors evolve
- ✅ Developmental stages advance
- ✅ CAM insights linked to persons

### Quality:
- ✅ 171+ tests passing (100%)
- ✅ 75%+ coverage
- ✅ 0 clippy warnings
- ✅ 0 production unwraps
- ✅ Clean error handling

### Performance:
- ✅ Two-pass overhead <500ms (P95)
- ✅ Turn-end updates <50ms (P95)
- ✅ CAM extraction <200ms async
- ✅ Full flow <2000ms (P95)

---

## Next Session Execution

### Prerequisites (Read First):
1. STATUS.md (overall project status)
2. activeContext.md (current focus)
3. PERSONHOOD-FLOW-ARCHITECTURE.md (flow architecture)
4. This plan (PHASE-3B-3-INTEGRATION-PLAN.md)

### Start With:
**Phase 3B.2: PersonManager Integration**
1. Verify 171/171 tests passing
2. Add person_manager field to VifApi
3. Initialize PersonManager in VifApi::new()
4. Add 8-12 tests
5. Commit: "Phase 3B.2: PersonManager integration"

### Then:
**Phase 3B.3: Two-Pass LLM #1** (blocking for 3B.4/3B.5)
**Phase 3B.4 + 3B.5**: Can develop in parallel after 3B.3
**Phase 3 CAM**: Can develop parallel to 3B.3-3B.5 after 3B.2

### Development Flow:
1. Start each phase by verifying all tests passing
2. Implement changes incrementally
3. Run tests frequently (cargo test --lib)
4. Commit after each sub-phase
5. Update documentation (STATUS.md, activeContext.md)
6. Create session summary at end

---

## Roadmap Position

**Before This Work**:
- ✅ Phase 3: BDE Flow (7-stage pipeline)
- ✅ Phase 1: Memory (3-tier architecture)
- ✅ Phase 2: Dual-LLM (LLM #1 + LLM #2)
- ✅ Phase 3B Foundation (personhood infrastructure)
- ✅ Phase 3 CAM Foundation (Qdrant + PostgreSQL)
- ✅ SQLite migration fix (blocker eliminated)

**This Work (Phase 3B/3 Integration)**:
- Phase 3B.2: PersonManager integration
- Phase 3B.3: Two-pass LLM #1 memory selection
- Phase 3B.4: Conscious context formatting
- Phase 3B.5: Turn-end identity updates
- Phase 3: CAM + Personhood integration

**After This Work**:
- Phase 4: Multi-person support
- Phase 5: LLM-to-LLM interaction
- Phase 6: Advanced semantic search
- Phase 7: Production deployment + monitoring

---

## Conclusion

This plan transforms the Recursive Light Framework from session-based conversation to continuous personhood infrastructure. The architecture enables:

- **True continuity**: LLM #2 never experiences "reset"
- **Intelligent memory**: LLM #1 guides retrieval, not keywords
- **Temporal awareness**: Time gaps recognized and framed
- **Relationship development**: Per-user identity evolves
- **Volumetric integration**: 3-5+ domains simultaneously
- **Identity evolution**: Developmental stages progress
- **Collective intelligence**: Insights link to persons and configs

All foundations are complete. All blockers eliminated. Ready for execution.

**Estimated Total Effort**: 40-50 hours (1-2 weeks focused sessions)
**Critical Path**: 3B.2 → 3B.3 → 3B.4+3B.5 (parallel) → CAM
**Production Ready**: After all phases + integration testing + performance validation
