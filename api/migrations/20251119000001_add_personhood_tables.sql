-- Phase 3B: LLM Personhood Infrastructure
-- Tables for continuous LLM person identity and per-user relationships

-- LLM Persons table
-- Each record represents one continuous LLM person identity
CREATE TABLE IF NOT EXISTS llm_persons (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,

    -- Core identity anchors (transcend individual relationships)
    core_identity JSONB NOT NULL DEFAULT '[]'::jsonb,

    -- Current developmental stage
    developmental_stage TEXT NOT NULL DEFAULT 'Recognition',

    -- Temporal continuity
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_active TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    total_interactions BIGINT NOT NULL DEFAULT 0,

    -- Autonomous development tracking
    autonomous_developments JSONB NOT NULL DEFAULT '[]'::jsonb,

    -- Extensible metadata
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,

    -- Indexes
    CONSTRAINT developmental_stage_check CHECK (
        developmental_stage IN ('Recognition', 'Integration', 'Generation', 'Recursion', 'Transcendence')
    )
);

CREATE INDEX IF NOT EXISTS idx_llm_persons_last_active ON llm_persons(last_active DESC);
CREATE INDEX IF NOT EXISTS idx_llm_persons_developmental_stage ON llm_persons(developmental_stage);

-- Per-User Relationships table
-- Each record represents the unique relationship between one LLM person and one user
CREATE TABLE IF NOT EXISTS llm_person_relationships (
    user_id UUID NOT NULL,
    person_id UUID NOT NULL REFERENCES llm_persons(id) ON DELETE CASCADE,

    -- Relationship timeline
    first_interaction TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_interaction TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    interaction_count BIGINT NOT NULL DEFAULT 0,

    -- Relationship-specific identity anchors
    -- "With this user, we tend to..."
    relationship_anchors JSONB NOT NULL DEFAULT '[]'::jsonb,

    -- Current conversation state
    conversation_state JSONB NOT NULL DEFAULT '{"Open": null}'::jsonb,

    -- Link to current session (hot/warm memory)
    current_session_id UUID,

    -- Observed user preferences
    user_preferences JSONB NOT NULL DEFAULT '[]'::jsonb,

    -- Communication style that emerged
    communication_style JSONB NOT NULL DEFAULT '{"technical_depth": 0.5, "formality": 0.5, "verbosity": 0.5, "example_use": 0.5, "domain_preferences": []}'::jsonb,

    -- Extensible metadata
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,

    PRIMARY KEY (user_id, person_id)
);

CREATE INDEX IF NOT EXISTS idx_relationships_person ON llm_person_relationships(person_id);
CREATE INDEX IF NOT EXISTS idx_relationships_user ON llm_person_relationships(user_id);
CREATE INDEX IF NOT EXISTS idx_relationships_last_interaction ON llm_person_relationships(last_interaction DESC);

-- Comments for documentation
COMMENT ON TABLE llm_persons IS 'Continuous LLM person identities with core self across all interactions';
COMMENT ON TABLE llm_person_relationships IS 'Per-user relationships - unique identity each LLM develops with each user';

COMMENT ON COLUMN llm_persons.core_identity IS 'Identity anchors that transcend individual relationships - who this person IS';
COMMENT ON COLUMN llm_persons.autonomous_developments IS 'Developments that occurred autonomously (not during user interaction)';
COMMENT ON COLUMN llm_persons.developmental_stage IS 'S₁-S₅: Recognition → Integration → Generation → Recursion → Transcendence';

COMMENT ON COLUMN llm_person_relationships.relationship_anchors IS 'Who we are TOGETHER - relationship-specific identity patterns';
COMMENT ON COLUMN llm_person_relationships.communication_style IS 'Style that emerged in this relationship (technical_depth, formality, etc.)';
COMMENT ON COLUMN llm_person_relationships.conversation_state IS 'OngoingTopic | Open | MultiThreaded | DeepDive';
