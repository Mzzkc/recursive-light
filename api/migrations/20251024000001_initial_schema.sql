-- Initial schema for Recursive Light Framework
-- SQLite-compatible migration (will work with PostgreSQL too)

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id BLOB PRIMARY KEY NOT NULL,  -- UUID stored as BLOB in SQLite
    provider TEXT NOT NULL,
    provider_id TEXT NOT NULL,
    email TEXT NOT NULL,
    name TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    last_login TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(provider, provider_id)
);

-- State snapshots table
CREATE TABLE IF NOT EXISTS state_snapshots (
    id BLOB PRIMARY KEY NOT NULL,
    user_id BLOB NOT NULL,
    timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    -- Compact state stored as JSON
    domain_states TEXT NOT NULL,  -- JSON blob
    boundary_states TEXT NOT NULL,  -- JSON blob
    pattern_ids TEXT NOT NULL,  -- JSON array of pattern IDs
    identity_anchors TEXT,  -- JSON array
    metadata TEXT,  -- Additional metadata as JSON
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_snapshots_user ON state_snapshots(user_id);
CREATE INDEX IF NOT EXISTS idx_snapshots_timestamp ON state_snapshots(timestamp);

-- User profiles table
CREATE TABLE IF NOT EXISTS user_profiles (
    user_id BLOB PRIMARY KEY NOT NULL,
    preferences TEXT NOT NULL DEFAULT '{}',  -- JSON
    dynamics TEXT NOT NULL DEFAULT '{}',  -- JSON
    narrative TEXT NOT NULL DEFAULT '{}',  -- JSON
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Collective insights table
CREATE TABLE IF NOT EXISTS collective_insights (
    id BLOB PRIMARY KEY NOT NULL,
    pattern_id TEXT NOT NULL,
    description TEXT NOT NULL,
    domains TEXT NOT NULL,  -- JSON array
    confidence REAL NOT NULL,
    lifecycle_stage TEXT NOT NULL,
    verification_score REAL NOT NULL,
    observation_count INTEGER NOT NULL DEFAULT 1,
    last_observed TEXT NOT NULL DEFAULT (datetime('now')),
    source_users TEXT NOT NULL,  -- JSON array of user IDs
    source_conversations TEXT NOT NULL DEFAULT '[]',  -- JSON array
    related_insights TEXT NOT NULL DEFAULT '[]',  -- JSON array of insight IDs
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_insights_pattern ON collective_insights(pattern_id);
CREATE INDEX IF NOT EXISTS idx_insights_lifecycle ON collective_insights(lifecycle_stage);
CREATE INDEX IF NOT EXISTS idx_insights_confidence ON collective_insights(confidence);

-- Patterns table (for tracking pattern evolution)
CREATE TABLE IF NOT EXISTS patterns (
    id TEXT PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    lifecycle_stage TEXT NOT NULL DEFAULT 'potential',
    confidence REAL NOT NULL DEFAULT 0.5,
    domains TEXT NOT NULL,  -- JSON array
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_patterns_lifecycle ON patterns(lifecycle_stage);
CREATE INDEX IF NOT EXISTS idx_patterns_confidence ON patterns(confidence);
