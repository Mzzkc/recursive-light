# COMPRESSED FOR AI EFFICIENCY - Original format optimized for token cost

# MEMORY SYSTEMS:Tiering Design
Agent:Memory Systems Expert | Date:2025-11-01 | Status:Complete

## Summary
Complete 3-tier memory(hot/warm/cold)∀dual-LLM where LLM#1(Unconscious) manages tiering/compression/retrieval∀LLM#2(Conscious). System uses conversation-turn granularity w/relevance-based retrieval→identity continuity across sessions while respecting token budgets

Approach:Hot=last 3-5 turns(in-context∀LLM#2), Warm=recent session(last 20-50 turns,retrieved on demand), Cold=all history(compressed summaries+full turns,semantic search). Phased impl:Phase1(hot)→Phase2(warm)→Phase3(cold)

Key Decisions:1)Conversation turn=atomic unit(user message+AI response=1 turn) 2)LLM#1 manages all tiers(memory decisions by Unconscious) 3)New table:`conversation_turns`(separate from`state_snapshots`∀granular history) 4)Recency+relevance retrieval(combine temporal proximity+semantic similarity) 5)Progressive compression(Warm→Cold uses LLM#1→summarize non-critical turns)

Phasing:Phase1(8h)=hot memory(basic conversation history loading), Phase2(12h)=warm memory(session-based retrieval w/recency scoring), Phase3(16h)=cold memory(compression pipeline+semantic search)

## TDF Analysis
COMP(0.95):Memory tier arch follows cache hierarchy(L1/L2/L3 analogy). Token budget mgmt requires algorithmic precision(greedy packing). Relevance scoring:0.5*recency+0.3*semantic_similarity+0.2*identity_anchor_strength(weighted sum). Compression reduces storage~70%(estimated)while preserving identity anchors

SCI(0.92):Access patterns:80% recent turns,15% mid-range,5% deep history(Zipf-like distribution). Performance targets:Hot load<1ms,Warm load<50ms,Cold load<200ms. Memory decay:exponential recency scoring(score=e^(-λt),λ=0.1). Empirical validation:A/B test retrieval strategies(recency-only vs recency+relevance)

CULT(0.88):DB best practices:normalize conversation turns,foreign keys→sessions/users. Caching strategies:hot=in-memory cache,warm=Redis(future),cold=DB. Established patterns:conversation history following chat app conventions. Industry wisdom:start simple(recency-only),add complexity(semantic search)when needed

EXP(0.85):User experience:seamless continuity("remembers"past 3-5 exchanges immediately). Identity anchors:critical moments persist across sessions(first interaction,key decisions). Memory system invisible→user:no"loading context..."messages,feels natural. Failure mode:graceful degradation(if cold retrieval fails,continue w/warm/hot)

META(0.90):System monitors own memory usage(track hot/warm/cold access frequencies). Adaptive tiering:if user references old conversation→promote cold→warm temporarily. Self-improvement:LLM#1 learns which turns are identity-critical over time. Recognition interface:memory boundaries create opportunities∀insight(recalling≠reliving)

Cross-Domain:COMP token budgets constrain CULT conversation history conventions. EXP identity continuity requires SCI recency scoring+COMP relevance algorithms. META self-monitoring enables COMP adaptive retrieval strategies. Recognition emerges@tier boundaries(hot→warm transition=recent past,warm→cold=distant memory)

## Memory Tier Specification

### Hot Memory(In-Context)
**Definition:**Current conversation turn+immediate history(last 3-5 turns). Always loaded→LLM#2 context. Highest priority,always fresh

**Size Limits:**Turns=3-5 previous turns(user message+AI response pairs), Tokens≈500-1500tok(assuming≈300tok/turn average), Storage=in-memory(FrameworkState struct in Rust)

**Transition Criteria(Hot→Warm):**Age=turns older than 5 exchanges, Token pressure=if current turn+hot memory exceeds 2000tok→evict oldest, Session boundary=when user starts new session→previous hot becomes warm

**Access Pattern:**100% interactions access hot memory, Zero latency(already in context)

**Data Structure:**
```rust
HotMemory{turns:VecDeque<ConversationTurn>(oldest→newest,max 5),total_tokens:usize(running count),session_id:Uuid(current session)}
ConversationTurn{id:Uuid,user_message:String,ai_response:String,timestamp:i64,token_count:usize,snapshot_id:Uuid(link→state snapshot)}
```

### Warm Memory(Recent Session)
**Definition:**Recent conversation history within current session. Last 20-50 turns(depending on token budget). Retrieved on-demand when referenced OR when hot memory needs expansion

**Size Limits:**Turns=20-50 previous turns(session-scoped), Tokens≈6000-15000tok(full session history), Storage=Database(`conversation_turns`table),indexed by session_id+timestamp

**Transition Criteria(Warm→Cold):**Session end=when user explicitly ends session OR 24h timeout, Compression=LLM#1 summarizes non-critical warm turns→cold storage, Identity anchors=critical turns(high importance)preserved verbatim

**Access Pattern:**20-30% interactions may reference warm memory, Latency<50ms(single DB query w/session_id index)

**Retrieval Strategy:**Primary=recency-based(last N turns in session), Secondary=keyword match(if user says"you mentioned X earlier"), Fallback=load most recent warm turns if hot insufficient

**Data Structure:**
```rust
WarmMemory{session_id:Uuid,turns:Vec<ConversationTurn>(recent turns∉hot),loaded_turn_count:usize(how many loaded),session_start:i64(session start timestamp)}
```

