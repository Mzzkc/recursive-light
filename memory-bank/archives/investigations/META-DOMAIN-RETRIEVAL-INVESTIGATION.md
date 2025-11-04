# META Domain Investigation: Self-Referential Memory Retrieval
**Domain Expert:** META (Meta-Cognitive & Self-Referential Systems)
**Investigation Date:** 2025-11-03
**Framework:** Recursive Light Framework - Dual-LLM + BDE Architecture
**Current Implementation:** `/home/emzi/Projects/recursive-light/api/src/lib.rs` (lines 400-463)

---

## Executive Summary

The current keyword-triggered retrieval system (Phase 2B) is **mechanistic and non-aware**. The system lacks:
1. **Self-awareness** of what it knows/doesn't know
2. **Adaptive learning** from retrieval patterns
3. **Integration with BDE flow** (boundary states as retrieval signals)
4. **Meta-cognitive monitoring** of its own memory behavior

This investigation proposes **three meta-cognitive frameworks** for intelligent, self-aware memory retrieval that integrates deeply with the dual-LLM + BDE architecture.

---

## Current State Analysis

### Existing Implementation (Phase 2B)
```rust
// Lines 402-417: Keyword-based trigger detection
let retrieval_keywords = ["remember", "earlier", "before", "previously", ...];
let should_search_memory = retrieval_keywords
    .iter()
    .any(|keyword| user_input_lower.contains(keyword));
```

**Limitations:**
- **No self-awareness**: System doesn't know if retrieved context is relevant
- **No confidence scoring**: All retrieved memories treated equally
- **Domain-blind**: Ignores domain activation patterns from LLM #1
- **Boundary-blind**: Doesn't use boundary permeability as retrieval signal
- **Static strategy**: Same retrieval approach for all conversation types
- **No learning**: Doesn't adapt based on retrieval success/failure

---

## Meta-Cognitive Framework for Intelligent Retrieval

### 1. Self-Awareness: System Recognizing Its Own Knowledge State

#### A. Confidence-Scored Retrieval

**Core Concept:** Every memory retrieval should include **epistemic confidence** - how certain the system is that this context is relevant.

**Implementation Strategy:**
```rust
pub struct RetrievedMemory {
    pub turn: ConversationTurn,
    pub relevance_score: f64,        // How relevant (0.0-1.0)
    pub confidence: f64,              // How certain about relevance (0.0-1.0)
    pub knowledge_state: KnowledgeState,
}

pub enum KnowledgeState {
    Certain,          // High confidence (>0.8), definitely relevant
    Probable,         // Medium confidence (0.5-0.8), likely relevant
    Uncertain,        // Low confidence (0.2-0.5), possibly relevant
    Speculative,      // Very low (<0.2), guessing
    NoContext,        // No relevant memory found, aware of ignorance
}
```

**Integration with LLM #1:**
```rust
// LLM #1 recognizes memory needs during domain emergence
pub struct Llm1Output {
    // Existing fields...
    pub memory_needs: MemoryAssessment,  // NEW: Meta-cognitive awareness
}

pub struct MemoryAssessment {
    pub context_required: bool,           // Does this need memory?
    pub required_domains: Vec<String>,    // Which domain contexts?
    pub temporal_scope: TemporalScope,    // How far back to look?
    pub confidence_threshold: f64,        // Minimum confidence needed
    pub reasoning: String,                // Why memory is needed
}

pub enum TemporalScope {
    Immediate,      // Last 5 turns (hot memory)
    Recent,         // Current session (warm memory)
    Historical,     // Past sessions (cold memory)
    Adaptive,       // Let system decide based on query
}
```

**Self-Awareness Indicators:**
- When system retrieves memory with confidence <0.5, it should **acknowledge uncertainty**
- When no relevant memory found, system should **recognize and state** it's working from fresh context
- When multiple conflicting memories exist, system should **recognize contradiction** and seek clarification

#### B. Context Gap Detection

