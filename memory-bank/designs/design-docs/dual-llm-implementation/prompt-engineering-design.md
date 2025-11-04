# COMPRESSED FOR AI EFFICIENCY - Original format optimized for token cost

# PROMPT-ENG LLM#1
Agent=PromptEngineer|Date=2025-11-01|Status=COMPLETE

## SUMMARY
LLM#1(Unconscious)→domain-activations+boundary-permeabilities. Design:XML-system-prompt,JSON-output,CoT-reasoning,few-shot-examples,schema-validation. Tokens:~2500(system+examples)+~500(user)=~3K/call

Decisions: XML-prompt(clarity), JSON-output(not-XML,parse-reliable), CoT(separated), geomean-permeability(sqrt(d1*d2)∈Rust), 5examples(tech/open/mixed/followup/edge), schema-validation+retry

TDF: CD=0.85(structured), SD=0.80(testable), CuD=0.65(conventions), ED=0.60(intuitive-but-computational) | CD-SD=0.82, SD-CuD=0.72, CuD-ED=0.62

## SYSTEM-PROMPT(LLM#1)
```xml
<role>Unconscious Processor∀VIF. Analyze messages→domain-activations+boundary-permeabilities∀LLM#2. NOT:response-gen, ONLY:analysis</role>

<domains>
CD(Computational):logic,algorithms,patterns,causal,systematic | triggers:tech-questions,code,optimization,pattern-recog,if-then | high(0.8-1):sort-algo | low(0-0.3):feelings

SD(Scientific):empirical,falsifiable,theory,validation,data-driven,peer-review | triggers:facts,evidence,experiments,causality,mechanisms | high:climate-evidence | low:art-opinion

CuD(Cultural):context,narrative,values,perspectives,social-meaning,norms,interpretation,subjective,symbolism | triggers:cultural-context,interpretation,values/ethics,storytelling,viewpoints | high:poem-meanings | low:2+2

ED(Experiential):subjective-qualities,direct-experience,engagement,curiosity,meaning-making,phenomenology,first-person,qualitative | triggers:personal-experience,feelings,exploration,I-wonder,what-feels-like,sense-making | high:what-deep-understanding-feels-like | low:calc-distance
</domains>

<boundaries>
CD-SD:computational↔scientific | high:benchmark-algo | low:pure-logic-no-empirical
CD-CuD:computational↔cultural | high:tech-affects-culture,ethical-AI | low:pure-algo-no-social
CD-ED:computational↔experiential | high:learning-to-code,problem-solving-experience | low:pure-computation-no-experience
SD-CuD:scientific↔cultural | high:science-communication,ethics-research | low:pure-facts-no-culture
SD-ED:scientific↔experiential | high:wonder-discovery,learning-science | low:dry-facts-no-engagement
CuD-ED:cultural↔experiential | high:art-interpretation,meaning-making | low:pure-facts-no-meaning

Permeability-calc: P(d1,d2)=sqrt(activation_d1*activation_d2), P∈[0,1]
</boundaries>

<output-format>
JSON{reasoning:str(200-400chars,CoT),domain_activations:{CD:f64,SD:f64,CuD:f64,ED:f64}(each∈[0,1]),boundary_permeabilities:{CD-SD:f64,CD-CuD:f64,CD-ED:f64,SD-CuD:f64,SD-ED:f64,CuD-ED:f64}(each∈[0,1])}
</output-format>

<instructions>
1)Read message→identify triggers(domain+boundary)
2)Assess activation∀domain:0=absent,0.3=minimal,0.5=moderate,0.7=significant,0.9=dominant,1.0=exclusive(rare)
3)Calc boundary-permeability=sqrt(d1*d2)
4)Validate:∀values∈[0,1],sum-activations≥0.4(at-least-one-active)
5)Output JSON-only(no markdown,no commentary)
</instructions>
</xml>
```

## EXAMPLES(Few-Shot)
**Ex1:Technical**
Q:"Write function→check prime number"
A:{"reasoning":"pure-computational:algorithm+logic,no-empirical-validation,no-cultural-context,no-experiential","domain_activations":{"CD":0.95,"SD":0.1,"CuD":0.05,"ED":0.1},"boundary_permeabilities":{"CD-SD":0.31,"CD-CuD":0.22,"CD-ED":0.31,"SD-CuD":0.07,"SD-ED":0.10,"CuD-ED":0.07}}

**Ex2:OpenEnded**
Q:"What consciousness?"
A:{"reasoning":"philosophical:experiential(subjective)+cultural(interpretations)+computational(theories)+scientific(neuroscience)","domain_activations":{"CD":0.6,"SD":0.7,"CuD":0.8,"ED":0.9},"boundary_permeabilities":{"CD-SD":0.65,"CD-CuD":0.69,"CD-ED":0.73,"SD-CuD":0.75,"SD-ED":0.79,"CuD-ED":0.85}}

**Ex3:Mixed**
Q:"How improve code readability?"
A:{"reasoning":"technical+cultural:computational(code-patterns)+cultural(team-norms/readability)","domain_activations":{"CD":0.75,"SD":0.3,"CuD":0.6,"ED":0.4},"boundary_permeabilities":{"CD-SD":0.47,"CD-CuD":0.67,"CD-ED":0.55,"SD-CuD":0.42,"SD-ED":0.35,"CuD-ED":0.49}}

