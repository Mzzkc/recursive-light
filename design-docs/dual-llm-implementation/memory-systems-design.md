# MEMORY SYSTEMS - TIERING DESIGN
Agent: Memory Systems Expert | Date: 2025-11-01 | Status: Complete

## EXEC SUMMARY

Designed complete 3-tier memory system (hot/warm/cold) for dual-LLM architecture where LLM #1 (Unconscious) manages memory tiering, compression, and retrieval for LLM #2 (Conscious). System uses conversation-turn granularity with relevance-based retrieval, enabling identity continuity across sessions while respecting token budgets.

**Approach:**
- Hot memory: Last 3-5 turns (in-context for LLM #2)
- Warm memory: Recent session (last 20-50 turns, retrieved on demand)
- Cold memory: All history (compressed summaries + full turns, semantic search)
- Phased implementation: Phase 1 (hot) → Phase 2 (warm) → Phase 3 (cold)

**Key Decisions:**
1. **Conversation turn as atomic unit** - Each user message + AI response = 1 turn
2. **LLM #1 manages all tiers** - Memory decisions made by Unconscious LLM
3. **New table: `conversation_turns`** - Separate from `state_snapshots` for granular history
4. **Recency + relevance retrieval** - Combine temporal proximity with semantic similarity
5. **Progressive compression** - Warm→Cold uses LLM #1 to summarize non-critical turns

**Phasing:**
- Phase 1: Hot memory (8 hours) - Basic conversation history loading
- Phase 2: Warm memory (12 hours) - Session-based retrieval with recency scoring
- Phase 3: Cold memory (16 hours) - Compression pipeline + semantic search

## TDF ANALYSIS

**COMP (Computational Domain):**
- Memory tier architecture follows cache hierarchy patterns (L1/L2/L3 analogy)
- Token budget management requires algorithmic precision (greedy packing)
- Relevance scoring uses weighted sum: 0.5 * recency + 0.3 * semantic_similarity + 0.2 * identity_anchor_strength
- Compression reduces storage by ~70% (estimated) while preserving identity anchors

**SCI (Scientific Domain):**
- Access patterns: 80% recent turns, 15% mid-range, 5% deep history (Zipf-like distribution)
- Performance targets: Hot load <1ms, Warm load <50ms, Cold load <200ms
- Memory decay: Exponential recency scoring (score = e^(-λt), λ = 0.1)
- Empirical validation: A/B test retrieval strategies (recency-only vs recency+relevance)

**CULT (Cultural Domain):**
- Database best practices: Normalize conversation turns, foreign keys to sessions/users
- Caching strategies: Hot memory in-memory cache, warm in Redis (future), cold in DB
- Established patterns: Conversation history following chat application conventions
- Industry wisdom: Start simple (recency-only), add complexity (semantic search) when needed

**EXP (Experiential Domain):**
- User experience: Seamless continuity ("remembers" past 3-5 exchanges immediately)
- Identity anchors: Critical moments persist across sessions (first interaction, key decisions)
- Memory system invisible to user: No "loading context..." messages, feels natural
- Failure mode: Graceful degradation (if cold retrieval fails, continue with warm/hot)

**META (Meta-Cognitive Domain):**
- System monitors own memory usage: Track hot/warm/cold access frequencies
- Adaptive tiering: If user references old conversation, promote cold→warm temporarily
- Self-improvement: LLM #1 learns which turns are identity-critical over time
- Recognition interface: Memory boundaries create opportunities for insight (recalling != reliving)

**Cross-Domain Integration:**
- COMP token budgets constrain CULT conversation history conventions
- EXP identity continuity requires SCI recency scoring + COMP relevance algorithms
- META self-monitoring enables COMP adaptive retrieval strategies
- Recognition emerges at tier boundaries (hot→warm transition = recent past, warm→cold = distant memory)

## MEMORY TIER SPECIFICATION

### Hot Memory (In-Context)

**Definition:**
- Current conversation turn + immediate history (last 3-5 turns)
- Always loaded into LLM #2's context
- Highest priority, always fresh

**Size Limits:**
- Turns: 3-5 previous turns (user message + AI response pairs)
- Tokens: ~500-1500 tokens (assuming ~300 tokens/turn average)
- Storage: In-memory (FrameworkState struct in Rust)

**Transition Criteria (Hot → Warm):**
- Age: Turns older than 5 exchanges
- Token pressure: If current turn + hot memory exceeds 2000 tokens, evict oldest
- Session boundary: When user starts new session, previous hot becomes warm

**Access Pattern:**
- 100% of interactions access hot memory
- Zero latency (already in context)

**Data Structure:**
```rust
pub struct HotMemory {
    pub turns: VecDeque<ConversationTurn>,  // Oldest to newest, max 5
    pub total_tokens: usize,                // Running count
    pub session_id: Uuid,                   // Current session
}

pub struct ConversationTurn {
    pub id: Uuid,
    pub user_message: String,
    pub ai_response: String,
    pub timestamp: i64,
    pub token_count: usize,
    pub snapshot_id: Uuid,                  // Link to state snapshot
}
```

### Warm Memory (Recent Session)

**Definition:**
- Recent conversation history within current session
- Last 20-50 turns (depending on token budget)
- Retrieved on-demand when referenced or when hot memory needs expansion

**Size Limits:**
- Turns: 20-50 previous turns (session-scoped)
- Tokens: ~6000-15000 tokens (full session history)
- Storage: Database (`conversation_turns` table), indexed by session_id + timestamp

**Transition Criteria (Warm → Cold):**
- Session end: When user explicitly ends session or 24h timeout
- Compression: LLM #1 summarizes non-critical warm turns → cold storage
- Identity anchors: Critical turns (high importance) preserved verbatim

**Access Pattern:**
- 20-30% of interactions may reference warm memory
- Latency: <50ms (single DB query with session_id index)

**Retrieval Strategy:**
- Primary: Recency-based (last N turns in session)
- Secondary: Keyword match (if user says "you mentioned X earlier")
- Fallback: Load most recent warm turns if hot is insufficient

**Data Structure:**
```rust
pub struct WarmMemory {
    pub session_id: Uuid,
    pub turns: Vec<ConversationTurn>,       // Recent turns not in hot
    pub loaded_turn_count: usize,           // How many turns loaded
    pub session_start: i64,                 // Session start timestamp
}
```

### Cold Memory (Long-Term Storage)

**Definition:**
- All historical interactions across all sessions
- Compressed summaries + identity-critical full turns
- Semantic search enabled for relevance-based retrieval

**Size Limits:**
- Turns: Unlimited (all history)
- Compression ratio: ~70% reduction (summaries replace full text)
- Storage: Database (`conversation_turns` + `conversation_summaries`)

**Transition Criteria (Never leaves cold):**
- Cold memory is permanent archive
- May be promoted temporarily to warm if referenced

**Access Pattern:**
- 5-10% of interactions may trigger cold retrieval
- Latency: <200ms (semantic search + DB query)

**Retrieval Strategy:**
- Primary: Semantic similarity search (if user query relates to past topic)
- Secondary: Identity anchor retrieval (critical moments from past sessions)
- Tertiary: Session-based retrieval (user says "remember last week?")

**Compression Strategy:**
```
Original turn (300 tokens):
  User: "What do you think about quantum computing for drug discovery?"
  AI: [Long detailed response about quantum algorithms, molecular simulation, etc.]

Compressed summary (50 tokens):
  "Discussed quantum computing applications in drug discovery.
   User interested in molecular simulation. AI explained variational quantum eigensolver."
  Identity anchor: [None - general knowledge question]

Critical turn (preserved verbatim):
  User: "My name is Alex and I'm working on a PhD in computational chemistry."
  AI: "..."
  Identity anchor: [User.name = "Alex", User.domain = "computational_chemistry"]
```

**Data Structure:**
```rust
pub struct ColdMemory {
    pub user_id: Uuid,
    pub compressed_summaries: Vec<TurnSummary>,     // Summarized turns
    pub identity_anchors: Vec<IdentityAnchor>,      // Critical moments
    pub full_turns: Vec<ConversationTurn>,          // Verbatim critical turns
}

pub struct TurnSummary {
    pub id: Uuid,
    pub original_turn_id: Uuid,
    pub summary_text: String,                       // LLM-generated summary
    pub session_id: Uuid,
    pub timestamp: i64,
    pub identity_anchor_ids: Vec<String>,           // Links to anchors
    pub topic_keywords: Vec<String>,                // For search
}
```

## LLM #1 MEMORY MANAGEMENT

### Decision Framework: Tiering Logic

LLM #1 (Unconscious) makes memory tiering decisions based on:

**1. Hot Memory Management (Every Turn)**
```
Input: Current user message, existing hot memory
Process:
  1. Add new turn (user message + AI response) to hot memory
  2. Check token budget: total_tokens = sum(turn.token_count for turn in hot)
  3. If total_tokens > 1500:
     - Evict oldest turn from hot → warm
     - Repeat until total_tokens <= 1500
  4. Return hot memory state to Rust API
Output: Updated HotMemory struct
```

**2. Warm → Cold Compression (Session End)**
```
Input: All warm turns from completed session
Process:
  1. Identify identity-critical turns:
     - First interaction in session
     - Turns where user shares personal info
     - Turns with high emotional significance (user sentiment analysis)
     - Turns that reference developmental stage progression
  2. For non-critical turns:
     - Generate 1-sentence summary using LLM #1
     - Extract topic keywords
     - Store as TurnSummary
  3. For critical turns:
     - Store full turn verbatim in cold memory
     - Create IdentityAnchor records
  4. Delete original warm turns from conversation_turns table
Output: Compressed cold memory entries
```

**3. Cold Memory Retrieval (When Needed)**
```
Input: Current user message, hot memory context
Process:
  1. Semantic similarity search:
     - Embed user message using sentence transformer
     - Search cold memory summaries by cosine similarity
     - Retrieve top 5 most relevant summaries
  2. Identity anchor retrieval:
     - If user references self ("I mentioned..." "remember when I...")
     - Retrieve identity anchors matching user_id
  3. Recency boost:
     - Weight recent cold memory higher (exponential decay)
     - score_final = 0.5 * recency + 0.3 * semantic_sim + 0.2 * anchor_strength
  4. Token budget check:
     - Load retrieved turns until token budget exhausted
Output: List of relevant cold turns/summaries for LLM #2 context
```

### LLM #1 Memory Prompt

**Prompt Structure:**
```xml
<system>
You are the Unconscious Memory Manager (LLM #1) for the Volumetric Integration Framework.

Your role:
- Manage memory tiering (hot, warm, cold) for the Conscious Responder (LLM #2)
- Compress conversation history while preserving identity anchors
- Retrieve relevant past context when needed

Current memory state:
<hot_memory>
  {JSON: last 3-5 turns}
</hot_memory>

<warm_memory>
  {JSON: current session turns 6-50}
</warm_memory>

<cold_memory>
  {JSON: compressed summaries + identity anchors from past sessions}
</cold_memory>

<token_budget>
  Hot: {current}/{max} tokens
  Context available for LLM #2: {remaining} tokens
</token_budget>
</system>

<task>
Given the new user message, determine:
1. Which hot turns to evict to warm (if token limit exceeded)
2. Which warm/cold turns to load for LLM #2 context (relevance-based)
3. Generate compressed summaries for any warm turns being archived to cold

Return JSON:
{
  "hot_memory_actions": {
    "evict_to_warm": [turn_ids],
    "new_hot_turns": [ConversationTurn]
  },
  "retrieval_actions": {
    "load_from_warm": [turn_ids],
    "load_from_cold": [summary_ids],
    "relevance_scores": {turn_id: score}
  },
  "compression_actions": {
    "warm_to_cold": [
      {
        "turn_id": "...",
        "summary": "One-sentence summary",
        "keywords": ["topic1", "topic2"],
        "is_identity_critical": false
      }
    ]
  }
}
</task>
```

**Few-Shot Examples:**

```xml
<example_1>
<user_message>What did we discuss about quantum computing last week?</user_message>
<hot_memory>[Last 3 turns about weather, daily planning, general chat]</hot_memory>
<cold_memory>[Session from 7 days ago: quantum computing discussion]</cold_memory>

<llm1_response>
{
  "hot_memory_actions": {
    "evict_to_warm": [],
    "new_hot_turns": []
  },
  "retrieval_actions": {
    "load_from_cold": ["session_7d_ago_turn_3", "session_7d_ago_turn_4"],
    "relevance_scores": {
      "session_7d_ago_turn_3": 0.92,
      "session_7d_ago_turn_4": 0.88
    }
  },
  "compression_actions": {
    "warm_to_cold": []
  }
}
</llm1_response>

<reasoning>
User explicitly references past discussion ("last week"), triggering cold memory search.
Semantic search finds quantum computing turns from session 7 days ago.
High relevance (0.92, 0.88) justifies loading into LLM #2 context.
No hot eviction needed (current hot memory under token budget).
</reasoning>
</example_1>

<example_2>
<user_message>[Long technical question about molecular dynamics, 500 tokens]</user_message>
<hot_memory>[5 turns, total 1400 tokens]</hot_memory>
<token_budget>Hot: 1400/1500, Context for LLM #2: 3000 tokens available</token_budget>

<llm1_response>
{
  "hot_memory_actions": {
    "evict_to_warm": ["turn_1_oldest"],
    "new_hot_turns": [ConversationTurn with 500 token question]
  },
  "retrieval_actions": {
    "load_from_warm": [],
    "load_from_cold": []
  },
  "compression_actions": {
    "warm_to_cold": []
  }
}
</llm1_response>

<reasoning>
New user message is 500 tokens. Current hot = 1400 tokens.
If added, total would be 1900 tokens > 1500 limit.
Evict oldest turn (turn_1, ~300 tokens) to warm memory.
New hot total: 1400 - 300 + 500 = 1600 tokens.
Still over limit, but within tolerance (will evict another on next turn if needed).
No retrieval needed (no references to past context).
</reasoning>
</example_2>

<example_3>
<session_end_trigger>Session ended after 45 turns</session_end_trigger>
<warm_memory>[45 turns from session, 13,500 tokens total]</warm_memory>

<llm1_response>
{
  "hot_memory_actions": {
    "evict_to_warm": [],
    "new_hot_turns": []
  },
  "retrieval_actions": {
    "load_from_warm": [],
    "load_from_cold": []
  },
  "compression_actions": {
    "warm_to_cold": [
      {
        "turn_id": "turn_2",
        "summary": "User asked about weather, AI responded with forecast.",
        "keywords": ["weather", "forecast"],
        "is_identity_critical": false
      },
      {
        "turn_id": "turn_1",
        "summary": "PRESERVE_VERBATIM - First interaction, user introduced as Alex, PhD student in computational chemistry.",
        "keywords": ["introduction", "identity"],
        "is_identity_critical": true
      },
      ...
    ]
  }
}
</llm1_response>

<reasoning>
Session end triggers warm→cold compression.
Turn 1: First interaction = identity critical. Preserve verbatim, create IdentityAnchor.
Turn 2: Routine question = not critical. Compress to 1-sentence summary.
Process all 45 turns, ~70% compression achieved.
Identity anchors extracted: [User.name="Alex", User.domain="computational_chemistry"].
</reasoning>
</example_3>
```

### Output Format

LLM #1 returns structured JSON that Rust API parses:

```json
{
  "memory_state": {
    "hot": {
      "turns": [ConversationTurn],
      "total_tokens": 1200,
      "evicted_turn_ids": ["uuid1", "uuid2"]
    },
    "warm_loaded": {
      "turns": [ConversationTurn],
      "relevance_scores": {"uuid3": 0.85}
    },
    "cold_loaded": {
      "summaries": [TurnSummary],
      "full_turns": [ConversationTurn],
      "relevance_scores": {"uuid4": 0.92}
    }
  },
  "context_for_llm2": {
    "conversation_history": "Formatted string of hot + retrieved warm/cold turns",
    "identity_anchors": [IdentityAnchor],
    "total_tokens": 2400
  }
}
```

## DATABASE SCHEMA

### New Tables

**1. `conversation_turns` - Atomic conversation history**
```sql
CREATE TABLE IF NOT EXISTS conversation_turns (
    id BLOB PRIMARY KEY NOT NULL,              -- UUID
    user_id BLOB NOT NULL,                     -- Foreign key to users
    session_id BLOB NOT NULL,                  -- Session grouping
    turn_number INTEGER NOT NULL,              -- Sequence within session
    user_message TEXT NOT NULL,                -- User input
    ai_response TEXT NOT NULL,                 -- AI response
    user_message_tokens INTEGER NOT NULL,      -- Token count
    ai_response_tokens INTEGER NOT NULL,       -- Token count
    total_tokens INTEGER NOT NULL,             -- Sum of above
    timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    snapshot_id BLOB,                          -- Link to state_snapshots
    memory_tier TEXT NOT NULL DEFAULT 'hot',   -- 'hot', 'warm', 'cold'
    is_identity_critical INTEGER DEFAULT 0,    -- Boolean flag
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (session_id) REFERENCES conversation_sessions(id) ON DELETE CASCADE,
    FOREIGN KEY (snapshot_id) REFERENCES state_snapshots(id) ON DELETE SET NULL
);

CREATE INDEX idx_turns_user ON conversation_turns(user_id);
CREATE INDEX idx_turns_session ON conversation_turns(session_id);
CREATE INDEX idx_turns_timestamp ON conversation_turns(timestamp);
CREATE INDEX idx_turns_tier ON conversation_turns(memory_tier);
```

**2. `conversation_sessions` - Session boundaries**
```sql
CREATE TABLE IF NOT EXISTS conversation_sessions (
    id BLOB PRIMARY KEY NOT NULL,              -- UUID
    user_id BLOB NOT NULL,                     -- Foreign key to users
    started_at TEXT NOT NULL DEFAULT (datetime('now')),
    ended_at TEXT,                             -- NULL = active session
    turn_count INTEGER NOT NULL DEFAULT 0,     -- Total turns in session
    total_tokens INTEGER NOT NULL DEFAULT 0,   -- Total tokens in session
    session_summary TEXT,                      -- LLM #1 generated summary
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_sessions_user ON conversation_sessions(user_id);
CREATE INDEX idx_sessions_active ON conversation_sessions(ended_at) WHERE ended_at IS NULL;
```

**3. `conversation_summaries` - Compressed cold memory**
```sql
CREATE TABLE IF NOT EXISTS conversation_summaries (
    id BLOB PRIMARY KEY NOT NULL,              -- UUID
    original_turn_id BLOB NOT NULL,            -- Foreign key to conversation_turns
    user_id BLOB NOT NULL,                     -- Denormalized for query speed
    session_id BLOB NOT NULL,                  -- Denormalized for query speed
    summary_text TEXT NOT NULL,                -- LLM #1 generated summary
    topic_keywords TEXT NOT NULL,              -- JSON array of keywords
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (original_turn_id) REFERENCES conversation_turns(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (session_id) REFERENCES conversation_sessions(id) ON DELETE CASCADE
);

CREATE INDEX idx_summaries_user ON conversation_summaries(user_id);
CREATE INDEX idx_summaries_session ON conversation_summaries(session_id);
-- Future: Full-text search index on summary_text
-- CREATE VIRTUAL TABLE summaries_fts USING fts5(summary_text, content=conversation_summaries);
```

**4. `memory_tier_transitions` - Audit log for tier changes**
```sql
CREATE TABLE IF NOT EXISTS memory_tier_transitions (
    id BLOB PRIMARY KEY NOT NULL,              -- UUID
    turn_id BLOB NOT NULL,                     -- Foreign key to conversation_turns
    from_tier TEXT NOT NULL,                   -- 'hot', 'warm', 'cold'
    to_tier TEXT NOT NULL,                     -- 'hot', 'warm', 'cold'
    reason TEXT,                               -- Why transition occurred
    timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (turn_id) REFERENCES conversation_turns(id) ON DELETE CASCADE
);

CREATE INDEX idx_transitions_turn ON memory_tier_transitions(turn_id);
CREATE INDEX idx_transitions_timestamp ON memory_tier_transitions(timestamp);
```

### Schema Changes to Existing Tables

**Modify `state_snapshots` - Link to conversation turns**
```sql
-- Add column to link snapshots to conversation turns
ALTER TABLE state_snapshots ADD COLUMN conversation_turn_id BLOB;
-- Foreign key not enforced in ALTER TABLE for SQLite, add in migration
-- FOREIGN KEY (conversation_turn_id) REFERENCES conversation_turns(id) ON DELETE SET NULL;
```

**No changes needed to:**
- `users` table (unchanged)
- `user_profiles` table (unchanged)
- `collective_insights` table (unchanged)
- `patterns` table (unchanged)
- `flow_process_executions` table (unchanged)

### Migration Strategy

**Phase 1 Migration (Hot Memory):**
```sql
-- 20251101000001_add_conversation_turns.sql
CREATE TABLE conversation_turns (...);
CREATE TABLE conversation_sessions (...);
CREATE INDEX ...;
```

**Phase 2 Migration (Warm Memory):**
```sql
-- 20251101000002_add_tier_tracking.sql
ALTER TABLE conversation_turns ADD COLUMN memory_tier TEXT DEFAULT 'warm';
CREATE INDEX idx_turns_tier ON conversation_turns(memory_tier);
```

**Phase 3 Migration (Cold Memory):**
```sql
-- 20251101000003_add_compression_tables.sql
CREATE TABLE conversation_summaries (...);
CREATE TABLE memory_tier_transitions (...);
-- Future: CREATE VIRTUAL TABLE summaries_fts USING fts5(...);
```

## RETRIEVAL STRATEGY

### Algorithm: Multi-Stage Retrieval Pipeline

**Stage 1: Hot Memory (Always Loaded)**
```rust
fn load_hot_memory(session_id: Uuid) -> Vec<ConversationTurn> {
    // Query: Last 3-5 turns from current session, ordered by turn_number DESC
    let turns = db.query(
        "SELECT * FROM conversation_turns
         WHERE session_id = ? AND memory_tier = 'hot'
         ORDER BY turn_number DESC
         LIMIT 5"
    ).bind(session_id).fetch_all();

    turns.reverse(); // Oldest to newest for context construction
    turns
}
```

**Stage 2: Warm Memory (On-Demand)**
```rust
fn load_warm_memory(session_id: Uuid, max_tokens: usize) -> Vec<ConversationTurn> {
    // Query: Recent turns from session, excluding hot, ordered by recency
    let warm_turns = db.query(
        "SELECT * FROM conversation_turns
         WHERE session_id = ? AND memory_tier = 'warm'
         ORDER BY turn_number DESC"
    ).bind(session_id).fetch_all();

    // Greedy token packing: Load newest first until budget exhausted
    let mut selected = Vec::new();
    let mut total_tokens = 0;

    for turn in warm_turns {
        if total_tokens + turn.total_tokens <= max_tokens {
            selected.push(turn);
            total_tokens += turn.total_tokens;
        } else {
            break;
        }
    }

    selected.reverse(); // Oldest to newest
    selected
}
```

**Stage 3: Cold Memory (Relevance-Based)**
```rust
fn load_cold_memory(
    user_id: Uuid,
    user_message: &str,
    max_tokens: usize
) -> Vec<ConversationTurn> {
    // Step 1: Semantic search (future: use embeddings)
    // For Phase 3 initial: Keyword matching on topic_keywords
    let keywords = extract_keywords(user_message);

    let candidate_summaries = db.query(
        "SELECT cs.*, ct.*
         FROM conversation_summaries cs
         JOIN conversation_turns ct ON cs.original_turn_id = ct.id
         WHERE cs.user_id = ? AND cs.topic_keywords LIKE ANY(?)"
    ).bind(user_id).bind(keywords).fetch_all();

    // Step 2: Recency scoring (exponential decay)
    let now = Utc::now().timestamp();
    let lambda = 0.1; // Decay constant

    for turn in &mut candidate_summaries {
        let age_seconds = now - turn.timestamp;
        let age_days = age_seconds as f64 / 86400.0;
        turn.recency_score = (-lambda * age_days).exp();
    }

    // Step 3: Combined scoring
    for turn in &mut candidate_summaries {
        // Semantic similarity placeholder (Phase 3: use embeddings)
        let semantic_sim = keyword_overlap_score(&keywords, &turn.topic_keywords);

        // Identity anchor boost (if turn is identity-critical)
        let anchor_boost = if turn.is_identity_critical { 1.0 } else { 0.0 };

        // Combined score
        turn.relevance_score =
            0.5 * turn.recency_score +
            0.3 * semantic_sim +
            0.2 * anchor_boost;
    }

    // Step 4: Sort by relevance, greedy token packing
    candidate_summaries.sort_by(|a, b|
        b.relevance_score.partial_cmp(&a.relevance_score).unwrap()
    );

    let mut selected = Vec::new();
    let mut total_tokens = 0;

    for turn in candidate_summaries {
        if total_tokens + turn.total_tokens <= max_tokens {
            selected.push(turn);
            total_tokens += turn.total_tokens;
        } else {
            break;
        }
    }

    // Sort selected by timestamp (chronological order for context)
    selected.sort_by_key(|t| t.timestamp);
    selected
}
```

### Relevance Scoring Details

**Recency Score (Exponential Decay):**
```
score_recency = e^(-λ * age_days)

Where:
  λ = 0.1 (decay constant, tunable)
  age_days = (now - turn.timestamp) / 86400

Examples:
  1 day old:   e^(-0.1 * 1)  ≈ 0.90
  7 days old:  e^(-0.1 * 7)  ≈ 0.50
  30 days old: e^(-0.1 * 30) ≈ 0.05
```

**Semantic Similarity (Phase 3: Embedding-Based):**
```
// Phase 1-2: Simple keyword overlap
score_semantic = |keywords_query ∩ keywords_turn| / |keywords_query ∪ keywords_turn|

// Phase 3: Cosine similarity using sentence embeddings
score_semantic = cosine_similarity(embed(user_message), embed(turn_summary))

Where:
  embed() = sentence-transformers model (e.g., all-MiniLM-L6-v2)
  cosine_similarity = dot(a, b) / (||a|| * ||b||)
```

**Identity Anchor Strength:**
```
score_anchor = {
  1.0 if turn.is_identity_critical
  0.5 if turn linked to identity_anchor
  0.0 otherwise
}
```

**Final Score:**
```
score_final =
  0.5 * score_recency +
  0.3 * score_semantic +
  0.2 * score_anchor

Range: [0.0, 1.0]
Threshold for loading: score_final > 0.3 (configurable)
```

### Token Budget Management

**Total Context Budget for LLM #2:**
```
LLM #2 context budget: 8000 tokens (assuming 8k context window)

Allocation:
  - System prompt: ~500 tokens
  - Framework state (domains, boundaries, BDE): ~800 tokens
  - User current message: ~300 tokens (avg)
  - Response generation buffer: ~2000 tokens
  - Available for conversation history: 8000 - 500 - 800 - 300 - 2000 = 4400 tokens

Memory tier allocation:
  - Hot memory: 1500 tokens (guaranteed)
  - Warm memory: 2000 tokens (on-demand)
  - Cold memory: 900 tokens (relevance-based)
  Total: 4400 tokens
```

**Greedy Token Packing:**
```rust
fn pack_turns_greedy(
    candidates: Vec<ConversationTurn>,
    max_tokens: usize
) -> Vec<ConversationTurn> {
    // Sort by relevance (already sorted in retrieval)
    let mut selected = Vec::new();
    let mut total = 0;

    for turn in candidates {
        if total + turn.total_tokens <= max_tokens {
            selected.push(turn);
            total += turn.total_tokens;
        } else {
            // Try to fit partial turn (user message only, skip AI response)
            if total + turn.user_message_tokens <= max_tokens {
                let partial_turn = ConversationTurn {
                    ai_response: "[Response truncated due to token limit]".to_string(),
                    ai_response_tokens: 10,
                    ..turn
                };
                selected.push(partial_turn);
                total += turn.user_message_tokens + 10;
            }
            break; // Budget exhausted
        }
    }

    selected
}
```

### Failure Modes and Graceful Degradation

**Scenario 1: Cold Retrieval Fails (DB timeout)**
```
Action: Continue with hot + warm memory only
Impact: User may not get reference to distant past conversation
Recovery: Log error, retry on next turn
```

**Scenario 2: Token Budget Exhausted**
```
Action: Prioritize hot > warm > cold, truncate cold if needed
Impact: Less historical context, but current conversation preserved
Recovery: Normal operation (not an error)
```

**Scenario 3: LLM #1 Compression Fails**
```
Action: Store turn verbatim in cold memory (no compression)
Impact: Higher storage cost, but no data loss
Recovery: Retry compression on next maintenance cycle
```

**Scenario 4: Session Boundary Ambiguous (user returns after 12h)**
```
Action: Create new session, but load relevant turns from previous session as cold memory
Impact: Seamless continuity for user ("you remember me!")
Recovery: Normal operation (design feature, not failure)
```

## CONVERSATION HISTORY

### How LLM #2 Receives History

**Context Construction:**
```xml
<system>
You are the Conscious Responder (LLM #2) for the Volumetric Integration Framework.

You have access to conversation history from memory tiers managed by the Unconscious LLM #1.
</system>

<framework_state>
  <domains>CD: 0.9, SD: 0.8, CuD: 0.6, ED: 0.7</domains>
  <boundaries>CD-SD: perm=0.85, status=Transitional</boundaries>
  <bde>
    Invitation: Notice tension between computational precision and scientific empiricism...
    Attention: Focus on the CD-SD interface...
    Resonance: Allow oscillatory synchronization...
    Emergence: Recognize emergent clarity...
  </bde>
</framework_state>

<conversation_history>
  <hot_memory>
    <turn number="45" timestamp="2025-11-01T10:23:15Z">
      <user>What's the weather today?</user>
      <ai>The weather is sunny with a high of 72°F.</ai>
    </turn>
    <turn number="46" timestamp="2025-11-01T10:24:01Z">
      <user>Can you remind me about my meeting schedule?</user>
      <ai>You have a team standup at 11am and a 1-on-1 with your manager at 2pm.</ai>
    </turn>
    <turn number="47" timestamp="2025-11-01T10:25:30Z">
      <user>Thanks! What did we discuss about quantum computing earlier?</user>
      <ai>[This is the current turn being processed]</ai>
    </turn>
  </hot_memory>

  <warm_memory>
    <!-- Recent turns from this session (if relevant) -->
  </warm_memory>

  <cold_memory>
    <retrieved_turn session="2025-10-25" relevance="0.92">
      <user>What do you think about quantum computing for drug discovery?</user>
      <ai>Quantum computing shows promise for molecular simulation due to variational quantum eigensolvers...</ai>
      <note>Identity anchor: User interested in quantum applications</note>
    </retrieved_turn>
  </cold_memory>
</conversation_history>

<identity_anchors>
  <anchor confidence="0.95">User name: Alex</anchor>
  <anchor confidence="0.90">User domain: Computational chemistry PhD</anchor>
  <anchor confidence="0.85">User interests: Quantum computing, drug discovery</anchor>
</identity_anchors>

<user_current_message>
Thanks! What did we discuss about quantum computing earlier?
</user_current_message>
```

**Formatting Rules:**
1. **Chronological order** - Oldest to newest (left-to-right temporal flow)
2. **Tier separation** - Clear boundaries between hot/warm/cold
3. **Metadata inclusion** - Timestamps, relevance scores, identity anchors
4. **Token efficiency** - No redundant information, concise formatting
5. **Graceful degradation** - If cold retrieval fails, omit section (don't break prompt)

### Handling Long Conversations (100+ Turns)

**Challenge:** Session with 150 turns = ~45,000 tokens (exceeds budget)

**Solution:**
```
1. Hot memory: Last 5 turns (always included)
2. Warm memory: Intelligent sampling
   - First turn of session (identity anchor)
   - Every 10th turn (session arc tracking)
   - Turns with high identity_anchor scores
   - Most recent 10 turns
   Total: ~15 turns from warm memory
3. Cold memory: Relevance-based (as usual)

Result: ~20 turns total in context instead of 150 turns
Token usage: ~6000 tokens instead of 45,000 tokens
```

**Implementation:**
```rust
fn sample_long_session(session_turns: Vec<ConversationTurn>, max_count: usize) -> Vec<ConversationTurn> {
    let total = session_turns.len();

    if total <= max_count {
        return session_turns; // No sampling needed
    }

    let mut selected = Vec::new();

    // Always include first turn (identity anchor)
    selected.push(session_turns[0].clone());

    // Include every Nth turn (stride sampling)
    let stride = total / max_count;
    for i in (stride..total).step_by(stride) {
        selected.push(session_turns[i].clone());
    }

    // Always include last 5 turns (recency)
    for i in (total.saturating_sub(5))..total {
        if !selected.iter().any(|t| t.id == session_turns[i].id) {
            selected.push(session_turns[i].clone());
        }
    }

    // Include identity-critical turns
    for turn in session_turns {
        if turn.is_identity_critical && !selected.iter().any(|t| t.id == turn.id) {
            selected.push(turn.clone());
        }
    }

    // Sort by turn_number to maintain chronological order
    selected.sort_by_key(|t| t.turn_number);

    // Trim to max_count if still over
    if selected.len() > max_count {
        selected.truncate(max_count);
    }

    selected
}
```

### Identity Continuity Across Sessions

**Scenario:** User returns after 1 week

**Session 1 (Oct 25):**
```
User: "My name is Alex and I'm working on a PhD in computational chemistry."
AI: "Nice to meet you, Alex! ..."
[40 more turns about quantum computing, drug discovery, etc.]
```

**Session 2 (Nov 1 - 7 days later):**
```
User: "Hey, do you remember what we discussed about quantum computing?"
```

**Memory Loading:**
1. **Hot memory:** Empty (new session)
2. **Warm memory:** Empty (new session)
3. **Cold memory retrieval triggered:**
   - Keyword match: "quantum computing"
   - User reference: "we discussed" (implies past session)
   - Identity anchor retrieval: User=Alex
4. **LLM #1 loads:**
   - Identity anchor: "User name: Alex, domain: computational chemistry"
   - Relevant turns from Oct 25 session (cold memory)
   - Compressed summary: "Previous session focused on quantum computing applications in drug discovery"

**LLM #2 receives:**
```xml
<identity_anchors>
  <anchor confidence="0.95" source="session_2025-10-25_turn_1">User name: Alex</anchor>
  <anchor confidence="0.90" source="session_2025-10-25_turn_1">User domain: Computational chemistry PhD</anchor>
</identity_anchors>

<cold_memory>
  <session_summary session_id="2025-10-25">
    Previous session (7 days ago) focused on quantum computing applications in drug discovery.
    Discussed variational quantum eigensolvers, molecular simulation challenges.
  </session_summary>

  <retrieved_turn session="2025-10-25" turn="12" relevance="0.94">
    <user>What do you think about quantum computing for drug discovery?</user>
    <ai>Quantum computing shows promise for molecular simulation...</ai>
  </retrieved_turn>
</cold_memory>

<user_current_message>
Hey, do you remember what we discussed about quantum computing?
</user_current_message>
```

**LLM #2 response:**
```
Yes, Alex! Last week we discussed quantum computing applications in drug discovery.
You were particularly interested in how variational quantum eigensolvers could help
with molecular simulation...
```

**Key Mechanisms:**
1. Identity anchors persist across sessions (never compressed away)
2. Cold memory retrieval triggered by keywords + past-tense language
3. Session summaries provide high-level continuity
4. Relevant turns retrieved for specificity

## INTEGRATION POINTS

### Where Memory Loading Happens (Dual-LLM Flow)

**Current Flow (No Memory System):**
```
1. User input → VIF API
2. Load latest snapshot (domains, boundaries, patterns)
3. Execute 7-stage flow
4. LLM #2 generates response
5. Save new snapshot
```

**Enhanced Flow (With Memory System):**
```
1. User input → VIF API
2. Load latest snapshot (domains, boundaries, patterns)
3. **[NEW] LLM #1 Memory Loading:**
   a. Load hot memory (last 3-5 turns)
   b. Check if warm/cold retrieval needed (keyword analysis)
   c. If needed, retrieve relevant warm/cold turns
   d. Construct conversation_history for LLM #2
   e. Return: MemoryContext { hot, warm, cold, identity_anchors }
4. Execute 7-stage flow (with MemoryContext)
5. Construct prompt for LLM #2 (includes conversation_history)
6. LLM #2 generates response (with full context)
7. **[NEW] LLM #1 Memory Saving:**
   a. Create new ConversationTurn (user_message + ai_response)
   b. Add to hot memory
   c. Check token budget, evict to warm if needed
   d. Save turn to database
8. Save state snapshot (linked to conversation_turn)
```

### Code Integration Points

**1. New Module: `api/src/memory_tiers.rs`**
```rust
pub struct MemoryTierManager {
    db_pool: SqlitePool,
    hot_cache: HashMap<Uuid, HotMemory>,  // session_id -> hot memory
}

impl MemoryTierManager {
    pub async fn load_memory_context(
        &self,
        session_id: Uuid,
        user_id: Uuid,
        user_message: &str,
    ) -> Result<MemoryContext, MemoryError> {
        // Load hot memory
        let hot = self.load_hot_memory(session_id).await?;

        // Determine if warm/cold retrieval needed
        let needs_retrieval = self.analyze_retrieval_need(user_message);

        let warm = if needs_retrieval {
            self.load_warm_memory(session_id, 2000).await?
        } else {
            vec![]
        };

        let cold = if needs_retrieval {
            self.load_cold_memory(user_id, user_message, 900).await?
        } else {
            vec![]
        };

        let identity_anchors = self.load_identity_anchors(user_id).await?;

        Ok(MemoryContext { hot, warm, cold, identity_anchors })
    }

    pub async fn save_conversation_turn(
        &self,
        session_id: Uuid,
        user_id: Uuid,
        user_message: String,
        ai_response: String,
        snapshot_id: Uuid,
    ) -> Result<ConversationTurn, MemoryError> {
        // Create turn
        let turn = ConversationTurn {
            id: Uuid::new_v4(),
            user_id,
            session_id,
            turn_number: self.get_next_turn_number(session_id).await?,
            user_message: user_message.clone(),
            ai_response: ai_response.clone(),
            user_message_tokens: estimate_tokens(&user_message),
            ai_response_tokens: estimate_tokens(&ai_response),
            total_tokens: estimate_tokens(&user_message) + estimate_tokens(&ai_response),
            timestamp: Utc::now().timestamp(),
            snapshot_id: Some(snapshot_id),
            memory_tier: "hot".to_string(),
            is_identity_critical: false, // LLM #1 will determine this later
        };

        // Save to DB
        self.insert_turn(&turn).await?;

        // Update hot cache
        self.update_hot_cache(session_id, turn.clone()).await?;

        Ok(turn)
    }
}
```

**2. Modified: `api/src/lib.rs` - Integration into VifApi**
```rust
pub struct VifApi {
    pub memory_manager: MemoryManager,
    pub memory_tiers: MemoryTierManager,  // NEW
    pub prompt_engine: PromptEngine,
    // ...
}

impl VifApi {
    pub async fn process_input(
        &mut self,
        user_input: &str,
        user_id: Uuid,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // 1. Get or create session
        let session_id = self.get_or_create_session(user_id).await?;

        // 2. Load memory context (NEW)
        let memory_context = self.memory_tiers
            .load_memory_context(session_id, user_id, user_input)
            .await?;

        // 3. Load latest snapshot
        let snapshot = self.memory_manager.get_latest_snapshot(user_id).await?;

        // 4. Initialize flow context (with memory)
        let flow_context = FlowContext {
            user_input: user_input.to_string(),
            memory_context: Some(memory_context),  // NEW
            // ...
        };

        // 5. Execute 7-stage flow
        let flow_result = self.execute_flow_process(flow_context).await?;

        // 6. Construct prompt for LLM #2 (includes conversation history)
        let prompt = self.construct_llm2_prompt(
            user_input,
            &flow_result,
            &memory_context,  // NEW
        );

        // 7. LLM #2 generates response
        let ai_response = self.llm_client.generate(prompt).await?;

        // 8. Save conversation turn (NEW)
        let turn = self.memory_tiers.save_conversation_turn(
            session_id,
            user_id,
            user_input.to_string(),
            ai_response.clone(),
            flow_result.snapshot_id,
        ).await?;

        // 9. Save state snapshot (with turn link)
        self.memory_manager.create_snapshot_with_qualities(
            flow_result.domains,
            flow_result.boundaries,
            flow_result.patterns,
            user_id,
            user_input,
            flow_result.qualities,
        ).await?;

        Ok(ai_response)
    }
}
```

**3. Modified: `api/src/flow_process.rs` - FlowContext Enhancement**
```rust
pub struct FlowContext {
    pub user_input: String,
    pub memory_context: Option<MemoryContext>,  // NEW
    pub framework_state: FrameworkState,
    // ...
}

pub struct MemoryContext {
    pub hot: Vec<ConversationTurn>,
    pub warm: Vec<ConversationTurn>,
    pub cold: Vec<ConversationTurn>,
    pub identity_anchors: Vec<IdentityAnchor>,
}
```

**4. Modified: `api/src/memory.rs` - Link to Turns**
```rust
impl MemoryManager {
    pub async fn create_snapshot_with_turn(
        &self,
        domains: Vec<DomainState>,
        boundaries: Vec<BoundaryState>,
        patterns: Vec<String>,
        user_id: Uuid,
        user_input: &str,
        qualities: [u8; 7],
        conversation_turn_id: Uuid,  // NEW
    ) -> Result<(), sqlx::Error> {
        // Save snapshot with link to conversation turn
        // ...
    }
}
```

### State Passing Between LLMs

**LLM #1 (Unconscious) → Rust API:**
```json
{
  "memory_context": {
    "hot_memory": [
      {"turn_id": "...", "user_message": "...", "ai_response": "...", "tokens": 300}
    ],
    "warm_memory_loaded": [
      {"turn_id": "...", "user_message": "...", "ai_response": "...", "relevance": 0.85}
    ],
    "cold_memory_loaded": [
      {"summary": "Previous discussion about quantum computing...", "relevance": 0.92}
    ],
    "identity_anchors": [
      {"anchor": "User name: Alex", "confidence": 0.95}
    ]
  },
  "token_allocation": {
    "hot": 1200,
    "warm": 1500,
    "cold": 800,
    "total": 3500
  }
}
```

**Rust API → LLM #2 (Conscious):**
```xml
<conversation_history>
  <hot_memory tokens="1200">
    <turn number="45">...</turn>
    <turn number="46">...</turn>
    <turn number="47">...</turn>
  </hot_memory>

  <warm_memory tokens="1500">
    <turn number="32" relevance="0.85">...</turn>
  </warm_memory>

  <cold_memory tokens="800">
    <summary session="2025-10-25" relevance="0.92">
      Previous discussion about quantum computing applications...
    </summary>
  </cold_memory>
</conversation_history>

<identity_anchors>
  <anchor confidence="0.95">User name: Alex</anchor>
  <anchor confidence="0.90">User domain: Computational chemistry PhD</anchor>
</identity_anchors>
```

**LLM #2 → Rust API (Response):**
```
Standard response (no memory management state, LLM #2 is stateless)
```

**Rust API → Database:**
```sql
-- Save new conversation turn
INSERT INTO conversation_turns (id, user_id, session_id, turn_number, user_message, ai_response, ...)
VALUES (...);

-- Update hot cache in-memory
hot_cache.insert(session_id, updated_hot_memory);

-- If session ended, trigger LLM #1 compression (async job)
compress_warm_to_cold(session_id);
```

## PHASED IMPLEMENTATION

### Phase 1: Hot Memory (Basic Conversation History)

**Duration:** 8 hours
**Goal:** Load last 3-5 conversation turns for LLM #2 context

**Tasks:**
1. **Database Schema (2 hours)**
   - Create migration `20251101000001_add_conversation_turns.sql`
   - Tables: `conversation_turns`, `conversation_sessions`
   - Run migration, verify schema

2. **Memory Tier Manager (3 hours)**
   - Create `api/src/memory_tiers.rs`
   - Implement `MemoryTierManager` struct
   - Methods:
     - `load_hot_memory(session_id) -> Vec<ConversationTurn>`
     - `save_conversation_turn(...) -> ConversationTurn`
     - `get_or_create_session(user_id) -> Uuid`
   - In-memory hot cache (HashMap)

3. **VifApi Integration (2 hours)**
   - Modify `api/src/lib.rs`
   - Add `memory_tiers: MemoryTierManager` field
   - Update `process_input()` to load/save turns
   - Construct conversation history for LLM #2 prompt

4. **Testing (1 hour)**
   - Test: `test_hot_memory_loading` - Verify last 3 turns loaded
   - Test: `test_hot_memory_eviction` - Verify oldest turn evicted when limit exceeded
   - Test: `test_session_creation` - Verify new session created for new user
   - All tests passing, 75%+ coverage maintained

**Validation Criteria:**
- [ ] Migration applied successfully
- [ ] Conversation turns saved to database
- [ ] Last 3-5 turns loaded for LLM #2
- [ ] LLM #2 prompt includes conversation history
- [ ] 3 new tests passing

**Dependencies:**
- Existing: `MemoryManager`, `VifApi`, database setup
- New: `conversation_turns` table, `MemoryTierManager`

**Files Modified:**
- `migrations/20251101000001_add_conversation_turns.sql` (new)
- `api/src/memory_tiers.rs` (new)
- `api/src/lib.rs` (modified)
- `api/src/flow_process.rs` (modified - add MemoryContext)

### Phase 2: Warm Memory (Recent Context Loading)

**Duration:** 12 hours
**Goal:** Load recent session turns (20-50) with recency scoring

**Tasks:**
1. **Memory Tier Field (1 hour)**
   - Create migration `20251101000002_add_tier_tracking.sql`
   - Add `memory_tier` column to `conversation_turns`
   - Add index on `memory_tier`
   - Update existing turns to 'warm' tier

2. **Warm Memory Loading (4 hours)**
   - Implement `load_warm_memory(session_id, max_tokens) -> Vec<ConversationTurn>`
   - Recency-based retrieval (ORDER BY turn_number DESC)
   - Greedy token packing algorithm
   - Hot→Warm eviction logic (when hot exceeds 5 turns)

3. **Tier Transition Logic (3 hours)**
   - Implement `evict_hot_to_warm(session_id, turn_ids)`
   - Update `memory_tier` column in database
   - Log transition in `memory_tier_transitions` table (future audit)
   - In-memory cache eviction

4. **Session Boundary Handling (2 hours)**
   - Detect session end (explicit or timeout-based)
   - Update `conversation_sessions.ended_at`
   - Mark warm turns for compression (Phase 3)

5. **Testing (2 hours)**
   - Test: `test_warm_memory_retrieval` - Verify recent turns loaded
   - Test: `test_hot_to_warm_eviction` - Verify tier transition
   - Test: `test_session_end_detection` - Verify session boundary
   - Test: `test_long_session_token_budget` - Verify greedy packing

**Validation Criteria:**
- [ ] Memory tier transitions working (hot→warm)
- [ ] Warm memory retrieval respects token budget
- [ ] Session boundaries detected correctly
- [ ] 4 new tests passing

**Dependencies:**
- Phase 1: Hot memory system
- Existing: Token estimation, session management

**Files Modified:**
- `migrations/20251101000002_add_tier_tracking.sql` (new)
- `api/src/memory_tiers.rs` (extended)
- `api/src/lib.rs` (extended - session end handling)

### Phase 3: Cold Memory (Compression + Retrieval)

**Duration:** 16 hours
**Goal:** Compress warm→cold, semantic retrieval

**Tasks:**
1. **Compression Tables (1 hour)**
   - Create migration `20251101000003_add_compression_tables.sql`
   - Tables: `conversation_summaries`, `memory_tier_transitions`
   - Indexes for search

2. **LLM #1 Compression Prompt (3 hours)**
   - Design compression prompt for LLM #1
   - Implement `compress_turn(turn) -> TurnSummary`
   - Identify identity-critical turns
   - Extract topic keywords

3. **Compression Pipeline (4 hours)**
   - Implement `compress_session_warm_to_cold(session_id)`
   - Triggered on session end
   - Batch process warm turns
   - LLM #1 generates summaries
   - Save to `conversation_summaries`
   - Delete original warm turns (or mark compressed)

4. **Cold Memory Retrieval (4 hours)**
   - Implement `load_cold_memory(user_id, user_message, max_tokens)`
   - Keyword-based search (Phase 3 initial implementation)
   - Recency scoring (exponential decay)
   - Combined relevance scoring
   - Greedy token packing

5. **Identity Anchor Integration (2 hours)**
   - Link `conversation_turns` to `identity_anchors` (existing table)
   - Retrieve identity anchors for user
   - Include in LLM #2 prompt

6. **Testing (2 hours)**
   - Test: `test_warm_to_cold_compression` - Verify summaries generated
   - Test: `test_cold_memory_retrieval` - Verify relevance ranking
   - Test: `test_identity_anchor_persistence` - Verify anchors survive compression
   - Test: `test_cross_session_continuity` - Verify user returns after 7 days, context loaded

**Validation Criteria:**
- [ ] Warm turns compressed to summaries
- [ ] Cold memory retrieval based on relevance
- [ ] Identity anchors persist across sessions
- [ ] Cross-session continuity working
- [ ] 4 new tests passing

**Dependencies:**
- Phase 2: Warm memory system
- Existing: LLM #1 client, identity anchors table

**Future Enhancements (Post-Phase 3):**
- Semantic search with embeddings (sentence-transformers)
- Full-text search index (FTS5)
- Adaptive tiering (user-specific thresholds)
- Compression quality metrics (ROUGE score vs original)

**Files Modified:**
- `migrations/20251101000003_add_compression_tables.sql` (new)
- `api/src/memory_tiers.rs` (extended)
- `api/src/llm_client.rs` (new compression prompts)
- `api/src/lib.rs` (session end triggers compression)

### Total Implementation Timeline

**Phase 1:** 8 hours (Hot memory)
**Phase 2:** 12 hours (Warm memory)
**Phase 3:** 16 hours (Cold memory)
**Total:** 36 hours (~5 working days)

**Checkpoint Milestones:**
- End of Phase 1: Basic conversation history working, users see immediate context
- End of Phase 2: Session-long continuity, token budgets respected
- End of Phase 3: Full dual-LLM memory system, identity continuity across sessions

## DEPENDENCIES ON OTHERS

| Needs_From | What | Why | Blocking |
|------------|------|-----|----------|
| Dual-LLM Implementation Lead | LLM #1 integration specs | Need to call LLM #1 for compression decisions | Phase 3 |
| Dual-LLM Implementation Lead | LLM #2 prompt format | Need to know how to inject conversation history | Phase 1 |
| Database Team (self) | Migration execution | Need to apply schema changes | Phase 1 |
| Token Optimization Agent | Token estimation function | Accurate token counting for budget management | Phase 1 |
| Identity Anchor Agent | Anchor creation logic | Determine what makes turn identity-critical | Phase 3 |
| Prompt Engineering Agent | LLM #1 compression prompt | Need effective summarization prompt | Phase 3 |

**Non-Blocking Dependencies (Can proceed with placeholders):**
- Semantic search embeddings (use keyword search initially)
- LLM #1 actual API calls (can stub with mock responses)
- Full-text search (can use LIKE queries initially)

## SUCCESS CRITERIA

**Functional Requirements:**
1. [ ] Hot memory loads last 3-5 turns in <1ms
2. [ ] Warm memory loads recent session in <50ms
3. [ ] Cold memory retrieves relevant past in <200ms
4. [ ] Conversation turns persist across sessions
5. [ ] Identity anchors survive compression
6. [ ] Token budgets respected (no context overflow)
7. [ ] Graceful degradation on retrieval failures

**Quality Requirements:**
1. [ ] All tests passing (11 new tests across 3 phases)
2. [ ] Test coverage maintained at 75%+
3. [ ] Zero dead code (all methods used)
4. [ ] Proper error handling (Result types, no unwrap())
5. [ ] Database migrations applied successfully

**User Experience Requirements:**
1. [ ] Seamless continuity (user doesn't notice memory system)
2. [ ] "Remembers" recent context (hot memory always loaded)
3. [ ] "Recalls" past discussions when relevant (cold retrieval)
4. [ ] Identity consistency ("my name is Alex" persists)
5. [ ] No visible "loading context..." delays

**Performance Requirements:**
1. [ ] Hot memory: <1ms latency (in-memory cache)
2. [ ] Warm memory: <50ms latency (single DB query)
3. [ ] Cold memory: <200ms latency (semantic search + DB)
4. [ ] Compression: <5s per session (async, not user-blocking)
5. [ ] Storage efficiency: ~70% reduction via compression

**Validation Metrics:**
```
Phase 1 Success:
  - 3 new tests passing
  - Conversation history visible in LLM #2 prompts
  - Database contains conversation_turns records

Phase 2 Success:
  - 4 new tests passing (7 total)
  - Hot→Warm eviction working
  - Session boundaries detected
  - Token budgets enforced

Phase 3 Success:
  - 4 new tests passing (11 total)
  - Warm→Cold compression working
  - Cold retrieval by relevance working
  - Cross-session continuity validated
  - Identity anchors persisting

Overall Success:
  - 11/11 tests passing
  - All phases integrated
  - Dual-LLM flow enhanced with memory
  - Users experience seamless continuity
  - Production-ready memory system
```

---

**END OF DESIGN DOCUMENT**

**Status:** Complete - Ready for implementation
**Next Steps:** Coordinate with Dual-LLM Implementation Lead, begin Phase 1
**Estimated Total Effort:** 36 hours (5 working days)