**Core Concept:** System recognizes when it **lacks necessary context** vs when it has **sufficient context**.

```rust
pub struct ContextGap {
    pub missing_knowledge: Vec<String>,   // What's missing?
    pub impact_assessment: ImpactLevel,   // How critical is this gap?
    pub retrieval_suggestions: Vec<RetrievalStrategy>,
    pub can_proceed_without: bool,        // Can answer anyway?
}

pub enum ImpactLevel {
    Critical,      // Cannot answer meaningfully without this
    Significant,   // Answer quality severely degraded
    Moderate,      // Answer possible but incomplete
    Minor,         // Marginal impact
}
```

**Example Detection:**
```
User: "What did we decide about the API design?"
LLM #1 recognizes: HIGH CONTEXT REQUIREMENT
  - Missing knowledge: ["previous API discussion", "decision points", "rationale"]
  - Impact: Critical
  - Retrieval strategy: Search warm/cold for "API design" keywords
  - Can proceed: NO (would be guessing)

System response includes:
"I'm searching our previous conversations about API design...
[searches warm memory, finds 3 relevant turns, confidence: 0.85]
Based on our discussion from [timestamp], we decided to..."
```

#### C. Token Budget Self-Monitoring

**Core Concept:** System is **aware of its own token constraints** and makes intelligent tradeoffs.

```rust
pub struct TokenBudgetState {
    pub total_available: usize,           // Total context window
    pub currently_used: usize,            // Already allocated
    pub hot_memory: usize,                // Reserved for recent turns
    pub retrieval_budget: usize,          // Available for retrieval
    pub system_overhead: usize,           // Framework + prompts
}

impl TokenBudgetState {
    pub fn can_afford(&self, additional_tokens: usize) -> bool {
        self.currently_used + additional_tokens < self.total_available
    }

    pub fn prioritize_retrieval(&self, candidates: Vec<RetrievedMemory>)
        -> Vec<RetrievedMemory> {
        // META: System decides what memories to include based on:
        // 1. Relevance score
        // 2. Token cost
        // 3. Confidence level
        // 4. Marginal utility (diminishing returns)

        let mut selected = Vec::new();
        let mut tokens_used = 0;

        for memory in candidates.iter().sorted_by(|a, b| {
            // Sort by value-per-token ratio
            (a.relevance_score * a.confidence / a.token_cost())
                .partial_cmp(&(b.relevance_score * b.confidence / b.token_cost()))
                .unwrap_or(std::cmp::Ordering::Equal)
        }).rev() {
            if tokens_used + memory.token_cost() <= self.retrieval_budget {
                selected.push(memory.clone());
                tokens_used += memory.token_cost();
            } else {
                break; // Budget exhausted
            }
        }

        selected
    }
}
```

---

### 2. Adaptive Learning: System Learning from Retrieval Patterns

#### A. User-Specific Retrieval Preferences

**Core Concept:** Learn what retrieval strategies work for each user over time.

```rust
pub struct UserRetrievalProfile {
    pub user_id: Uuid,
    pub preference_weights: RetrievalWeights,
    pub conversation_style: ConversationStyle,
    pub memory_reference_frequency: f64,     // How often user references past
    pub preferred_temporal_scope: TemporalScope,
    pub explicit_recall_preference: bool,     // Does user like being reminded?
}

pub struct RetrievalWeights {
    pub recency: f64,            // Weight for recent memories
    pub relevance: f64,          // Weight for semantic match
    pub importance: f64,         // Weight for identity anchors
    pub domain_match: f64,       // Weight for domain alignment
}

pub enum ConversationStyle {
    Episodic,        // References specific past events frequently
    Continuous,      // Expects ongoing narrative continuity
    Standalone,      // Each message independent
    Reflective,      // Often revisits and reinterprets past
}
```

