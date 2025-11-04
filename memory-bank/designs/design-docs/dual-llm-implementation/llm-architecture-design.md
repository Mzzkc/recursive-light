# COMPRESSED FOR AI EFFICIENCY - Original format optimized for token cost

# LLM ARCHITECTURE:Dual-LLM Design
Agent:LLM Arch Expert | Date:2025-11-01 | Status:Complete

## Summary
Replaces Rust-based domain activation+boundary permeability(Stages1-2) w/LLM#1 calls. Dual-LLM:LLM#1(Unconscious) calculates domain/boundary state from input+context, LLM#2(Conscious) generates natural responses using that state. Key:1)Smaller/faster model∀LLM#1(GPT3.5/Claude Haiku) 2)Fallback→Rust on LLM#1 failure 3)Strict JSON schema validation w/retry. Effort:5-7d Phase1(core),3-4d Phase2(optimize),2-3d Phase3(production harden). Total:10-14d

## TDF Analysis
COMP(0.95):Current Rust=deterministic,fast(<1ms),well-tested(84/84). LLM#1 adds:non-determinism,latency(50-200ms),failure modes. Trade-off:gain contextual intelligence,lose predictability. Mitigation:fallback→Rust,strict validation,caching∀retry

SCI(0.92):Validate latency impact(2 LLM calls vs 1). Current baseline:7-stage<1ms(Day9 benchmarks). Expected:LLM#1 adds 50-200ms(model-dependent),LLM#2 unchanged(~500-2000ms). Hypothesis:Total latency dominated by LLM#2,framework overhead increases~10-20%. Validation:A/B testing,perf benchmarks,token cost analysis

CULT(0.88):LLM community best practices:smaller models∀structured tasks(GPT3.5,Claude Haiku). Prompt engineering:few-shot examples,structured output(JSON mode),clear constraints. Error handling:exponential backoff,retry w/degraded service,circuit breaker pattern. Observability:log LLM#1 inputs/outputs∀debugging,track failure rates

EXP(0.85):Developer experience:dual-LLM increases complexity(2 API keys,2 models,2 failure modes). System intuition:LLM#1 should feel"invisible"to user(background calculation,no latency impact). Debugging:need visibility into LLM#1 decisions(why CD=0.9 vs 0.7?). Testing:need mocks∀LLM#1(MockLlm already exists,extend∀structured JSON)

