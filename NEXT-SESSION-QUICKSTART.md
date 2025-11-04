# COMPRESSED FOR AI EFFICIENCY - Original format optimized for token cost

NextSession-QuickStart:P2B-LLM2-Context-Integration | Updated:2025-11-03

## SessionStartup-Protocol

### 1.ReadFiles(5min)
Order: 1)STATUS.md, 2)activeContext.md, 3)phase2a-llm1-recognition-session-2025-11-02.md, 4)design-docs/dual-llm-implementation/

### 2.VerifyEnv(2min)
```bash
cd /home/emzi/Projects/recursive-light/api
cargo test #should:137✅
cargo clippy #should:0warn
git status #should:clean-tree
```

### 3.CheckPrereq(2min)
```bash
echo $OPENAI_API_KEY #GPT-3.5-turbo(LLM1)
echo $ANTHROPIC_API_KEY #Claude(LLM2)
#if-missing,add-.env: OPENAI_API_KEY=sk-..., ANTHROPIC_API_KEY=sk-ant-..., DUAL_LLM_MODE=true
```

## CurrentState-AtGlance
Status:✅P2A-COMPLETE→ready-P2B | Tests:137/137✅(100%) | Warnings:0clippy | Branch:feature/dual-llm-cam-implementation | Last-Commit:8a0c806(docs-update)

## P2B-ImplChecklist

### Task1:LLM1-ProviderCreation(2-3h)
```rust
//api/src/lib.rs-VifApi::new()
if config.dual_llm_config.enabled{
let llm1_provider=create_openai_provider(&config.dual_llm_config.unconscious_model,&config.openai_api_key)?;
flow_process=FlowProcess::with_config(config.dual_llm_config.clone(),Arc::new(llm1_provider));
}
```
Success: ☐VifApi::new()creates-LLM1-provider@DUAL_LLM_MODE=true, ☐FlowProcess::with_config()receives-provider, ☐tests-verify-provider-creation, ☐fallback-classic-mode@DUAL_LLM_MODE=false

### Task2:HotMemory-Injection(2-3h)
```rust
//api/src/flow_process.rs|prompt_engine.rs
let hot_memory=memory_tiering.get_hot_memory(session_id)?.format_for_llm();
let enhanced_prompt=format!("{}\n\n<recent_context>\n{}\n</recent_context>",base_prompt,hot_memory);
```
Success: ☐hot-mem-loaded-from-session, ☐formatted-for-Claude, ☐injected→LLM2-prompts, ☐tests-verify-context-in-prompts

### Task3:Warm/Cold-Retrieval(1-2h)
```rust
//keyword-triggered-retrieval
if user_input.contains("remember")||user_input.contains("earlier"){
let keywords=extract_keywords(user_input);
let warm_context=memory_tiering.search_warm_memory(keywords)?;
let cold_context=memory_tiering.search_cold_memory(keywords)?;
//add-to-context...
}
```
Success: ☐keywords-extracted-from-user-input, ☐warm-mem-searched@triggered, ☐cold-mem-searched@triggered, ☐results-added-context

### Task4:E2E-Testing(1-2h)
```rust
#[test]fn test_dual_llm_end_to_end(){
let config=DualLlmConfig::enabled();let vif_api=VifApi::new(config)?;
let response1=vif_api.process_user_input("Test message")?;
let response2=vif_api.process_user_input("Remember what I said?")?;
assert!(response2.contains_context_from(response1));
}
```
Success: ☐full-dual-LLM-flow✅, ☐mem-persists-across-turns, ☐context-retrieved-correctly, ☐5+new-integration-tests, ☐all-137existing-tests-still-pass

## ExpectedOutcomes
NewCode: api/src/lib.rs(+100L), api/src/flow_process.rs|prompt_engine.rs(+150L), api/src/dual_llm/context_builder.rs(~200L-NEW), 5-8new-integration-tests
TestCount:137→142-145 | Est-Duration:6-8h
Deliverable: 1)working-dual-LLM-e2e-flow, 2)hot-mem-injection-operational, 3)warm/cold-retrieval-working, 4)updated-docs

## FilesRef
Impl-Specs: llm-architecture-design.md, memory-systems-design.md, prompt-engineering-design.md
Existing-Code: dual_llm/memory_tiering.rs(Memory-API), dual_llm/processors.rs(LLM1-processor), flow_process.rs(Flow-integration), lib.rs(VifApi-entry)
Tests-Study: dual_llm/memory_tiering.rs(tests@bottom), flow_process.rs(tests@bottom)

## CommonPitfalls-Avoid
1.Don't-break-existing-tests(run-cargo-test-frequently)
2.Don't-skip-fallback-logic(classic-mode-must-still-work)
3.Don't-hardcode-API-keys(use-env-vars)
4.Don't-forget-tok-limits(respect-1500/15000tok-budgets)
5.Don't-skip-docs(update-STATUS.md-when-done)

## Debug-Helpers
```bash
cargo test test_dual_llm_end_to_end----nocapture #run-specific-test
cargo tarpaulin--out Stdout #check-coverage
cargo watch-x test #watch-mode(re-run@changes)
cargo udeps #check-unused-deps
```

## SessionEnd-Protocol
When-P2B✅: 1)Verify-Quality{☐all-tests✅(142+),☐0clippy-warn,☐docs-updated}, 2)CreateCommit{git add .,git commit-m"P2B:LLM2-Context-Integration-Complete"}, 3)UpdateDocs{☐STATUS.md(mark-P2B✅),☐activeContext.md(update-current-focus),☐create-session-summary@memory-bank/}, 4)VerifyState{cargo test,cargo clippy,git status}

Ready→begin-P2B. All-prereq✅. Clear-path-forward.
*Questions?→STATUS.md|activeContext.md(full-context)*