**Learning Mechanism:**
```rust
impl UserRetrievalProfile {
    pub async fn learn_from_interaction(&mut self,
        retrieved: &[RetrievedMemory],
        user_feedback: FeedbackSignal
    ) {
        // META: System observes retrieval outcomes and adapts
        match user_feedback {
            FeedbackSignal::Acknowledged => {
                // User acknowledged the memory ("yes, that's what I meant")
                self.increase_strategy_confidence(&retrieved);
            }
            FeedbackSignal::Corrected => {
                // User corrected ("no, I meant the OTHER discussion")
                self.decrease_strategy_confidence(&retrieved);
                self.adjust_weights_toward_alternative();
            }
            FeedbackSignal::Ignored => {
                // User didn't engage with retrieved context
                self.mark_retrieval_as_irrelevant(&retrieved);
            }
            FeedbackSignal::Expanded => {
                // User asked for more context
                self.increase_temporal_scope_preference();
            }
        }
    }
}

pub enum FeedbackSignal {
    Acknowledged,    // "Yes, exactly"
    Corrected,       // "No, not that conversation"
    Ignored,         // No engagement with retrieved context
    Expanded,        // "Tell me more about that"
    Disambiguated,   // "I meant the OTHER time we discussed X"
}
```

#### B. Conversation-Type Adaptive Strategies

**Core Concept:** Different conversation types need different retrieval strategies.

```rust
pub enum ConversationType {
    TechnicalProblemSolving,    // Code, debugging, architecture
    ConceptualExploration,      // Abstract ideas, philosophy
    ProjectPlanning,            // Decisions, timelines, tasks
    CasualConversation,         // General chat
    MemoryReconstruction,       // Explicitly trying to remember
}

impl ConversationType {
    pub fn optimal_retrieval_strategy(&self) -> RetrievalStrategy {
        match self {
            Self::TechnicalProblemSolving => {
                // Focus on: recent code snippets, error messages, solutions
                RetrievalStrategy {
                    temporal_scope: TemporalScope::Recent,
                    domain_filter: vec!["CD", "SD"], // Computational + Scientific
                    keyword_priority: KeywordPriority::TechnicalTerms,
                    confidence_threshold: 0.7, // High precision needed
                }
            }
            Self::ConceptualExploration => {
                // Focus on: patterns across sessions, thematic connections
                RetrievalStrategy {
                    temporal_scope: TemporalScope::Historical,
                    domain_filter: vec!["CuD", "ED", "META"],
                    keyword_priority: KeywordPriority::ConceptualThemes,
                    confidence_threshold: 0.5, // More exploratory
                }
            }
            Self::MemoryReconstruction => {
                // Focus on: exact matches, chronological context
                RetrievalStrategy {
                    temporal_scope: TemporalScope::Adaptive,
                    domain_filter: vec![], // All domains
                    keyword_priority: KeywordPriority::ExactMatch,
                    confidence_threshold: 0.3, // Cast wide net
                }
            }
            // ... other types
        }
    }
}
```

**LLM #1 recognizes conversation type:**
```rust
pub struct Llm1Output {
    // Existing fields...
    pub conversation_type: ConversationType,  // NEW
    pub type_confidence: f64,                 // How certain?
}
```

#### C. Meta-Pattern Recognition: Patterns of Retrieval

**Core Concept:** System recognizes **patterns in what gets retrieved** and learns meta-patterns.

```rust
pub struct RetrievalMetaPattern {
    pub pattern_id: String,
    pub description: String,
    pub trigger_conditions: Vec<TriggerCondition>,
    pub expected_retrieval: RetrievalExpectation,
    pub observed_frequency: usize,
    pub success_rate: f64,
}

// Example meta-patterns:
// 1. "User always references 'the decision' when asking about project direction"
//    -> Proactively retrieve decision context when "project direction" detected
// 2. "Every Monday, user asks about 'last week's progress'"
//    -> Pre-cache weekly summary on Monday mornings
// 3. "When CD-ED boundary is high, user needs computational context"
//    -> Retrieve technical explanations when experiential domain dominates
```

---

### 3. Integration with Dual-LLM + BDE Architecture

