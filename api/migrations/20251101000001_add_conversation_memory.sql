-- Conversation Memory Tables for Dual-LLM Architecture (Phase 0)
-- Hot/Warm/Cold memory tiering for LLM #1 context
-- SQLite-compatible (will work with PostgreSQL too)

-- Conversation sessions table
-- Groups conversation turns into sessions (typically 1 session per day)
CREATE TABLE IF NOT EXISTS conversation_sessions (
    id BLOB PRIMARY KEY NOT NULL,
    user_id BLOB NOT NULL,
    started_at TEXT NOT NULL DEFAULT (datetime('now')),
    ended_at TEXT,  -- NULL if session is still active
    turn_count INTEGER NOT NULL DEFAULT 0,
    -- Session metadata
    session_summary TEXT,  -- Summary generated when session ends
    total_tokens INTEGER DEFAULT 0,  -- Total tokens used in this session
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_sessions_user ON conversation_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_sessions_started ON conversation_sessions(started_at);
CREATE INDEX IF NOT EXISTS idx_sessions_ended ON conversation_sessions(ended_at);

-- Conversation turns table
-- Individual user message + AI response pairs
CREATE TABLE IF NOT EXISTS conversation_turns (
    id BLOB PRIMARY KEY NOT NULL,
    session_id BLOB NOT NULL,
    user_id BLOB NOT NULL,
    turn_number INTEGER NOT NULL,  -- Sequential within session (1, 2, 3, ...)
    -- User input
    user_message TEXT NOT NULL,
    user_timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    -- AI response
    ai_response TEXT,  -- NULL if not yet generated
    ai_timestamp TEXT,
    -- Flow process tracking
    snapshot_id BLOB,  -- References state_snapshots(id) for this turn
    flow_execution_id BLOB,  -- References flow_process_executions(id)
    -- Memory tier tracking (for hot/warm/cold transitions)
    memory_tier TEXT NOT NULL DEFAULT 'hot',  -- 'hot', 'warm', 'cold', 'archived'
    tier_changed_at TEXT,  -- When tier last changed
    -- Token usage
    input_tokens INTEGER DEFAULT 0,
    output_tokens INTEGER DEFAULT 0,
    -- Metadata
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (session_id) REFERENCES conversation_sessions(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (snapshot_id) REFERENCES state_snapshots(id) ON DELETE SET NULL,
    FOREIGN KEY (flow_execution_id) REFERENCES flow_process_executions(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_turns_session ON conversation_turns(session_id);
CREATE INDEX IF NOT EXISTS idx_turns_user ON conversation_turns(user_id);
CREATE INDEX IF NOT EXISTS idx_turns_number ON conversation_turns(session_id, turn_number);
CREATE INDEX IF NOT EXISTS idx_turns_timestamp ON conversation_turns(created_at);
CREATE INDEX IF NOT EXISTS idx_turns_tier ON conversation_turns(memory_tier);

-- Memory tier transitions table
-- Tracks when turns move between hot/warm/cold tiers
CREATE TABLE IF NOT EXISTS memory_tier_transitions (
    id BLOB PRIMARY KEY NOT NULL,
    turn_id BLOB NOT NULL,
    from_tier TEXT NOT NULL,
    to_tier TEXT NOT NULL,
    transitioned_at TEXT NOT NULL DEFAULT (datetime('now')),
    reason TEXT,  -- Why the transition happened (e.g., "age", "session_end", "manual")
    FOREIGN KEY (turn_id) REFERENCES conversation_turns(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_transitions_turn ON memory_tier_transitions(turn_id);
CREATE INDEX IF NOT EXISTS idx_transitions_timestamp ON memory_tier_transitions(transitioned_at);

-- Conversation summaries table
-- Compressed summaries of warm/cold memory for token efficiency
CREATE TABLE IF NOT EXISTS conversation_summaries (
    id BLOB PRIMARY KEY NOT NULL,
    session_id BLOB,  -- NULL if summary spans multiple sessions
    user_id BLOB NOT NULL,
    -- Summary content
    summary_text TEXT NOT NULL,
    -- What this summary covers
    turn_ids TEXT NOT NULL,  -- JSON array of turn IDs that were summarized
    turn_count INTEGER NOT NULL,
    start_timestamp TEXT NOT NULL,
    end_timestamp TEXT NOT NULL,
    -- Identity-critical flags
    contains_identity_anchor INTEGER DEFAULT 0,  -- 1 if summary mentions user name, preferences, etc.
    anchor_keywords TEXT,  -- JSON array of identity keywords found
    -- Compression metrics
    original_tokens INTEGER NOT NULL,  -- Total tokens before compression
    compressed_tokens INTEGER NOT NULL,  -- Tokens in summary
    compression_ratio REAL,  -- original / compressed
    -- Metadata
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (session_id) REFERENCES conversation_sessions(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_summaries_user ON conversation_summaries(user_id);
CREATE INDEX IF NOT EXISTS idx_summaries_session ON conversation_summaries(session_id);
CREATE INDEX IF NOT EXISTS idx_summaries_timestamp ON conversation_summaries(created_at);
CREATE INDEX IF NOT EXISTS idx_summaries_identity ON conversation_summaries(contains_identity_anchor);