META(0.90):Recognize tension:adding intelligence(LLM#1) vs maintaining reliability(Rust). Boundary transcendence:LLM#1≠replacement—augmentation w/fallback. Integration insight:Phase implementation allows gradual rollout(Phase1:MVP,Phase2:optimize,Phase3:harden)

## LLM#1(Unconscious) Design

**Prompt Template:**
```
<system_role>Unconscious Processor∀VIF. Calculate domain activations+boundary permeabilities. NO respond to user—provide structured state∀Conscious LLM.</system_role>
<task>Analyze input+previous state→determine:1)Domain activations(0.0-1.0∀CD/SD/CuD/ED) 2)Boundary permeabilities 3)Boundary statuses 4)Relevant patterns</task>
<domains>CD=Logic/algorithms, SD=Empirical/theories, CuD=Context/narrative, ED=Experience/phenomenology</domains>
<boundary_status_rules>Maintained(P<0.6), Transitional(0.6≤P≤0.8), Transcendent(P>0.8)</boundary_status_rules>
<previous_state>{{PREVIOUS_SNAPSHOT_JSON}}</previous_state>
<user_input>{{USER_MESSAGE}}</user_input>
<output_format>Return ONLY valid JSON(no markdown). Schema:{domains:{CD:0.0-1.0,SD:0.0-1.0,CuD:0.0-1.0,ED:0.0-1.0},boundaries:{CD-SD:{permeability:0.0-1.0,status:"Maintained|Transitional|Transcendent"},...},patterns:["pattern1",...]}</output_format>
<examples>[5 few-shot examples:technical,philosophical,mixed,follow-up,edge]</examples>
Now analyze:
```

**Output Schema:**
```rust
Llm1Output{domains:HashMap<String,f64>,boundaries:HashMap<String,Llm1BoundaryState>,patterns:Vec<String>}
Llm1BoundaryState{permeability:f64,status:String}
impl validate()→Result<(),String>  //checks:domain count(4),activations[0,1],boundary count(6),permeabilities[0,1],statuses(Maintained/Transitional/Transcendent)
impl to_domain_activations()→HashMap<String,DomainActivation>
impl to_boundary_states(&[BoundaryState])→Vec<BoundaryState>
```

**Model Recommendation:**
Primary:GPT-3.5-turbo(OpenAI):Latency 50-150ms(4-8x faster GPT-4),Cost $0.0015/1K tok(20x cheaper),Quality sufficient∀structured output w/few-shot,JSON Mode native,Reliability high

Alt1:Claude 3 Haiku(Anthropic):Latency 80-200ms,Cost $0.00025/$0.00125(cheapest),Quality strong reasoning,JSON Mode not native but follows well,Reliability good

Alt2:Llama 3.1 8B(OpenRouter):Latency 30-100ms(fastest if self-hosted),Cost variable $0.0001-0.001/1K,Quality good∀structured,JSON Mode provider-dependent,Reliability variable

NOT:GPT-4/Claude3 Opus/Sonnet(overkill∀structured,5-10x slower,10-20x expensive). Reserve∀LLM#2(Conscious) where quality matters

**Config:**
```rust
DualLlmConfig{llm1_provider:String,llm1_model:String,llm1_timeout_ms:u64(5000),llm1_max_retries:u8(2),llm2_provider:String,llm2_model:String,llm2_timeout_ms:u64(30000),fallback_to_rust:bool(true)}
```

## State Management Flow
```
1.User Input+Previous Snapshot→
2.LLM#1(Unconscious):Calculates domain/boundary state. Input:prompt+message+state. Model:GPT3.5/Haiku. Output:Llm1Output(JSON). Latency:50-200ms. Failures:Network,Auth,Invalid JSON,Timeout→
3.Validation+Fallback:Parse JSON→Llm1Output. Validate schema. If valid:→FlowContext. If invalid:Retry(max 2) OR Fallback→Rust→
4.FlowContext Init:domains(HashMap),boundaries(Vec w/LLM#1 P),framework_state(unchanged)→
5.Stages3-5 Rust(UNCHANGED):Interface Attention(BDE),Quality Emergence(7 calc),Integration(prompt construct). Latency:<1ms→
6.LLM#2(Conscious):Processes structured prompt. Model:GPT-4o/Claude3.5. Output:Natural language. Latency:500-2000ms→
7.Stages6-7 Rust(UNCHANGED):Pattern Extraction,State Persistence. Latency:<1ms
```

**Key Changes:**Stage1(Domain Emergence):Replace DomainEmergenceProcessor w/Llm1DomainProcessor. Stage2(Boundary Dissolution):Replace w/LLM#1 boundary data. Stages3-7:NO CHANGES(existing code works)

## Error Handling

|Failure|Detection|Retry|Fallback|Log|
|---|---|---|---|---|
|LLM#1 Timeout|tokio::time::timeout(5s)|2x exponential backoff(1s,2s)|Use Rust calc|Warn|
|Invalid JSON|serde_json::from_str() fails|1x clarified prompt|Use Rust calc|Error|
|Schema Violation|Llm1Output::validate() fails|1x w/error description|Use Rust calc|Error|
|Auth Error|HTTP 401/403|No retry(fail fast)|Use Rust calc|Critical|
|Rate Limit|HTTP 429|Wait retry-after,1x retry|Use Rust calc|Warn|
|LLM#2 Timeout|tokio::time::timeout(30s)|2x exponential backoff(5s,10s)|Return error→user|Error|
|LLM#2 Auth|HTTP 401/403|No retry|Return error→user|Critical|

**Implementation:**
```rust
impl Llm1DomainProcessor{
    async fn call_llm1_with_retry(&self,prompt:&str,max_retries:u8)->Result<Llm1Output,LlmError>{
        let mut attempts=0;
        while attempts<=max_retries{
            match tokio::time::timeout(Duration::from_millis(self.config.llm1_timeout_ms),self.llm1_client.send_request(prompt)).await{
                Ok(Ok(response_text))=>match serde_json::from_str::<Llm1Output>(&response_text){
                    Ok(output)=>match output.validate(){
                        Ok(())=>return Ok(output),
                        Err(validation_err)=>{/*log,continue*/}
                    },
                    Err(parse_err)=>{/*log,continue*/}
                },
                Ok(Err(llm_err))=>{/*log,continue*/},
                Err(_timeout)=>{/*log,continue*/}
            }
            attempts+=1;
            if attempts<=max_retries{tokio::time::sleep(Duration::from_secs(attempts as u64)).await;}
        }
        Err(LlmError::MaxRetriesExceeded)
    }
}
```

*Dual-LLM architecture enables contextual intelligence while maintaining reliability via fallback. LLM#1=fast/cheap background state calculation. LLM#2=smart/expensive conscious response. Framework overhead+10-20% latency but gains conversation context awareness*