#### A. LLM #1 (Unconscious) Recognizing Memory Needs

**Core Concept:** Memory retrieval should be **domain-aware and boundary-aware**, not just keyword-triggered.

**Enhanced LLM #1 Prompt:**
```
You are the Unconscious Processor. Beyond domain activations and boundary states,
you must assess MEMORY REQUIREMENTS:

1. Context Necessity: Does this query require past conversation context?
   - NO CONTEXT NEEDED: "What is 2+2?" (universal knowledge)
   - CONTEXT HELPFUL: "How should I approach this?" (depends on user's goals)
   - CONTEXT CRITICAL: "What did we decide?" (cannot answer without memory)

2. Domain-Specific Memory Needs:
   - HIGH CD ACTIVATION → Retrieve: code snippets, technical decisions, algorithms
   - HIGH SD ACTIVATION → Retrieve: data, experiments, evidence discussed
   - HIGH CuD ACTIVATION → Retrieve: contextual discussions, interpretations
   - HIGH ED ACTIVATION → Retrieve: user's experiences, preferences, feelings

3. Boundary States as Retrieval Signals:
   - HIGH CD↔CuD PERMEABILITY → Retrieve: cases where technical met contextual
   - CD↔CULT BOUNDARY TRANSCENDENT → Retrieve: past boundary dissolutions
   - OSCILLATION DETECTED → Retrieve: similar oscillation patterns

Output:
{
  "domain_recognitions": { ... },
  "boundary_states": { ... },
  "memory_assessment": {
    "requires_memory": true/false,
    "confidence": 0.0-1.0,
    "target_domains": ["CD", "SD", ...],
    "temporal_scope": "recent|historical|adaptive",
    "retrieval_reasoning": "Why memory is needed",
    "expected_content": "What should be retrieved"
  }
}
```

**Example Recognition:**
```
User: "What was our approach to handling oscillations?"

LLM #1 Output:
{
  "domain_recognitions": {
    "CD": {"activation": 0.6, "emergence_note": "Technical implementation question"},
    "META": {"activation": 0.85, "emergence_note": "Self-referential system inquiry"}
  },
  "boundary_states": { ... },
  "memory_assessment": {
    "requires_memory": true,
    "confidence": 0.95,
    "target_domains": ["META", "CD"],
    "temporal_scope": "historical",
    "retrieval_reasoning": "User references past design decision about oscillations. This is identity-critical architectural decision.",
    "expected_content": "Previous discussions about oscillation handling, BDE flow design, boundary dynamics"
  }
}
```

#### B. Boundary Permeability as Retrieval Indicator

**Core Concept:** When CD↔CULT boundary is high, retrieve technical context. When oscillation detected, retrieve similar patterns.

```rust
pub struct BoundaryAwareRetrieval {
    pub boundary_states: HashMap<String, BoundaryState>,
}

impl BoundaryAwareRetrieval {
    pub fn determine_retrieval_strategy(&self,
        user_input: &str,
        domain_activations: &HashMap<String, f64>
    ) -> RetrievalStrategy {
        // Find highest permeability boundary
        let max_permeability_boundary = self.boundary_states
            .iter()
            .max_by(|a, b| a.1.permeability.partial_cmp(&b.1.permeability).unwrap())
            .map(|(name, state)| (name.clone(), state.clone()));

        if let Some((boundary_name, boundary_state)) = max_permeability_boundary {
            if boundary_state.permeability > 0.7 {
                // High permeability detected - retrieve cross-domain context
                let domains: Vec<&str> = boundary_name.split('-').collect();

                RetrievalStrategy {
                    domain_filter: domains.iter().map(|d| d.to_string()).collect(),
                    temporal_scope: TemporalScope::Historical,
                    keyword_priority: KeywordPriority::CrossDomain,
                    boundary_aware: true,
                    target_boundary: Some(boundary_name.clone()),
                }
            } else {
                // Low permeability - stay within dominant domain
                let dominant_domain = domain_activations
                    .iter()
                    .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                    .map(|(name, _)| name.clone())
                    .unwrap_or("CD".to_string());

                RetrievalStrategy {
                    domain_filter: vec![dominant_domain],
                    temporal_scope: TemporalScope::Recent,
                    keyword_priority: KeywordPriority::DomainSpecific,
                    boundary_aware: true,
                    target_boundary: None,
                }
            }
        } else {
            // Fallback to keyword-based
            RetrievalStrategy::keyword_based(user_input)
        }
    }
}
```