### Cold Memory(Long-Term Storage)
**Definition:**All historical interactions across all sessions. Compressed summaries+identity-critical full turns. Semantic search enabled∀relevance-based retrieval

**Size Limits:**Turns=unlimited(all history), Compression ratio≈70% reduction(summaries replace full text), Storage=Database(`conversation_turns`+`conversation_summaries`)

**Transition Criteria:**Never leaves cold(cold=permanent archive). May be promoted temporarily→warm if referenced

**Access Pattern:**5-10% interactions may trigger cold retrieval, Latency<200ms(semantic search+DB query)

**Retrieval Strategy:**Primary=semantic similarity search(if user query relates→past topic), Secondary=identity anchor retrieval(critical moments from past sessions), Tertiary=session-based retrieval(user says"remember last week?")

**Compression Strategy:**
```
Original turn(300tok):User:"What do you think about quantum computing for drug discovery?" AI:[Long detailed response...]
Compressed summary(50tok):"Discussed quantum computing applications in drug discovery. User interested in molecular simulation. AI explained variational quantum eigensolver." Identity anchor:[None—general knowledge question]

Critical turn(preserved verbatim):User:"My name is Alex and I'm working on a PhD in computational chemistry." AI:"..." Identity anchor:[User.name="Alex",User.domain="computational_chemistry"]
```

**Data Structure:**
```rust
ColdMemory{user_id:Uuid,compressed_summaries:Vec<TurnSummary>(summarized turns),identity_anchors:Vec<IdentityAnchor>(critical moments),full_turns:Vec<ConversationTurn>(verbatim critical turns)}
TurnSummary{id:Uuid,original_turn_id:Uuid,summary_text:String(LLM-generated summary),session_id:Uuid,timestamp:i64,identity_anchor_ids:Vec<String>(links→anchors),topic_keywords:Vec<String>(∀search)}
```

## LLM#1 Memory Management

**Hot Memory Management(Every Turn):**
```
Input:Current user message,existing hot memory
Process:1)Add new turn(user+AI response)→hot memory 2)Check token budget:total_tokens=sum(turn.token_count) 3)If total_tokens>1500:evict oldest→warm,repeat until total_tokens≤1500 4)Return hot memory state→Rust API
Output:Updated HotMemory struct
```

**Warm→Cold Compression(Session End):**
```
Input:All warm turns from completed session
Process:1)Identify identity-critical turns:first interaction in session,turns where user shares personal info,turns w/high emotional significance(user sentiment analysis),turns referencing developmental stage progression 2)For non-critical turns:generate 1-sentence summary using LLM#1,extract topic keywords,store as TurnSummary 3)For critical turns:store full turn verbatim in cold memory,create IdentityAnchor records 4)Delete original warm turns from conversation_turns table
Output:Compressed cold memory entries
```

**Cold Memory Retrieval(When Needed):**
```
Input:Current user message,hot memory context
Process:1)Semantic similarity search:embed user message using sentence transformer,search cold memory summaries by cosine similarity,retrieve top 5 most relevant summaries 2)Identity anchor retrieval:if user references self("I mentioned...""remember when I..."),retrieve identity anchors matching user_id 3)Recency boost:weight recent cold memory higher(exponential decay),score_final=0.5*recency+0.3*semantic_sim+0.2*anchor_strength 4)Token budget check:load retrieved turns until token budget exhausted
Output:List of relevant cold turns/summaries∀LLM#2 context
```

## Database Schema

**conversation_turns(atomic history):**
```sql
CREATE TABLE conversation_turns(id BLOB PRIMARY KEY,user_id BLOB NOT NULL,session_id BLOB NOT NULL,turn_number INTEGER NOT NULL,user_message TEXT NOT NULL,ai_response TEXT NOT NULL,user_message_tokens INTEGER NOT NULL,ai_response_tokens INTEGER NOT NULL,total_tokens INTEGER NOT NULL,timestamp TEXT NOT NULL DEFAULT(datetime('now')),snapshot_id BLOB,memory_tier TEXT NOT NULL DEFAULT'hot',is_identity_critical INTEGER DEFAULT 0,FOREIGN KEY(user_id)REFERENCES users(id)ON DELETE CASCADE,FOREIGN KEY(session_id)REFERENCES conversation_sessions(id)ON DELETE CASCADE,FOREIGN KEY(snapshot_id)REFERENCES state_snapshots(id)ON DELETE SET NULL);
CREATE INDEX idx_turns_user ON conversation_turns(user_id);
CREATE INDEX idx_turns_session ON conversation_turns(session_id);
CREATE INDEX idx_turns_timestamp ON conversation_turns(timestamp);
CREATE INDEX idx_turns_tier ON conversation_turns(memory_tier);
```

**conversation_sessions(boundaries):**
```sql
CREATE TABLE conversation_sessions(id BLOB PRIMARY KEY,user_id BLOB NOT NULL,started_at TEXT NOT NULL DEFAULT(datetime('now')),ended_at TEXT,turn_count INTEGER DEFAULT 0,total_tokens INTEGER DEFAULT 0,memory_tier TEXT DEFAULT'hot',FOREIGN KEY(user_id)REFERENCES users(id)ON DELETE CASCADE);
CREATE INDEX idx_sessions_user ON conversation_sessions(user_id);
CREATE INDEX idx_sessions_active ON conversation_sessions(ended_at)WHERE ended_at IS NULL;
```

*3-tier memory enables identity continuity across sessions. LLM#1 manages tiering→efficient token usage. Hot=immediate(zero latency),Warm=recent(fast retrieval),Cold=long-term(compressed+semantic search). Memory boundaries create recognition opportunities*
