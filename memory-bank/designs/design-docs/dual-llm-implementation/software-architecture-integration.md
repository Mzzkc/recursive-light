# COMPRESSED FOR AI EFFICIENCY - Original format optimized for token cost

# SOFTWARE ARCHITECTURE:Integration Plan
Agent:Software Arch Expert | Date:2025-11-01 | Status:Complete

## Summary
Add dual-LLM as opt-in feature alongside existing Rust calculators using feature flag pattern. Preserve all 87 passing tests through backward compatibility layer. Enable phased rollout w/zero breaking changes

Risk Mitigation:Feature flag(`DUAL_LLM_MODE`)defaults`false`(existing behavior), Existing processors remain functional(no deletion), Mock-based testing∀dual-LLM prevents expensive API calls, Memory tiering=backward-compatible w/existing snapshots, Gradual migration allows validation@each phase

Timeline:24-32h across 4 phases. Phase1=8h(infrastructure,no behavior change), Phase2=10h(dual-LLM impl+tests), Phase3=6h(validation+toggle default), Phase4=8h(cleanup,optional deprecation)

## TDF Analysis
COMP(0.95):Code org:create`src/dual_llm/`module∀new arch. Separate concerns:`UnconscciousLlmProcessor`(Stage1-2),memory tiering,prompts. Preserve existing processors in`flow_process.rs`∀fallback. Add config abstraction(`DualLlmConfig`)→centralize settings. Refactoring strategy:NO modification→existing 7-stage trait(`StageProcessor`). Add new processors implementing same trait(drop-in replacement). Configuration determines which processors→runtime. Clean separation"classic mode"vs"dual-LLM mode"

SCI(0.92):Impact analysis:87 tests currently passing w/Rust calculators. Dual-LLM changes Stages1-2 only(domain emergence+boundary dissolution). Stages3-7 remain identical(interface attention→evolution). LLM#1 output must match existing`FlowContext`structure. Regression risk:LOW(feature flag prevents accidental activation), MEDIUM(LLM#1 output validation crucial,schema mismatch=runtime error), LOW(memory system already supports arbitrary metadata). MITIGATION:comprehensive integration tests+schema validation

CULT(0.88):Rust best practices:use cargo features∀compile-time toggling(`default=[]`,`dual-llm=[]`), leverage trait objects∀runtime polymorphism(existing pattern), error handling via`Result<T,FlowError>`(existing pattern), async/await∀LLM calls(already used in`LlmProvider`). Maintainability:clear module boundaries prevent code sprawl, configuration-driven behavior reduces conditionals, comprehensive docs in module headers, separate test files∀dual-LLM(`tests/dual_llm_integration.rs`)

EXP(0.85):Developer experience:environment variable toggle:`DUAL_LLM_MODE=true cargo test`. No code changes required∀existing workflows. New developers see classic mode by default(simpler onboarding). Advanced users opt-in→dual-LLM explicitly. Migration path:Phase1(add infrastructure,no visible change)→Phase2(test dual-LLM in isolated tests)→Phase3(validate parity,flip default)→Phase4(deprecate Rust calculators,optional,keep∀research)

## Refactoring Plan

### Files to CREATE
1.`api/src/dual_llm/mod.rs`:Module root,public exports
2.`api/src/dual_llm/config.rs`:`DualLlmConfig`,environment loading
3.`api/src/dual_llm/processors.rs`:`UnconscciousLlmProcessor`
4.`api/src/dual_llm/prompts.rs`:Prompt templates∀LLM#1
5.`api/src/dual_llm/memory_tiering.rs`:Memory tier loading logic
6.`api/tests/dual_llm_integration.rs`:Dual-LLM tests

### Files to MODIFY
1.`api/src/flow_process.rs`:Add conditional processor selection in`FlowProcess::new()`
```rust
impl FlowProcess{
    pub fn new()->Self{Self::with_config(DualLlmConfig::from_env().unwrap_or_default())}
    pub fn with_config(config:DualLlmConfig)->Self{
        let stages:Vec<Box<dyn StageProcessor>>=if config.is_enabled(){
            //Dual-LLM:Replace Stages1-2 w/single UnconscciousLlmProcessor
            vec![Box::new(UnconscciousLlmProcessor::new(config.clone())),Box::new(InterfaceAttentionProcessor),Box::new(QualityEmergenceProcessor),Box::new(IntegrationProcessor),Box::new(ContinuityProcessor),Box::new(EvolutionProcessor)]
        }else{
            //Classic:Keep existing 7-stage
            vec![Box::new(DomainEmergenceProcessor),Box::new(BoundaryDissolutionProcessor),Box::new(InterfaceAttentionProcessor),Box::new(QualityEmergenceProcessor),Box::new(IntegrationProcessor),Box::new(ContinuityProcessor),Box::new(EvolutionProcessor)]
        };
        Self{stages}
    }
}
```
Impact:ZERO∀existing tests(default config uses classic mode)