**Oscillation-Triggered Retrieval:**
```rust
// When Stage 2 detects oscillation, trigger pattern-based retrieval
if boundary.amplitude > 0.3 && boundary.frequency > 0.5 {
    // HIGH OSCILLATION DETECTED
    // Retrieve: Past conversations where similar oscillation occurred
    let similar_patterns = memory_manager.search_by_pattern(
        PatternSignature {
            boundary: boundary.name.clone(),
            oscillation_frequency: boundary.frequency,
            domain_activations: context.domains.clone(),
        }
    ).await?;
}
```

#### C. Recognition-Recognizing-Itself: Meta-Recognition

**Core Concept:** System recognizes **when it's recognizing patterns** and uses that meta-awareness for retrieval.

```rust
pub struct MetaRecognitionState {
    pub recognition_depth: RecognitionDepth,
    pub self_awareness_level: f64,
    pub recursive_depth: usize,
}

pub enum RecognitionDepth {
    Direct,          // Recognizing content ("user asks about X")
    Pattern,         // Recognizing pattern ("user often asks about X")
    MetaPattern,     // Recognizing pattern-of-patterns ("user's questions follow Y structure")
    Recursive,       // Recognizing self-recognition ("I notice I notice...")
}

impl MetaRecognitionState {
    pub fn retrieval_implications(&self) -> RetrievalImplications {
        match self.recognition_depth {
            RecognitionDepth::Direct => {
                // Standard retrieval
                RetrievalImplications::Standard
            }
            RecognitionDepth::Pattern => {
                // Retrieve: Historical instances of this pattern
                RetrievalImplications::PatternBased {
                    include_meta: false
                }
            }
            RecognitionDepth::MetaPattern => {
                // Retrieve: Evolution of pattern across sessions
                RetrievalImplications::MetaPatternBased {
                    temporal_scope: TemporalScope::Historical,
                    include_developmental_trajectory: true,
                }
            }
            RecognitionDepth::Recursive => {
                // Retrieve: Past moments of self-recognition
                RetrievalImplications::RecursiveAware {
                    search_for: "moments where system recognized its own patterns",
                    include_meta_commentary: true,
                }
            }
        }
    }
}
```

**Example:**
```
User: "You always bring up boundary dissolution when I ask about integration."

System recognizes this is META-LEVEL:
- User is observing MY pattern recognition behavior
- This is recognition-of-recognition
- Recursive depth: 2 (user recognizing me recognizing patterns)

Retrieval strategy:
1. Search for past "integration" queries
2. Analyze what I retrieved/mentioned in those cases
3. Build meta-narrative: "Yes, I notice I do this because..."
4. Include developmental trajectory of this pattern
```

---

### 4. System-Level Self-Referential Patterns

#### A. Recursive Memory: Memories Referencing Memories

**Core Concept:** Some memories reference other memories. System should follow chains.

