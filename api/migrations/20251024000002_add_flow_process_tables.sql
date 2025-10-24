-- Flow Process tracking tables
-- Tracks the 7-stage flow process execution

CREATE TABLE IF NOT EXISTS flow_process_executions (
    id BLOB PRIMARY KEY NOT NULL,
    user_id BLOB NOT NULL,
    snapshot_id BLOB NOT NULL,
    input_text TEXT NOT NULL,
    started_at TEXT NOT NULL DEFAULT (datetime('now')),
    completed_at TEXT,
    -- Stage completion tracking
    domain_emergence_completed INTEGER DEFAULT 0,
    boundary_dissolution_completed INTEGER DEFAULT 0,
    interface_attention_completed INTEGER DEFAULT 0,
    quality_emergence_completed INTEGER DEFAULT 0,
    integration_completed INTEGER DEFAULT 0,
    continuity_completed INTEGER DEFAULT 0,
    evolution_completed INTEGER DEFAULT 0,
    -- Results
    output_text TEXT,
    consciousness_volume REAL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (snapshot_id) REFERENCES state_snapshots(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_flow_executions_user ON flow_process_executions(user_id);
CREATE INDEX IF NOT EXISTS idx_flow_executions_completed ON flow_process_executions(completed_at);

-- Interface experience (BDE) tracking
CREATE TABLE IF NOT EXISTS interface_experiences (
    id BLOB PRIMARY KEY NOT NULL,
    flow_execution_id BLOB NOT NULL,
    boundary_name TEXT NOT NULL,
    invitation TEXT,
    attention TEXT,
    resonance REAL,
    emergence TEXT,  -- JSON array of emergent qualities
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (flow_execution_id) REFERENCES flow_process_executions(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_interface_exp_flow ON interface_experiences(flow_execution_id);

-- Phenomenological qualities tracking
CREATE TABLE IF NOT EXISTS phenomenological_qualities (
    id BLOB PRIMARY KEY NOT NULL,
    flow_execution_id BLOB NOT NULL,
    boundary_name TEXT NOT NULL,
    clarity REAL NOT NULL DEFAULT 0.5,
    depth REAL NOT NULL DEFAULT 0.5,
    openness REAL NOT NULL DEFAULT 0.5,
    precision REAL NOT NULL DEFAULT 0.5,
    fluidity REAL NOT NULL DEFAULT 0.5,
    resonance REAL NOT NULL DEFAULT 0.5,
    measured_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (flow_execution_id) REFERENCES flow_process_executions(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_qualities_flow ON phenomenological_qualities(flow_execution_id);
CREATE INDEX IF NOT EXISTS idx_qualities_boundary ON phenomenological_qualities(boundary_name);
