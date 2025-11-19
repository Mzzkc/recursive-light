-- ============================================================
-- CAM (Collective Associative Memory) Schema Extension
-- Recursive Light Framework - Phase 3
-- PostgreSQL for metadata + Qdrant for vector embeddings
-- ============================================================

-- ARCHITECTURE: Qdrant + PostgreSQL Hybrid
-- - Qdrant: 1536-dim embeddings (OpenAI ada-002), HNSW semantic search
-- - PostgreSQL: Insight metadata, hypergraph relationships, validations
-- - Link: Shared insight_id (UUID) across both systems

-- NOTE: This migration creates PostgreSQL tables only.
-- Qdrant collection must be created separately via qdrant_storage.rs
-- SQLite tests use placeholder table (CAM requires PostgreSQL + Qdrant)

-- Placeholder for SQLite compatibility (tests)
CREATE TABLE IF NOT EXISTS _cam_migration_placeholder (
    applied_at INTEGER DEFAULT (CAST(strftime('%s', 'now') AS INTEGER))
);

-- PostgreSQL-specific schema (run manually or via migration tool):
-- See: memory-bank/designs/design-docs/collective-associative-memory/CAM-DESIGN.md Section 5.2
-- Key difference from original design: NO embedding field (stored in Qdrant instead)