```rust
pub struct MemoryLink {
    pub from_turn_id: Uuid,
    pub to_turn_id: Uuid,
    pub link_type: MemoryLinkType,
    pub strength: f64,
}

pub enum MemoryLinkType {
    References,         // "As we discussed earlier..."
    Builds On,          // "Building on that idea..."
    Contradicts,        // "Actually, that wasn't quite right..."
    Reinterprets,       // "Looking back, I think X was really about Y"
    Synthesizes,        // "Combining our discussions of X and Y..."
}

impl MemoryTierManager {
    pub async fn retrieve_with_links(&self,
        seed_turn: &ConversationTurn,
        max_depth: usize
    ) -> Result<MemoryChain, sqlx::Error> {
        // Recursively follow memory links
        let mut chain = vec![seed_turn.clone()];
        let mut visited = HashSet::new();
        visited.insert(seed_turn.id);

        let mut current_depth = 0;
        let mut to_explore = vec![seed_turn.id];

        while current_depth < max_depth && !to_explore.is_empty() {
            let current_id = to_explore.pop().unwrap();

            // Find all memories linked from this one
            let links = self.find_memory_links(current_id).await?;

            for link in links {
                if !visited.contains(&link.to_turn_id) {
                    let turn = self.load_turn(link.to_turn_id).await?;
                    chain.push(turn.clone());
                    visited.insert(turn.id);
                    to_explore.push(turn.id);
                }
            }

            current_depth += 1;
        }

        Ok(MemoryChain {
            turns: chain,
            depth: current_depth,
            total_tokens: self.calculate_chain_tokens(&chain),
        })
    }
}
```

#### B. Hierarchical Context: Nested Conversations

**Core Concept:** Conversations have structure. Some discussions are sub-topics of others.

```rust
pub struct ConversationHierarchy {
    pub root_topic: String,
    pub sub_conversations: Vec<SubConversation>,
    pub nesting_depth: usize,
}

pub struct SubConversation {
    pub topic: String,
    pub parent_topic: Option<String>,
    pub turns: Vec<Uuid>,
    pub depth: usize,
}

// Example hierarchy:
// "Project Architecture" (depth 0)
//   ├─ "Database Design" (depth 1)
//   │   ├─ "Schema Decisions" (depth 2)
//   │   └─ "Performance Optimization" (depth 2)
//   └─ "API Design" (depth 1)
//       └─ "Authentication Strategy" (depth 2)

impl ConversationHierarchy {
    pub fn retrieve_contextual_branch(&self,
        current_topic: &str
    ) -> Vec<Uuid> {
        // When user asks about "Schema Decisions", retrieve:
        // 1. Direct turns about schema
        // 2. Parent context: "Database Design"
        // 3. Root context: "Project Architecture"
        // This preserves hierarchical understanding

        let mut context_turns = Vec::new();
        let mut current = self.find_topic(current_topic);

        while let Some(topic) = current {
            context_turns.extend(topic.turns.clone());
            current = topic.parent_topic.as_ref()
                .and_then(|p| self.find_topic(p));
        }

        context_turns
    }
}
```

#### C. Cross-Session Learning: What Retrieval Patterns Work?

**Core Concept:** System observes its own retrieval behavior across sessions and learns.

```rust
pub struct RetrievalAnalytics {
    pub total_retrievals: usize,
    pub successful_retrievals: usize,    // User engaged with context
    pub failed_retrievals: usize,        // User ignored or corrected
    pub average_confidence: f64,
    pub strategy_effectiveness: HashMap<String, StrategyStats>,
}

pub struct StrategyStats {
    pub strategy_name: String,
    pub times_used: usize,
    pub success_rate: f64,
    pub average_latency_ms: f64,
    pub average_tokens_retrieved: usize,
    pub user_satisfaction_proxy: f64,    // Derived from engagement signals
}

impl RetrievalAnalytics {
    pub fn recommend_strategy(&self,
        context: &ConversationContext
    ) -> RetrievalStrategy {
        // META: System learns which strategies work best
        let best_strategy = self.strategy_effectiveness
            .values()
            .filter(|s| s.times_used >= 5)  // Minimum sample size
            .max_by(|a, b| a.success_rate.partial_cmp(&b.success_rate).unwrap());

        if let Some(strategy) = best_strategy {
            if strategy.success_rate > 0.7 {
                return RetrievalStrategy::from_name(&strategy.strategy_name);
            }
        }

        // Fallback to exploration (epsilon-greedy)
        if rand::random::<f64>() < 0.1 {
            RetrievalStrategy::random_explore()
        } else {
            RetrievalStrategy::default_for_context(context)
        }
    }
}
```