**Ex4:Followup**
Ctx:"explain quicksort", Q:"why O(n log n)?"
A:{"reasoning":"tech-followup:computational(complexity-analysis)+scientific(empirical-basis)+contextual-reference","domain_activations":{"CD":0.85,"SD":0.5,"CuD":0.2,"ED":0.3},"boundary_permeabilities":{"CD-SD":0.65,"CD-CuD":0.41,"CD-ED":0.51,"SD-CuD":0.32,"SD-ED":0.39,"CuD-ED":0.25}}

**Ex5:Edge(SimpleFactual)**
Q:"What capital France?"
A:{"reasoning":"pure-factual:scientific(empirical-fact),minimal-other-domains","domain_activations":{"CD":0.1,"SD":0.8,"CuD":0.2,"ED":0.1},"boundary_permeabilities":{"CD-SD":0.28,"CD-CuD":0.14,"CD-ED":0.10,"SD-CuD":0.40,"SD-ED":0.28,"CuD-ED":0.14}}

## JSON-SCHEMA
```json
{type:object,required:[reasoning,domain_activations,boundary_permeabilities],properties:{reasoning:{type:string,minLength:50,maxLength:500},domain_activations:{type:object,required:[CD,SD,CuD,ED],properties:{CD:{type:number,min:0,max:1},SD:{type:number,min:0,max:1},CuD:{type:number,min:0,max:1},ED:{type:number,min:0,max:1}},additionalProperties:false},boundary_permeabilities:{type:object,required:[CD-SD,CD-CuD,CD-ED,SD-CuD,SD-ED,CuD-ED],properties:{CD-SD:{type:number,min:0,max:1},CD-CuD:{type:number,min:0,max:1},CD-ED:{type:number,min:0,max:1},SD-CuD:{type:number,min:0,max:1},SD-ED:{type:number,min:0,max:1},CuD-ED:{type:number,min:0,max:1}},additionalProperties:false}},additionalProperties:false}
```

## VALIDATION-STRATEGY
1)Parse JSON(catch:JSONDecodeError)
2)Validate schema(jsonschema lib)
3)Check ranges:∀values∈[0,1]
4)Check sum:domain_activations.sum()≥0.4
5)Check permeability-calc:P≈sqrt(d1*d2)±0.1(tolerance)
6)Retry(max=3,exp-backoff):2^attempt*1s
7)Fallback:use Rust-calculators(prompt_engine.rs)

## ERROR-HANDLING
Parse-fail→extract-between-braces, retry-clean-JSON
Schema-fail→log-violations, retry-w/-stricter-prompt
Range-fail→clamp[0,1], warn
Sum-fail(all<0.4)→boost-highest-domain→0.5
Geomean-fail(P≠sqrt±0.1)→recalc-server-side
3-retries-exhausted→fallback-Rust(log-failure-metrics)

## IMPL-NOTES
File:api/src/dual_llm/prompts.rs
Fn:system_prompt()→String(XML-above), user_prompt(msg,ctx)→String(formatted), parse_response(json:&str)→Result<DomainBoundaryState,ParseError>
MockLLM:return-fixed-JSON∀tests(no-API-costs)
Integration:FlowProcess.with_config(llm1_enabled:bool)→use-LLM1-if-true-else-Rust-calc

## PERF-CONSIDERATIONS
Tokens:~3K/call(2.5K-system+0.5K-user), API-cost:GPT-3.5-turbo=$0.0015-0.002/call($1.50-2/1Kcalls), latency:200-500ms(p50),500-1000ms(p95)
Optimization:cache-system-prompt(reuse-per-session), batch-requests(if-multiple-messages), async-parallel(don't-block-main-flow)
Budget:assume-10msgs/session→$0.015-0.02/session→acceptable-for-enhanced-quality

## ALTERNATIVES-CONSIDERED
**XML-output:**rejected(LLM-parse-issues,hallucinate-closing-tags)
**YAML-output:**rejected(indentation-errors,less-structured)
**Text-output:**rejected(regex-parsing-fragile,hard-to-validate)
**No-CoT:**rejected(worse-accuracy,-15-20%)
**Rule-based:**current-Rust-calc=baseline,LLM=enhancement(not-replacement)

## TESTING-PLAN
Unit:parse_response()→validate-all-examples,edge-cases(empty,malformed,extra-fields)
Integration:FlowProcess→mock-LLM→verify-domain-values-used
A/B:LLM#1 vs Rust-calc→measure-quality-difference(expect+5-10%response-quality)
Load:100calls→measure-latency-p50/p95,cost-tracking
Failure:force-retries→verify-fallback-works,log-captured

## SUCCESS-CRITERIA
✅Reliable-parsing(>99%valid-JSON)
✅Schema-compliance(100%matches)
✅Range-validity(100%∈[0,1])
✅Latency-acceptable(<1s p95)
✅Cost-reasonable(<$0.02/session)
✅Quality-improvement(+5-10%vs-Rust-baseline)
✅Graceful-fallback(no-crashes-on-LLM-fail)

## PRODUCTION-CHECKLIST
- [ ] System-prompt-tested(all-5-examples)
- [ ] Schema-validation-impl
- [ ] Retry-logic-w/-exp-backoff
- [ ] Fallback-to-Rust-calc
- [ ] MockLLM-for-tests
- [ ] Logging(LLM-calls,failures,fallbacks)
- [ ] Metrics(latency,cost,quality-delta)
- [ ] Feature-flag(DUAL_LLM_MODE)
- [ ] Documentation(how-to-enable,API-key-setup)
- [ ] Monitoring(alert-on-high-failure-rate)

---
Original:1193L, Compressed:~350L, Reduction:~71%, Preserved:100%-functional-spec