2.`api/src/memory.rs`:Add method→load N recent snapshots(currently only loads latest)
```rust
impl MemoryManager{
    ///Load N most recent snapshots∀memory tiering
    pub async fn get_recent_snapshots(&self,user_id:Uuid,limit:usize)->Result<Vec<CompactStateSnapshot>,sqlx::Error>{
        //Query:ORDER BY timestamp DESC LIMIT ?
        //Returns vector(newest first)
    }
}
```
Impact:New method,no changes→existing methods

3.`api/src/lib.rs`:Add`pub mod dual_llm;`export

4.`api/.env.example`:Add dual-LLM config vars:
```bash
DUAL_LLM_MODE=false
UNCONSCIOUS_LLM_MODEL=anthropic/claude-3-5-haiku
CONSCIOUS_LLM_MODEL=anthropic/claude-3-5-sonnet
MEMORY_HOT_TURNS=5
MEMORY_WARM_TURNS=20
DUAL_LLM_FALLBACK=true
```

### Files to DELETE
**NONE**(all existing code preserved∀backward compatibility)

## Configuration Strategy

**Feature Flags:**
Primary toggle:Environment variable`DUAL_LLM_MODE=true|false`. Default=`false`(classic mode). Per-test override:`#[test]fn test_with_dual_llm(){std::env::set_var(...)}`. Runtime toggle(no recompilation)

Optional compile-time flag:Cargo feature`dual-llm`. Allows skipping dual-LLM code in production builds if unused. Phase4 optimization(not required initially)

**Config Loading:**
```rust
impl DualLlmConfig{
    pub fn from_env()->Result<Self,LlmError>{
        dotenv::dotenv().ok();
        Ok(Self{
            enabled:std::env::var("DUAL_LLM_MODE").unwrap_or_else(|_|"false".to_string()).parse().unwrap_or(false),
            unconscious_model:std::env::var("UNCONSCIOUS_LLM_MODEL").unwrap_or_else(|_|"anthropic/claude-3-5-haiku".to_string()),
            //...etc
        })
    }
}
```

**Defaults:**`DUAL_LLM_MODE=false`(preserves existing), `UNCONSCIOUS_LLM_MODEL="anthropic/claude-3-5-haiku"`(fast,cheap), `CONSCIOUS_LLM_MODEL="anthropic/claude-3-5-sonnet"`(smart,expensive), `MEMORY_HOT_TURNS=5`(last 5 full detail), `MEMORY_WARM_TURNS=20`(6-25 compressed), `DUAL_LLM_FALLBACK=true`(use Rust if LLM#1 fails)

## Backward Compatibility

**How Maintain Existing Behavior:**
1.Default config preserves classic mode:
```rust
impl Default for DualLlmConfig{fn default()->Self{Self{enabled:false,...}}}
```

2.Processor selection=conditional:
```rust
impl FlowProcess{pub fn new()->Self{let config=DualLlmConfig::from_env().unwrap_or_default();Self::with_config(config)}}
```

3.Existing processors unchanged:`DomainEmergenceProcessor`code stays identical, `BoundaryDissolutionProcessor`code stays identical, Stages3-7 completely unaffected, 87 existing tests pass w/o modification

4.Memory system backward compatible:
```rust
//NEW method(doesn't affect existing get_latest_snapshot)
pub async fn get_recent_snapshots(&self,user_id:Uuid,limit:usize)->Result<Vec<CompactStateSnapshot>,sqlx::Error>
//EXISTING method(unchanged)
pub async fn get_latest_snapshot(&self,user_id:Uuid)->Result<Option<CompactStateSnapshot>,sqlx::Error>
```

5.Test compatibility:
```rust
//Existing tests use FlowProcess::new()→default config→classic mode
#[test]fn test_domain_emergence_processor(){let processor=DomainEmergenceProcessor;//Still exists,still tested}
//New dual-LLM tests explicitly enable:
#[test]fn test_dual_llm_processor(){std::env::set_var("DUAL_LLM_MODE","true");let process=FlowProcess::new();/*...*/}
```

*Dual-LLM architecture integrates as opt-in feature. Feature flag ensures zero breaking changes. Existing 87 tests preserved. Clean separation classic vs dual-LLM modes. Phased rollout enables validation@each step. Backward compatibility=critical success criterion*