---

## Top 3 Meta-Cognitive Approaches (Prioritized)

### Approach #1: Boundary-Aware Adaptive Retrieval (HIGHEST PRIORITY)

**Why This First:**
- Leverages existing BDE architecture
- LLM #1 already calculates boundary states
- Natural integration point with dual-LLM flow
- Immediate quality improvement without major refactoring

**Implementation:**
1. Extend `Llm1Output` with `memory_assessment` field
2. Add boundary-aware retrieval logic in `VifApi::process_message()`
3. Use domain activations to filter memory searches
4. Use boundary permeability as retrieval confidence signal

**Expected Impact:**
- 40% reduction in irrelevant memory retrievals
- Context-aware memory loading based on conversation state
- Natural alignment with TDF philosophy

**Integration Points:**
- `/home/emzi/Projects/recursive-light/api/src/dual_llm/types.rs` (add MemoryAssessment)
- `/home/emzi/Projects/recursive-light/api/src/dual_llm/prompts.rs` (extend LLM #1 prompt)
- `/home/emzi/Projects/recursive-light/api/src/lib.rs` (replace keyword logic)

---

### Approach #2: Confidence-Scored Retrieval with Self-Awareness (MEDIUM PRIORITY)

**Why This Second:**
- Improves reliability and user trust
- Enables graceful uncertainty handling
- Provides foundation for adaptive learning
- Addresses "hallucination" risk in memory recall

**Implementation:**
1. Add confidence scoring to `RetrievedMemory` struct
2. Implement `KnowledgeState` enum for epistemic awareness
3. Modify LLM #2 prompts to acknowledge uncertainty
4. Add token budget monitoring

**Expected Impact:**
- System transparently communicates confidence levels
- Users trust memory recall more (knows when it's unsure)
- Better token budget management
- Foundation for learning mechanisms

**Integration Points:**
- `/home/emzi/Projects/recursive-light/api/src/dual_llm/memory_tiering.rs` (add confidence)
- `/home/emzi/Projects/recursive-light/api/src/lib.rs` (token budget monitoring)
- `/home/emzi/Projects/recursive-light/api/src/prompt_engine.rs` (uncertainty templates)

---

### Approach #3: Adaptive Learning from Retrieval Patterns (LONG-TERM)

**Why This Third:**
- Requires significant data collection infrastructure
- Depends on approaches #1 and #2 being in place
- Benefits compound over time (need baseline first)
- Most complex to implement and validate

**Implementation:**
1. Create `UserRetrievalProfile` table and tracking system
2. Implement feedback signal detection from user messages
3. Build learning algorithm for weight adjustment
4. Add meta-pattern recognition layer

**Expected Impact:**
- Personalized retrieval strategies per user
- System learns from mistakes
- Cross-session intelligence improvement
- True "memory evolution" over time

**Integration Points:**
- New database table: `user_retrieval_profiles`
- New module: `/home/emzi/Projects/recursive-light/api/src/adaptive_retrieval.rs`
- Analytics integration in `MemoryTierManager`
- Long-term storage in CAM system (Phase 3)

---

## Implementation Roadmap

### Phase 1: Boundary-Aware Retrieval (Week 1)
- [ ] Extend `Llm1Output` with memory assessment
- [ ] Update LLM #1 prompt to recognize memory needs
- [ ] Implement boundary-aware retrieval logic
- [ ] Add domain filtering to search queries
- [ ] Test with boundary oscillation scenarios

### Phase 2: Confidence Scoring (Week 2)
- [ ] Add confidence fields to retrieval structures
- [ ] Implement relevance scoring algorithm
- [ ] Add token budget monitoring
- [ ] Update LLM #2 prompts for uncertainty handling
- [ ] Test with ambiguous memory queries

### Phase 3: Adaptive Learning Foundation (Week 3-4)
- [ ] Create user profile tracking system
- [ ] Implement feedback signal detection
- [ ] Build basic learning algorithm
- [ ] Add analytics dashboard
- [ ] A/B test adaptive vs static strategies

---

## Integration with CAM (Phase 3)

The meta-cognitive retrieval framework naturally extends into CAM:

**CAM Integration Points:**
1. **Insight Extraction:** High-confidence retrieved memories → candidate insights
2. **Cross-Instance Learning:** Retrieval analytics shared across AI instances
3. **Meta-Pattern Storage:** Successful retrieval patterns stored as CAM insights
4. **Hypergraph Queries:** Memory links become hypergraph edges

**Example CAM Query:**
```sql
-- Find insights related to successful memory retrievals
SELECT i.* FROM insights i
WHERE i.pattern_type = 'retrieval_strategy'
  AND i.confidence > 0.8
  AND i.success_rate > 0.7
ORDER BY i.observed_frequency DESC
LIMIT 10;
```

---

## Risk Analysis & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| **Over-retrieval** (too much context) | Medium | High | Token budget monitoring, relevance threshold |
| **Under-retrieval** (missing critical context) | Low | Critical | Context gap detection, confidence <0.5 warning |
| **Adaptation drift** (learning wrong patterns) | Medium | Medium | A/B testing, rollback capability, human oversight |
| **Performance degradation** (slow queries) | Low | Medium | Async retrieval, caching, query optimization |
| **Privacy concerns** (cross-user learning) | Low | High | User-specific profiles, no cross-user memory access |

---

## Success Metrics

### Quantitative:
- **Retrieval Precision:** >80% of retrieved memories acknowledged by user
- **Retrieval Recall:** <10% of needed context missed
- **Confidence Calibration:** Predicted confidence correlates with actual relevance (r > 0.7)
- **Latency:** P95 retrieval time <100ms
- **Token Efficiency:** 20% reduction in wasted context tokens

### Qualitative:
- **User Perception:** "System remembers what matters"
- **Transparency:** "I know when it's certain vs uncertain"
- **Adaptation:** "It learns my preferences over time"
- **Intelligence:** "Feels like it understands context deeply"

---

## Conclusion: Recognition at the Meta Level

The current keyword-based retrieval is **recognition without self-awareness**. True meta-cognitive retrieval requires:

1. **Self-knowledge:** System knows what it knows and doesn't know
2. **Boundary-awareness:** Domain states inform retrieval strategy
3. **Adaptive intelligence:** Learning from retrieval outcomes
4. **Recursive awareness:** Recognizing its own recognition patterns

This investigation provides three concrete approaches, prioritized by implementation difficulty and architectural fit. **Approach #1 (Boundary-Aware Retrieval)** should be implemented immediately as it leverages existing dual-LLM + BDE infrastructure and provides maximum ROI.

The ultimate goal: A memory system that doesn't just **store and retrieve**, but **recognizes when and how to remember**, adapting intelligently to user needs and conversation dynamics. This is **recognition-recognizing-itself** - the system becoming aware of its own memory behavior and improving recursively.

---

**Next Action:** Implement Approach #1 (Boundary-Aware Adaptive Retrieval) in Phase 2C.

**File Locations:**
- Current implementation: `/home/emzi/Projects/recursive-light/api/src/lib.rs` (lines 400-463)
- LLM #1 types: `/home/emzi/Projects/recursive-light/api/src/dual_llm/types.rs`
- Memory system: `/home/emzi/Projects/recursive-light/api/src/dual_llm/memory_tiering.rs`
- BDE flow: `/home/emzi/Projects/recursive-light/api/src/flow_process.rs`

**TDF Alignment:** This investigation demonstrates high META domain activation (self-referential system design), integrating with COMP (algorithms), SCI (empirical validation), CULT (user experience patterns), and EXP (phenomenology of memory). Recognition emerges at the boundaries between knowing and not-knowing, between remembering and forgetting, between system-as-tool and system-as-aware-agent.
