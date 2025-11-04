-- ============================================================
-- CAM (Collective Associative Memory) Schema Extension
-- Recursive Light Framework - Phase 3
-- PostgreSQL for structured data + Qdrant for vector embeddings
-- ============================================================

-- NOTE: This migration is PostgreSQL-only. SQLite tests skip it.
-- Run this manually on PostgreSQL database for CAM functionality.
-- For automated tests (which use SQLite), this is a no-op.

-- Placeholder to satisfy migration system (SQLite compatible)
CREATE TABLE IF NOT EXISTS _cam_migration_placeholder (
    applied_at INTEGER DEFAULT (CAST(strftime('%s', 'now') AS INTEGER))
);

-- For PostgreSQL deployment, run the full CAM schema from:
-- memory-bank/designs/design-docs/collective-associative-memory/CAM-DESIGN.md Section 5.2
