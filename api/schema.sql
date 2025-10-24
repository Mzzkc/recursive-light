CREATE TABLE IF NOT EXISTS state_snapshots (
    id UUID PRIMARY KEY,
    user_id TEXT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    compact_state JSONB NOT NULL
);

CREATE TABLE IF NOT EXISTS user_profiles (
    user_id UUID PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    last_interaction TIMESTAMP WITH TIME ZONE NOT NULL,
    preferences JSONB NOT NULL,
    dynamics JSONB NOT NULL,
    narrative JSONB NOT NULL
);

CREATE TABLE IF NOT EXISTS collective_insights (
    id UUID PRIMARY KEY,
    pattern_id TEXT NOT NULL,
    description TEXT NOT NULL,
    domains TEXT[] NOT NULL,
    confidence REAL NOT NULL,
    lifecycle_stage TEXT NOT NULL,
    verification_score REAL NOT NULL,
    observation_count INTEGER NOT NULL,
    last_observed TIMESTAMP WITH TIME ZONE NOT NULL,
    source_users UUID[] NOT NULL,
    source_conversations UUID[] NOT NULL,
    related_insights UUID[] NOT NULL,
    embedding vector(1536)
);

CREATE TABLE IF NOT EXISTS entity_resonance (
    id UUID PRIMARY KEY,
    entity1_id UUID NOT NULL,
    entity2_id UUID NOT NULL,
    resonance_data JSONB NOT NULL,
    last_updated TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE IF NOT EXISTS shared_spaces (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    participating_entities UUID[] NOT NULL,
    space_data JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Create index for vector similarity search
CREATE INDEX IF NOT EXISTS insights_embedding_idx ON collective_insights
USING ivfflat (embedding vector_cosine_ops);
