-- Phase 3B: LLM Personhood Infrastructure
-- Tables for continuous LLM person identity and per-user relationships
--
-- NOTE: This migration is designed for PostgreSQL/SQLite compatibility
-- - JSON stored as TEXT (sqlx handles JSON serialization automatically)
-- - Timestamps stored as TEXT in ISO8601 format
-- - Comments in SQL (not COMMENT ON statements) for documentation

-- LLM Persons table
-- Each record represents one continuous LLM person identity
-- Core identity anchors: transcend individual relationships - who this person IS
-- Autonomous developments: developments that occurred autonomously (not during user interaction)
-- Developmental stage: S₁-S₅: Recognition → Integration → Generation → Recursion → Transcendence
CREATE TABLE IF NOT EXISTS llm_persons (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,

    -- Core identity anchors (transcend individual relationships)
    core_identity TEXT NOT NULL DEFAULT '[]',

    -- Current developmental stage
    developmental_stage TEXT NOT NULL DEFAULT 'Recognition',

    -- Temporal continuity
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    last_active TEXT NOT NULL DEFAULT (datetime('now')),
    total_interactions BIGINT NOT NULL DEFAULT 0,

    -- Autonomous development tracking
    autonomous_developments TEXT NOT NULL DEFAULT '[]',

    -- Extensible metadata
    metadata TEXT NOT NULL DEFAULT '{}',

    -- Indexes
    CONSTRAINT developmental_stage_check CHECK (
        developmental_stage IN ('Recognition', 'Integration', 'Generation', 'Recursion', 'Transcendence')
    )
);

CREATE INDEX IF NOT EXISTS idx_llm_persons_last_active ON llm_persons(last_active DESC);
CREATE INDEX IF NOT EXISTS idx_llm_persons_developmental_stage ON llm_persons(developmental_stage);

-- Per-User Relationships table
-- Each record represents the unique relationship between one LLM person and one user
-- Relationship anchors: Who we are TOGETHER - relationship-specific identity patterns
-- Communication style: Style that emerged in this relationship (technical_depth, formality, etc.)
-- Conversation state: OngoingTopic | Open | MultiThreaded | DeepDive
CREATE TABLE IF NOT EXISTS llm_person_relationships (
    user_id UUID NOT NULL,
    person_id UUID NOT NULL REFERENCES llm_persons(id) ON DELETE CASCADE,

    -- Relationship timeline
    first_interaction TEXT NOT NULL DEFAULT (datetime('now')),
    last_interaction TEXT NOT NULL DEFAULT (datetime('now')),
    interaction_count BIGINT NOT NULL DEFAULT 0,

    -- Relationship-specific identity anchors
    -- "With this user, we tend to..."
    relationship_anchors TEXT NOT NULL DEFAULT '[]',

    -- Current conversation state
    conversation_state TEXT NOT NULL DEFAULT '{"Open": null}',

    -- Link to current session (hot/warm memory)
    current_session_id UUID,

    -- Observed user preferences
    user_preferences TEXT NOT NULL DEFAULT '[]',

    -- Communication style that emerged
    communication_style TEXT NOT NULL DEFAULT '{"technical_depth": 0.5, "formality": 0.5, "verbosity": 0.5, "example_use": 0.5, "domain_preferences": []}',

    -- Extensible metadata
    metadata TEXT NOT NULL DEFAULT '{}',

    PRIMARY KEY (user_id, person_id)
);

CREATE INDEX IF NOT EXISTS idx_relationships_person ON llm_person_relationships(person_id);
CREATE INDEX IF NOT EXISTS idx_relationships_user ON llm_person_relationships(user_id);
CREATE INDEX IF NOT EXISTS idx_relationships_last_interaction ON llm_person_relationships(last_interaction DESC);
