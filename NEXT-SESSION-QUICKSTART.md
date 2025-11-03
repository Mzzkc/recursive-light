# Next Session Quick Start Guide
*For Phase 2B: LLM #2 Context Integration*
*Updated: 2025-11-03*

## Session Startup Protocol

### 1. Read These Files (5 minutes)
```bash
# In order of importance:
1. /home/emzi/Projects/recursive-light/STATUS.md
2. /home/emzi/Projects/recursive-light/memory-bank/activeContext.md
3. /home/emzi/Projects/recursive-light/memory-bank/phase2a-llm1-recognition-session-2025-11-03.md
4. /home/emzi/Projects/recursive-light/design-docs/dual-llm-implementation/
```

### 2. Verify Environment (2 minutes)
```bash
cd /home/emzi/Projects/recursive-light/api

# Should show 137 passing
cargo test

# Should show 0 warnings
cargo clippy

# Should show clean tree
git status
```

### 3. Check Prerequisites (2 minutes)
```bash
# Verify API keys
echo $OPENAI_API_KEY      # For GPT-3.5-turbo (LLM #1)
echo $ANTHROPIC_API_KEY   # For Claude (LLM #2)

# If missing, add to .env:
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...
DUAL_LLM_MODE=true
```

## Current State (At a Glance)

**Status:** ✅ Phase 2A COMPLETE, ready for Phase 2B
**Tests:** 137/137 passing (100%)
**Warnings:** 0 clippy warnings
**Branch:** feature/dual-llm-cam-implementation
**Last Commit:** 8a0c806 (docs update)

## Phase 2B Implementation Checklist

### Task 1: LLM #1 Provider Creation (2-3 hours)
```rust
// In api/src/lib.rs - VifApi::new()

if config.dual_llm_config.enabled {
    // Create GPT-3.5-turbo provider
    let llm1_provider = create_openai_provider(
        &config.dual_llm_config.unconscious_model,
        &config.openai_api_key
    )?;

    // Pass to FlowProcess
    flow_process = FlowProcess::with_config(
        config.dual_llm_config.clone(),
        Arc::new(llm1_provider)
    );
}
```

**Success Criteria:**
- [ ] VifApi::new() creates LLM #1 provider when DUAL_LLM_MODE=true
- [ ] FlowProcess::with_config() receives provider
- [ ] Tests verify provider creation
- [ ] Fallback to classic mode when DUAL_LLM_MODE=false

### Task 2: Hot Memory Injection (2-3 hours)
```rust
// In api/src/flow_process.rs or prompt_engine.rs

// Load hot memory
let hot_memory = memory_tiering
    .get_hot_memory(session_id)?
    .format_for_llm();

// Inject into Claude prompt
let enhanced_prompt = format!(
    "{}\n\n<recent_context>\n{}\n</recent_context>",
    base_prompt,
    hot_memory
);
```

**Success Criteria:**
- [ ] Hot memory loaded from session
- [ ] Formatted appropriately for Claude
- [ ] Injected into LLM #2 prompts
- [ ] Tests verify context appears in prompts

### Task 3: Warm/Cold Retrieval (1-2 hours)
```rust
// Keyword-triggered retrieval
if user_input.contains("remember") || user_input.contains("earlier") {
    let keywords = extract_keywords(user_input);
    let warm_context = memory_tiering.search_warm_memory(keywords)?;
    let cold_context = memory_tiering.search_cold_memory(keywords)?;

    // Add to context...
}
```

**Success Criteria:**
- [ ] Keywords extracted from user input
- [ ] Warm memory searched when triggered
- [ ] Cold memory searched when triggered
- [ ] Results added to context

### Task 4: End-to-End Testing (1-2 hours)
```rust
#[test]
fn test_dual_llm_end_to_end() {
    // Setup
    let config = DualLlmConfig::enabled();
    let vif_api = VifApi::new(config)?;

    // Test conversation flow
    let response1 = vif_api.process_user_input("Test message")?;
    let response2 = vif_api.process_user_input("Remember what I said?")?;

    // Verify
    assert!(response2.contains_context_from(response1));
}
```

**Success Criteria:**
- [ ] Full dual-LLM flow works
- [ ] Memory persists across turns
- [ ] Context retrieved correctly
- [ ] 5+ new integration tests
- [ ] All 137 existing tests still pass

## Expected Session Outcomes

**New Code:**
- Modified: `api/src/lib.rs` (+100 lines)
- Modified: `api/src/flow_process.rs` or `prompt_engine.rs` (+150 lines)
- New: `api/src/dual_llm/context_builder.rs` (~200 lines)
- New tests: 5-8 integration tests

**Test Count:** 137 → 142-145
**Estimated Duration:** 6-8 hours
**Deliverables:**
1. Working dual-LLM end-to-end flow
2. Hot memory injection operational
3. Warm/cold retrieval working
4. Updated documentation

## Files to Reference

### Implementation Specs
- `design-docs/dual-llm-implementation/llm-architecture-design.md`
- `design-docs/dual-llm-implementation/memory-systems-design.md`
- `design-docs/dual-llm-implementation/prompt-engineering-design.md`

### Existing Code
- `api/src/dual_llm/memory_tiering.rs` - Memory API
- `api/src/dual_llm/processors.rs` - LLM #1 processor
- `api/src/flow_process.rs` - Flow integration
- `api/src/lib.rs` - VifApi entry point

### Tests to Study
- `api/src/dual_llm/memory_tiering.rs` (tests at bottom)
- `api/src/flow_process.rs` (tests at bottom)

## Common Pitfalls to Avoid

1. **Don't break existing tests** - Run `cargo test` frequently
2. **Don't skip fallback logic** - Classic mode must still work
3. **Don't hard-code API keys** - Use environment variables
4. **Don't forget token limits** - Respect 1500/15000 token budgets
5. **Don't skip documentation** - Update STATUS.md when done

## Debug Helpers

```bash
# Run specific test
cargo test test_dual_llm_end_to_end -- --nocapture

# Check test coverage
cargo tarpaulin --out Stdout

# Watch mode (re-run on changes)
cargo watch -x test

# Check for unused dependencies
cargo udeps
```

## Session End Protocol

When Phase 2B is complete:

1. **Verify Quality:**
   - [ ] All tests passing (142+)
   - [ ] Zero clippy warnings
   - [ ] Documentation updated

2. **Create Commit:**
   ```bash
   git add .
   git commit -m "Phase 2B: LLM #2 Context Integration Complete"
   ```

3. **Update Documentation:**
   - [ ] STATUS.md (mark Phase 2B complete)
   - [ ] activeContext.md (update current focus)
   - [ ] Create session summary in memory-bank/

4. **Verify State:**
   ```bash
   cargo test
   cargo clippy
   git status
   ```

---

**Ready to begin Phase 2B. All prerequisites complete. Clear path forward.**

*Questions? See STATUS.md or activeContext.md for full context.*
