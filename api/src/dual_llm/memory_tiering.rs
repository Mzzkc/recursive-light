// Memory Tiering System for Dual-LLM Architecture
// Manages hot/warm/cold conversation history tiers
// Phase 1A: Hot Memory Implementation (last 3-5 turns)
// Phase 1B: Warm Memory Implementation (session-scoped, 20-50 turns)
// Phase 1C: Cold Memory Implementation (cross-session, all history)

use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, Row, SqlitePool};
use std::collections::VecDeque;
// Wave 1: Proper BM25 implementation
use bm25::{Document, Language, SearchEngineBuilder};

/// Phase 2D: Significance scoring for intelligent retrieval ranking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnSignificance {
    pub recency_score: f32,        // Exponential temporal decay
    pub semantic_relevance: f32,   // Keyword overlap with query
    pub identity_criticality: f32, // Is this identity-forming? (0.0-1.0)
    pub emotional_weight: f32,     // Sentiment/intensity (reserved for Phase 3)
    pub factual_density: f32,      // Proper nouns, dates, facts (reserved for Phase 3)
    pub narrative_importance: f32, // Milestone, decision point (reserved for Phase 3)
}

impl TurnSignificance {
    /// Calculate combined significance score (weighted sum)
    pub fn combined_score(&self) -> f32 {
        // Phase 2D: Use implemented fields only
        0.5 * self.recency_score + 0.35 * self.semantic_relevance + 0.15 * self.identity_criticality
        // Phase 3 will add: + 0.10 * (emotional + factual + narrative)
    }

    /// Create default significance (lowest scores)
    pub fn default_significance() -> Self {
        Self {
            recency_score: 0.0,
            semantic_relevance: 0.0,
            identity_criticality: 0.0,
            emotional_weight: 0.0,
            factual_density: 0.0,
            narrative_importance: 0.0,
        }
    }
}

/// Conversation turn: one user message + AI response pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTurn {
    pub id: Uuid,
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub turn_number: i32,
    pub user_message: String,
    pub ai_response: String,
    pub user_timestamp: String,
    pub ai_timestamp: Option<String>,
    pub snapshot_id: Option<Uuid>,
    pub input_tokens: i32,
    pub output_tokens: i32,
}

/// Hot memory cache: last 3-5 turns always in-context for LLM #2
#[derive(Debug, Clone)]
pub struct HotMemory {
    pub turns: VecDeque<ConversationTurn>,
    pub total_tokens: usize,
    pub session_id: Uuid,
}

impl HotMemory {
    /// Maximum number of turns to keep in hot memory
    const MAX_TURNS: usize = 5;

    /// Maximum total tokens for hot memory (budget constraint)
    const MAX_TOKENS: usize = 1500;

    pub fn new(session_id: Uuid) -> Self {
        Self {
            turns: VecDeque::with_capacity(Self::MAX_TURNS),
            total_tokens: 0,
            session_id,
        }
    }

    /// Add a turn to hot memory, evicting oldest if necessary
    pub fn add_turn(&mut self, turn: ConversationTurn) {
        let turn_tokens = (turn.input_tokens + turn.output_tokens) as usize;

        // Evict oldest turns if we exceed limits
        while self.turns.len() >= Self::MAX_TURNS
            || (self.total_tokens + turn_tokens > Self::MAX_TOKENS && !self.turns.is_empty())
        {
            if let Some(evicted) = self.turns.pop_front() {
                self.total_tokens -= (evicted.input_tokens + evicted.output_tokens) as usize;
            }
        }

        self.total_tokens += turn_tokens;
        self.turns.push_back(turn);
    }

    /// Format conversation history for LLM #2 context
    pub fn format_for_llm(&self) -> String {
        if self.turns.is_empty() {
            return String::new();
        }

        let mut formatted = String::from("# Recent Conversation History\n\n");
        for turn in &self.turns {
            formatted.push_str(&format!("User: {}\n", turn.user_message));
            formatted.push_str(&format!("Assistant: {}\n\n", turn.ai_response));
        }
        formatted
    }
}

/// Warm memory: recent session history (turns not in hot, up to 50 total)
#[derive(Debug, Clone)]
pub struct WarmMemory {
    pub session_id: Uuid,
    pub turns: Vec<ConversationTurn>,
    pub loaded_turn_count: usize,
    pub total_tokens: usize,
}

impl WarmMemory {
    /// Maximum turns to load from warm memory
    const MAX_TURNS: usize = 50;

    /// Maximum tokens for warm memory (budget constraint)
    const MAX_TOKENS: usize = 15000;

    pub fn new(session_id: Uuid) -> Self {
        Self {
            session_id,
            turns: Vec::new(),
            loaded_turn_count: 0,
            total_tokens: 0,
        }
    }

    /// Add turn to warm memory (only if within limits)
    pub fn add_turn(&mut self, turn: ConversationTurn) -> bool {
        let turn_tokens = (turn.input_tokens + turn.output_tokens) as usize;

        if self.turns.len() >= Self::MAX_TURNS || self.total_tokens + turn_tokens > Self::MAX_TOKENS
        {
            return false; // Would exceed limits
        }

        self.total_tokens += turn_tokens;
        self.turns.push(turn);
        self.loaded_turn_count += 1;
        true
    }

    /// Format warm memory for LLM #2 context (less prominent than hot)
    pub fn format_for_llm(&self) -> String {
        if self.turns.is_empty() {
            return String::new();
        }

        let mut formatted = String::from("# Earlier in this session\n\n");
        for turn in &self.turns {
            formatted.push_str(&format!(
                "[Turn {}] User: {}\n",
                turn.turn_number, turn.user_message
            ));
            formatted.push_str(&format!(
                "[Turn {}] Assistant: {}\n\n",
                turn.turn_number, turn.ai_response
            ));
        }
        formatted
    }
}

/// Cold memory: long-term storage across all sessions
#[derive(Debug, Clone)]
pub struct ColdMemory {
    pub user_id: Uuid,
    pub turns: Vec<ConversationTurn>,
    pub total_sessions: usize,
    pub total_turns: usize,
}

impl ColdMemory {
    /// Maximum turns to load from cold memory per query
    const MAX_TURNS: usize = 100;

    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            turns: Vec::new(),
            total_sessions: 0,
            total_turns: 0,
        }
    }

    /// Add turn to cold memory
    pub fn add_turn(&mut self, turn: ConversationTurn) {
        self.turns.push(turn);
        self.total_turns += 1;
    }

    /// Format cold memory for LLM #2 context (most condensed)
    pub fn format_for_llm(&self) -> String {
        if self.turns.is_empty() {
            return String::new();
        }

        let mut formatted = String::from("# Past Conversations\n\n");
        for turn in &self.turns {
            formatted.push_str(&format!(
                "[{} - Turn {}] User: {}\n",
                turn.user_timestamp.split('T').next().unwrap_or(""),
                turn.turn_number,
                turn.user_message
            ));
            formatted.push_str(&format!(
                "[{} - Turn {}] Assistant: {}\n\n",
                turn.ai_timestamp.as_deref().unwrap_or(""),
                turn.turn_number,
                turn.ai_response
            ));
        }
        formatted
    }
}

/// Memory tier manager: manages hot/warm/cold conversation history
pub struct MemoryTierManager {
    db_pool: SqlitePool,
    // Wave 1: Cache for snapshot identity criticality to avoid repeated DB queries
    identity_cache: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<Uuid, bool>>>,
}

impl MemoryTierManager {
    /// Create new MemoryTierManager with existing database pool
    pub fn new(db_pool: SqlitePool) -> Self {
        Self {
            db_pool,
            identity_cache: std::sync::Arc::new(tokio::sync::RwLock::new(
                std::collections::HashMap::new(),
            )),
        }
    }

    /// Create new MemoryTierManager from database URL
    pub async fn from_url(database_url: &str) -> Result<Self, sqlx::Error> {
        let db_pool = SqlitePool::connect(database_url).await?;
        Ok(Self {
            db_pool,
            identity_cache: std::sync::Arc::new(tokio::sync::RwLock::new(
                std::collections::HashMap::new(),
            )),
        })
    }

    /// Get reference to database pool for sharing with PersonManager
    pub fn pool(&self) -> &SqlitePool {
        &self.db_pool
    }

    /// Get or create active session for user
    /// Returns session_id
    pub async fn get_or_create_session(&self, user_id: Uuid) -> Result<Uuid, sqlx::Error> {
        // Check for active session (ended_at is NULL)
        let result = sqlx::query(
            "SELECT id FROM conversation_sessions
             WHERE user_id = ?1 AND ended_at IS NULL
             ORDER BY started_at DESC LIMIT 1",
        )
        .bind(user_id)
        .fetch_optional(&self.db_pool)
        .await?;

        if let Some(row) = result {
            let session_id: Uuid = row.get("id");
            Ok(session_id)
        } else {
            // Create new session
            let new_session_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO conversation_sessions (id, user_id, started_at, turn_count, total_tokens)
                 VALUES (?1, ?2, datetime('now'), 0, 0)",
            )
            .bind(new_session_id)
            .bind(user_id)
            .execute(&self.db_pool)
            .await?;
            Ok(new_session_id)
        }
    }

    /// Load hot memory for session: last 3-5 turns
    pub async fn load_hot_memory(&self, session_id: Uuid) -> Result<HotMemory, sqlx::Error> {
        let mut hot_memory = HotMemory::new(session_id);

        let rows = sqlx::query(
            "SELECT id, session_id, user_id, turn_number, user_message, ai_response,
                    user_timestamp, ai_timestamp, snapshot_id, input_tokens, output_tokens
             FROM conversation_turns
             WHERE session_id = ?1 AND ai_response IS NOT NULL
             ORDER BY turn_number DESC
             LIMIT 5",
        )
        .bind(session_id)
        .fetch_all(&self.db_pool)
        .await?;

        // Collect turns in reverse order (DESC query returns newest first, we want oldest first)
        let mut turns: Vec<ConversationTurn> = rows
            .into_iter()
            .map(|row| ConversationTurn {
                id: row.get("id"),
                session_id: row.get("session_id"),
                user_id: row.get("user_id"),
                turn_number: row.get("turn_number"),
                user_message: row.get("user_message"),
                ai_response: row.get("ai_response"),
                user_timestamp: row.get("user_timestamp"),
                ai_timestamp: row.get("ai_timestamp"),
                snapshot_id: row.get("snapshot_id"),
                input_tokens: row.get("input_tokens"),
                output_tokens: row.get("output_tokens"),
            })
            .collect();

        turns.reverse();

        for turn in turns {
            hot_memory.add_turn(turn);
        }

        Ok(hot_memory)
    }

    /// Load warm memory for session: turns older than hot memory (up to 50 turns)
    /// Excludes the 5 most recent turns (which are in hot memory)
    pub async fn load_warm_memory(&self, session_id: Uuid) -> Result<WarmMemory, sqlx::Error> {
        let mut warm_memory = WarmMemory::new(session_id);

        let rows = sqlx::query(
            "SELECT id, session_id, user_id, turn_number, user_message, ai_response,
                    user_timestamp, ai_timestamp, snapshot_id, input_tokens, output_tokens
             FROM conversation_turns
             WHERE session_id = ?1 AND ai_response IS NOT NULL
             ORDER BY turn_number DESC
             LIMIT 50 OFFSET 5",
        )
        .bind(session_id)
        .fetch_all(&self.db_pool)
        .await?;

        // Collect turns in chronological order (oldest first)
        let mut turns: Vec<ConversationTurn> = rows
            .into_iter()
            .map(|row| ConversationTurn {
                id: row.get("id"),
                session_id: row.get("session_id"),
                user_id: row.get("user_id"),
                turn_number: row.get("turn_number"),
                user_message: row.get("user_message"),
                ai_response: row.get("ai_response"),
                user_timestamp: row.get("user_timestamp"),
                ai_timestamp: row.get("ai_timestamp"),
                snapshot_id: row.get("snapshot_id"),
                input_tokens: row.get("input_tokens"),
                output_tokens: row.get("output_tokens"),
            })
            .collect();

        turns.reverse(); // Oldest first for warm memory

        for turn in turns {
            if !warm_memory.add_turn(turn) {
                break; // Reached capacity
            }
        }

        Ok(warm_memory)
    }

    /// Search warm memory by keyword (case-insensitive)
    /// Returns turns matching the keyword in either user or AI message
    pub async fn search_warm_memory(
        &self,
        session_id: Uuid,
        keyword: &str,
        limit: usize,
    ) -> Result<Vec<ConversationTurn>, sqlx::Error> {
        let search_pattern = format!("%{}%", keyword);

        let rows = sqlx::query(
            "SELECT id, session_id, user_id, turn_number, user_message, ai_response,
                    user_timestamp, ai_timestamp, snapshot_id, input_tokens, output_tokens
             FROM conversation_turns
             WHERE session_id = ?1
               AND ai_response IS NOT NULL
               AND (LOWER(user_message) LIKE LOWER(?2) OR LOWER(ai_response) LIKE LOWER(?2))
             ORDER BY turn_number DESC
             LIMIT ?3",
        )
        .bind(session_id)
        .bind(&search_pattern)
        .bind(limit as i32)
        .fetch_all(&self.db_pool)
        .await?;

        let turns = rows
            .into_iter()
            .map(|row| ConversationTurn {
                id: row.get("id"),
                session_id: row.get("session_id"),
                user_id: row.get("user_id"),
                turn_number: row.get("turn_number"),
                user_message: row.get("user_message"),
                ai_response: row.get("ai_response"),
                user_timestamp: row.get("user_timestamp"),
                ai_timestamp: row.get("ai_timestamp"),
                snapshot_id: row.get("snapshot_id"),
                input_tokens: row.get("input_tokens"),
                output_tokens: row.get("output_tokens"),
            })
            .collect();

        Ok(turns)
    }

    /// Save a conversation turn to database
    #[allow(clippy::too_many_arguments)]
    pub async fn save_conversation_turn(
        &self,
        session_id: Uuid,
        user_id: Uuid,
        user_message: &str,
        ai_response: &str,
        snapshot_id: Option<Uuid>,
        input_tokens: i32,
        output_tokens: i32,
    ) -> Result<Uuid, sqlx::Error> {
        let turn_id = Uuid::new_v4();

        // Get next turn number for session
        let turn_number: i32 = sqlx::query_scalar(
            "SELECT COALESCE(MAX(turn_number), 0) + 1 FROM conversation_turns WHERE session_id = ?1",
        )
        .bind(session_id)
        .fetch_one(&self.db_pool)
        .await?;

        // Insert conversation turn
        sqlx::query(
            "INSERT INTO conversation_turns
             (id, session_id, user_id, turn_number, user_message, ai_response,
              user_timestamp, ai_timestamp, snapshot_id, memory_tier,
              input_tokens, output_tokens, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, datetime('now'), datetime('now'), ?7, 'hot', ?8, ?9, datetime('now'))",
        )
        .bind(turn_id)
        .bind(session_id)
        .bind(user_id)
        .bind(turn_number)
        .bind(user_message)
        .bind(ai_response)
        .bind(snapshot_id)
        .bind(input_tokens)
        .bind(output_tokens)
        .execute(&self.db_pool)
        .await?;

        // Update session turn count
        sqlx::query(
            "UPDATE conversation_sessions SET turn_count = turn_count + 1, total_tokens = total_tokens + ?2
             WHERE id = ?1",
        )
        .bind(session_id)
        .bind(input_tokens + output_tokens)
        .execute(&self.db_pool)
        .await?;

        Ok(turn_id)
    }

    /// End current session (sets ended_at timestamp)
    pub async fn end_session(&self, session_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE conversation_sessions SET ended_at = datetime('now') WHERE id = ?1")
            .bind(session_id)
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }

    /// Load cold memory for user: turns from completed sessions (memory_tier = 'cold')
    /// Returns up to MAX_TURNS most recent cold turns across all user's sessions
    pub async fn load_cold_memory(&self, user_id: Uuid) -> Result<ColdMemory, sqlx::Error> {
        let mut cold_memory = ColdMemory::new(user_id);

        // Get count of unique sessions with cold turns
        let session_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(DISTINCT session_id) FROM conversation_turns
             WHERE user_id = ?1 AND memory_tier = 'cold'",
        )
        .bind(user_id)
        .fetch_one(&self.db_pool)
        .await?;

        cold_memory.total_sessions = session_count as usize;

        // Load most recent cold turns (up to MAX_TURNS)
        let rows = sqlx::query(
            "SELECT id, session_id, user_id, turn_number, user_message, ai_response,
                    user_timestamp, ai_timestamp, snapshot_id, input_tokens, output_tokens
             FROM conversation_turns
             WHERE user_id = ?1 AND memory_tier = 'cold' AND ai_response IS NOT NULL
             ORDER BY created_at DESC
             LIMIT ?2",
        )
        .bind(user_id)
        .bind(ColdMemory::MAX_TURNS as i32)
        .fetch_all(&self.db_pool)
        .await?;

        let mut turns: Vec<ConversationTurn> = rows
            .into_iter()
            .map(|row| ConversationTurn {
                id: row.get("id"),
                session_id: row.get("session_id"),
                user_id: row.get("user_id"),
                turn_number: row.get("turn_number"),
                user_message: row.get("user_message"),
                ai_response: row.get("ai_response"),
                user_timestamp: row.get("user_timestamp"),
                ai_timestamp: row.get("ai_timestamp"),
                snapshot_id: row.get("snapshot_id"),
                input_tokens: row.get("input_tokens"),
                output_tokens: row.get("output_tokens"),
            })
            .collect();

        turns.reverse(); // Oldest first for chronological reading

        for turn in turns {
            cold_memory.add_turn(turn);
        }

        Ok(cold_memory)
    }

    /// Update memory tier for a specific turn
    pub async fn update_memory_tier(
        &self,
        turn_id: Uuid,
        new_tier: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE conversation_turns
             SET memory_tier = ?1, tier_changed_at = datetime('now')
             WHERE id = ?2",
        )
        .bind(new_tier)
        .bind(turn_id)
        .execute(&self.db_pool)
        .await?;

        // Record tier transition
        sqlx::query(
            "INSERT INTO memory_tier_transitions (id, turn_id, from_tier, to_tier, transitioned_at, reason)
             SELECT ?1, ?2,
                    (SELECT memory_tier FROM conversation_turns WHERE id = ?2),
                    ?3,
                    datetime('now'),
                    'manual_transition'
             FROM conversation_turns WHERE id = ?2 LIMIT 1",
        )
        .bind(Uuid::new_v4())
        .bind(turn_id)
        .bind(new_tier)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    /// Transition all warm memory from a session to cold (called on session end)
    pub async fn transition_warm_to_cold(&self, session_id: Uuid) -> Result<usize, sqlx::Error> {
        // Get all warm/hot turns from this session (excluding already cold)
        let turn_ids: Vec<Uuid> = sqlx::query_scalar(
            "SELECT id FROM conversation_turns
             WHERE session_id = ?1 AND memory_tier IN ('hot', 'warm')",
        )
        .bind(session_id)
        .fetch_all(&self.db_pool)
        .await?;

        let count = turn_ids.len();

        // Update all to cold
        sqlx::query(
            "UPDATE conversation_turns
             SET memory_tier = 'cold', tier_changed_at = datetime('now')
             WHERE session_id = ?1 AND memory_tier IN ('hot', 'warm')",
        )
        .bind(session_id)
        .execute(&self.db_pool)
        .await?;

        // Record transitions
        for turn_id in turn_ids {
            sqlx::query(
                "INSERT INTO memory_tier_transitions (id, turn_id, from_tier, to_tier, transitioned_at, reason)
                 VALUES (?1, ?2, 'warm', 'cold', datetime('now'), 'session_end')",
            )
            .bind(Uuid::new_v4())
            .bind(turn_id)
            .execute(&self.db_pool)
            .await?;
        }

        Ok(count)
    }

    /// Search cold memory by keyword across all sessions
    pub async fn search_cold_memory(
        &self,
        user_id: Uuid,
        keyword: &str,
        limit: usize,
    ) -> Result<Vec<ConversationTurn>, sqlx::Error> {
        let search_pattern = format!("%{}%", keyword);

        let rows = sqlx::query(
            "SELECT id, session_id, user_id, turn_number, user_message, ai_response,
                    user_timestamp, ai_timestamp, snapshot_id, input_tokens, output_tokens
             FROM conversation_turns
             WHERE user_id = ?1
               AND memory_tier = 'cold'
               AND ai_response IS NOT NULL
               AND (LOWER(user_message) LIKE LOWER(?2) OR LOWER(ai_response) LIKE LOWER(?2))
             ORDER BY created_at DESC
             LIMIT ?3",
        )
        .bind(user_id)
        .bind(&search_pattern)
        .bind(limit as i32)
        .fetch_all(&self.db_pool)
        .await?;

        let turns = rows
            .into_iter()
            .map(|row| ConversationTurn {
                id: row.get("id"),
                session_id: row.get("session_id"),
                user_id: row.get("user_id"),
                turn_number: row.get("turn_number"),
                user_message: row.get("user_message"),
                ai_response: row.get("ai_response"),
                user_timestamp: row.get("user_timestamp"),
                ai_timestamp: row.get("ai_timestamp"),
                snapshot_id: row.get("snapshot_id"),
                input_tokens: row.get("input_tokens"),
                output_tokens: row.get("output_tokens"),
            })
            .collect();

        Ok(turns)
    }

    /// Wave 1: Batch check identity criticality for multiple snapshots (with caching)
    /// Returns HashMap of snapshot_id -> is_identity_critical
    async fn batch_check_identity_criticality(
        &self,
        snapshot_ids: Vec<Uuid>,
    ) -> Result<std::collections::HashMap<Uuid, bool>, sqlx::Error> {
        let mut results = std::collections::HashMap::new();

        if snapshot_ids.is_empty() {
            return Ok(results);
        }

        // Check cache first
        {
            let cache = self.identity_cache.read().await;
            for snapshot_id in &snapshot_ids {
                if let Some(&is_critical) = cache.get(snapshot_id) {
                    results.insert(*snapshot_id, is_critical);
                }
            }
        }

        // Find snapshot_ids not in cache
        let uncached_ids: Vec<Uuid> = snapshot_ids
            .into_iter()
            .filter(|id| !results.contains_key(id))
            .collect();

        if !uncached_ids.is_empty() {
            // Build query to check identity_anchors for all uncached snapshots
            // A snapshot is identity-critical if identity_anchors is non-null and non-empty
            let placeholders = uncached_ids
                .iter()
                .enumerate()
                .map(|(i, _)| format!("?{}", i + 1))
                .collect::<Vec<_>>()
                .join(",");

            let query_str = format!(
                "SELECT id, identity_anchors FROM state_snapshots WHERE id IN ({})",
                placeholders
            );

            let mut query = sqlx::query(&query_str);
            for id in &uncached_ids {
                query = query.bind(id);
            }

            let rows = query.fetch_all(&self.db_pool).await?;

            // Update cache and results
            let mut cache = self.identity_cache.write().await;
            for row in rows {
                let snapshot_id: Uuid = row.get("id");
                let identity_anchors: Option<String> = row.get("identity_anchors");

                // Identity critical if anchors exist and not empty/null
                let is_critical = identity_anchors
                    .map(|anchors| !anchors.is_empty() && anchors != "[]" && anchors != "null")
                    .unwrap_or(false);

                cache.insert(snapshot_id, is_critical);
                results.insert(snapshot_id, is_critical);
            }

            // For snapshot_ids that weren't found in DB, mark as not critical
            for id in uncached_ids {
                results.entry(id).or_insert_with(|| {
                    cache.insert(id, false);
                    false
                });
            }
        }

        Ok(results)
    }

    /// Wave 1: Rank conversation turns by BM25 + temporal decay + significance
    /// Now uses proper BM25 with IDF and avgdl calculations from corpus
    /// AND proper identity criticality lookup from database
    pub async fn rank_turns_by_relevance(
        &self,
        turns: Vec<ConversationTurn>,
        query: &str,
    ) -> Vec<(ConversationTurn, TurnSignificance)> {
        if turns.is_empty() {
            return Vec::new();
        }

        // Wave 1: Pre-fetch identity criticality for all snapshots
        let snapshot_ids: Vec<Uuid> = turns.iter().filter_map(|turn| turn.snapshot_id).collect();

        let identity_map = self
            .batch_check_identity_criticality(snapshot_ids)
            .await
            .unwrap_or_default(); // Fallback to empty map on error

        // Build BM25 search engine from corpus of turns
        // Each turn becomes a document combining user message + AI response
        let documents: Vec<Document<usize>> = turns
            .iter()
            .enumerate()
            .map(|(idx, turn)| {
                let contents = format!("{} {}", turn.user_message, turn.ai_response);
                Document { id: idx, contents }
            })
            .collect();

        // Build search engine with English language tokenization using with_documents
        // This automatically calculates proper IDF and avgdl from the corpus
        let search_engine =
            SearchEngineBuilder::<usize>::with_documents(Language::English, documents).build();

        // Query the search engine once for all turns
        let search_results = search_engine.search(query, turns.len());

        // Build a map of turn index -> BM25 score for fast lookup
        let mut bm25_scores = std::collections::HashMap::new();
        for result in search_results {
            bm25_scores.insert(result.document.id, result.score);
        }

        // Calculate significance for each turn using proper BM25 scores and identity data
        let mut scored_turns: Vec<(ConversationTurn, TurnSignificance)> = turns
            .into_iter()
            .enumerate()
            .map(|(idx, turn)| {
                // Get BM25 score from search results (0.0 if term not found)
                let bm25_score = bm25_scores.get(&idx).copied().unwrap_or(0.0);

                // Get identity criticality from pre-fetched data
                let identity_critical = turn
                    .snapshot_id
                    .and_then(|id| identity_map.get(&id).copied())
                    .unwrap_or(false);

                let significance =
                    self.calculate_turn_significance(&turn, bm25_score, identity_critical);
                (turn, significance)
            })
            .collect();

        // Sort by combined significance score (descending)
        scored_turns.sort_by(|a, b| {
            b.1.combined_score()
                .partial_cmp(&a.1.combined_score())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        scored_turns
    }

    /// Wave 1: Calculate significance for a single turn using pre-computed data
    fn calculate_turn_significance(
        &self,
        turn: &ConversationTurn,
        bm25_score: f32,
        identity_critical: bool,
    ) -> TurnSignificance {
        // 1. Recency score (exponential temporal decay)
        let recency_score = self.calculate_recency_score(&turn.user_timestamp);

        // 2. Semantic relevance (from proper BM25 with IDF and avgdl)
        let semantic_relevance = bm25_score;

        // 3. Identity criticality (Wave 1: ✅ IMPLEMENTED - DB lookup with caching)
        // 1.0 if identity-critical snapshot, 0.0 otherwise
        let identity_criticality = if identity_critical { 1.0 } else { 0.0 };

        TurnSignificance {
            recency_score,
            semantic_relevance,
            identity_criticality,
            emotional_weight: 0.0,     // Reserved for Phase 3
            factual_density: 0.0,      // Reserved for Phase 3
            narrative_importance: 0.0, // Reserved for Phase 3
        }
    }

    /// Phase 2D: Calculate recency score with exponential temporal decay
    /// Formula: e^(-λ * days_ago) where λ = 0.01
    fn calculate_recency_score(&self, timestamp_str: &str) -> f32 {
        use chrono::{DateTime, Utc};

        // Parse timestamp
        let parsed = DateTime::parse_from_rfc3339(timestamp_str).or_else(|_| {
            chrono::NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S")
                .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc).fixed_offset())
        });

        let timestamp = match parsed {
            Ok(dt) => dt.with_timezone(&Utc),
            Err(_) => return 0.5, // Fallback: medium recency
        };

        let now = Utc::now();
        let duration = now.signed_duration_since(timestamp);
        let days_ago = duration.num_days() as f32;

        // Exponential decay: λ = 0.01 (COMP domain recommendation)
        let lambda = 0.01;
        (-lambda * days_ago).exp()
    }

    // Wave 1: Removed calculate_bm25_score and tokenize methods
    // Now using proper BM25 implementation from bm25 crate in rank_turns_by_relevance
    // This provides:
    // - Proper IDF calculation from corpus statistics
    // - Proper avgdl calculation from actual document lengths
    // - Professional tokenization with stemming and stop word removal
    // - O(m*log(n)) performance with inverted index
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    async fn create_test_user(pool: &SqlitePool) -> Uuid {
        let user_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO users (id, provider, provider_id, email, name, created_at)
             VALUES (?1, 'test', ?2, ?3, ?4, datetime('now'))",
        )
        .bind(user_id)
        .bind("test-provider-id")
        .bind("test@example.com")
        .bind("Test User")
        .execute(pool)
        .await
        .unwrap();
        user_id
    }

    #[tokio::test]
    async fn test_get_or_create_session_creates_new() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let manager = MemoryTierManager::new(pool.clone());

        let session_id = manager.get_or_create_session(user_id).await.unwrap();
        assert!(!session_id.is_nil());

        // Verify session exists in database
        let count: i32 =
            sqlx::query_scalar("SELECT COUNT(*) FROM conversation_sessions WHERE id = ?1")
                .bind(session_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_get_or_create_session_returns_existing() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let manager = MemoryTierManager::new(pool);

        let session_id_1 = manager.get_or_create_session(user_id).await.unwrap();
        let session_id_2 = manager.get_or_create_session(user_id).await.unwrap();

        // Should return same session (ended_at is NULL)
        assert_eq!(session_id_1, session_id_2);
    }

    #[tokio::test]
    async fn test_save_conversation_turn() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let manager = MemoryTierManager::new(pool.clone());

        let session_id = manager.get_or_create_session(user_id).await.unwrap();

        let turn_id = manager
            .save_conversation_turn(
                session_id,
                user_id,
                "Hello, how are you?",
                "I'm doing well, thank you for asking!",
                None,
                10,
                15,
            )
            .await
            .unwrap();

        assert!(!turn_id.is_nil());

        // Verify turn exists with correct data
        let row = sqlx::query(
            "SELECT user_message, ai_response, turn_number FROM conversation_turns WHERE id = ?1",
        )
        .bind(turn_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let user_msg: String = row.get("user_message");
        let ai_msg: String = row.get("ai_response");
        let turn_num: i32 = row.get("turn_number");

        assert_eq!(user_msg, "Hello, how are you?");
        assert_eq!(ai_msg, "I'm doing well, thank you for asking!");
        assert_eq!(turn_num, 1);
    }

    #[tokio::test]
    async fn test_load_hot_memory() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let manager = MemoryTierManager::new(pool);

        let session_id = manager.get_or_create_session(user_id).await.unwrap();

        // Add 7 turns (should only load last 5)
        for i in 1..=7 {
            manager
                .save_conversation_turn(
                    session_id,
                    user_id,
                    &format!("User message {}", i),
                    &format!("AI response {}", i),
                    None,
                    10,
                    15,
                )
                .await
                .unwrap();
        }

        let hot_memory = manager.load_hot_memory(session_id).await.unwrap();

        // Should have exactly 5 turns (hot memory limit)
        assert_eq!(hot_memory.turns.len(), 5);

        // Should have turns 3-7 (newest 5)
        assert_eq!(hot_memory.turns[0].turn_number, 3);
        assert_eq!(hot_memory.turns[4].turn_number, 7);

        // Verify formatted output
        let formatted = hot_memory.format_for_llm();
        assert!(formatted.contains("User message 3"));
        assert!(formatted.contains("AI response 7"));
    }

    #[tokio::test]
    async fn test_load_warm_memory() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let manager = MemoryTierManager::new(pool);

        let session_id = manager.get_or_create_session(user_id).await.unwrap();

        // Add 20 turns (last 5 go to hot, turns 1-15 go to warm)
        for i in 1..=20 {
            manager
                .save_conversation_turn(
                    session_id,
                    user_id,
                    &format!("User message {}", i),
                    &format!("AI response {}", i),
                    None,
                    10,
                    15,
                )
                .await
                .unwrap();
        }

        let warm_memory = manager.load_warm_memory(session_id).await.unwrap();

        // Warm memory should have turns 1-15 (excluding the 5 most recent which are in hot)
        assert_eq!(warm_memory.turns.len(), 15);
        assert_eq!(warm_memory.turns[0].turn_number, 1); // Oldest first
        assert_eq!(warm_memory.turns[14].turn_number, 15); // Newest in warm

        // Verify formatted output
        let formatted = warm_memory.format_for_llm();
        assert!(formatted.contains("Earlier in this session"));
        assert!(formatted.contains("[Turn 1]"));
        assert!(formatted.contains("User message 1"));
        assert!(formatted.contains("[Turn 15]"));
    }

    #[tokio::test]
    async fn test_warm_memory_capacity_limit() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let manager = MemoryTierManager::new(pool);

        let session_id = manager.get_or_create_session(user_id).await.unwrap();

        // Add 100 turns (way more than warm memory can hold)
        for i in 1..=100 {
            manager
                .save_conversation_turn(
                    session_id,
                    user_id,
                    &format!("User message {}", i),
                    &format!("AI response {}", i),
                    None,
                    10,
                    15,
                )
                .await
                .unwrap();
        }

        let warm_memory = manager.load_warm_memory(session_id).await.unwrap();

        // Warm memory should cap at 50 turns (excluding the 5 most recent in hot)
        assert!(warm_memory.turns.len() <= 50);

        // Should have turns 46-95 (50 turns, excluding the 5 most recent)
        assert_eq!(warm_memory.turns[0].turn_number, 46);
        assert_eq!(warm_memory.turns.last().unwrap().turn_number, 95);
    }

    #[tokio::test]
    async fn test_search_warm_memory_keyword() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let manager = MemoryTierManager::new(pool);

        let session_id = manager.get_or_create_session(user_id).await.unwrap();

        // Add diverse conversation turns
        manager
            .save_conversation_turn(
                session_id,
                user_id,
                "Tell me about quantum computing",
                "Quantum computing uses qubits and superposition...",
                None,
                10,
                50,
            )
            .await
            .unwrap();

        manager
            .save_conversation_turn(
                session_id,
                user_id,
                "What's the weather today?",
                "The weather is sunny and warm.",
                None,
                8,
                10,
            )
            .await
            .unwrap();

        manager
            .save_conversation_turn(
                session_id,
                user_id,
                "More about quantum algorithms",
                "Quantum algorithms like Shor's and Grover's...",
                None,
                8,
                40,
            )
            .await
            .unwrap();

        // Search for "quantum" keyword
        let results = manager
            .search_warm_memory(session_id, "quantum", 10)
            .await
            .unwrap();

        // Should find 2 turns matching "quantum"
        assert_eq!(results.len(), 2);
        assert!(
            results[0].user_message.to_lowercase().contains("quantum")
                || results[0].ai_response.to_lowercase().contains("quantum")
        );
        assert!(
            results[1].user_message.to_lowercase().contains("quantum")
                || results[1].ai_response.to_lowercase().contains("quantum")
        );

        // Search for "weather"
        let weather_results = manager
            .search_warm_memory(session_id, "weather", 10)
            .await
            .unwrap();

        assert_eq!(weather_results.len(), 1);
        assert!(weather_results[0].user_message.contains("weather"));
    }

    #[tokio::test]
    async fn test_warm_memory_format() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let manager = MemoryTierManager::new(pool);

        let session_id = manager.get_or_create_session(user_id).await.unwrap();

        // Add a few turns
        for i in 1..=10 {
            manager
                .save_conversation_turn(
                    session_id,
                    user_id,
                    &format!("Question {}", i),
                    &format!("Answer {}", i),
                    None,
                    5,
                    10,
                )
                .await
                .unwrap();
        }

        let warm_memory = manager.load_warm_memory(session_id).await.unwrap();
        let formatted = warm_memory.format_for_llm();

        // Verify structure includes turn numbers and labels
        assert!(formatted.contains("# Earlier in this session"));
        assert!(formatted.contains("[Turn 1] User: Question 1"));
        assert!(formatted.contains("[Turn 1] Assistant: Answer 1"));
        assert!(formatted.contains("[Turn 5] User: Question 5"));
    }

    #[test]
    fn test_hot_memory_token_eviction() {
        let mut hot_memory = HotMemory::new(Uuid::new_v4());

        // Add turns that exceed token limit
        for i in 1..=10 {
            let turn = ConversationTurn {
                id: Uuid::new_v4(),
                session_id: hot_memory.session_id,
                user_id: Uuid::new_v4(),
                turn_number: i,
                user_message: format!("Message {}", i),
                ai_response: format!("Response {}", i),
                user_timestamp: "2025-11-01T00:00:00Z".to_string(),
                ai_timestamp: Some("2025-11-01T00:00:01Z".to_string()),
                snapshot_id: None,
                input_tokens: 200, // Each turn = 400 tokens
                output_tokens: 200,
            };
            hot_memory.add_turn(turn);
        }

        // Should have evicted old turns to stay within limits
        assert!(hot_memory.turns.len() <= HotMemory::MAX_TURNS);
        assert!(hot_memory.total_tokens <= HotMemory::MAX_TOKENS);

        // Should have newest turns
        assert!(hot_memory.turns.back().unwrap().turn_number >= 6);
    }

    #[tokio::test]
    async fn test_end_session() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let manager = MemoryTierManager::new(pool.clone());

        let session_id = manager.get_or_create_session(user_id).await.unwrap();
        manager.end_session(session_id).await.unwrap();

        // Verify ended_at is set
        let ended_at: Option<String> =
            sqlx::query_scalar("SELECT ended_at FROM conversation_sessions WHERE id = ?1")
                .bind(session_id)
                .fetch_one(&pool)
                .await
                .unwrap();

        assert!(ended_at.is_some());

        // get_or_create_session should create NEW session after ending
        let new_session_id = manager.get_or_create_session(user_id).await.unwrap();
        assert_ne!(session_id, new_session_id);
    }

    #[tokio::test]
    async fn test_transition_warm_to_cold() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let manager = MemoryTierManager::new(pool.clone());

        let session_id = manager.get_or_create_session(user_id).await.unwrap();

        // Add 10 turns
        for i in 1..=10 {
            manager
                .save_conversation_turn(
                    session_id,
                    user_id,
                    &format!("Message {}", i),
                    &format!("Response {}", i),
                    None,
                    10,
                    15,
                )
                .await
                .unwrap();
        }

        // End session and transition to cold
        manager.end_session(session_id).await.unwrap();
        let transitioned_count = manager.transition_warm_to_cold(session_id).await.unwrap();

        // Should have transitioned all 10 turns
        assert_eq!(transitioned_count, 10);

        // Verify all turns are now in cold tier
        let cold_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM conversation_turns WHERE session_id = ?1 AND memory_tier = 'cold'",
        )
        .bind(session_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(cold_count, 10);
    }

    #[tokio::test]
    async fn test_load_cold_memory_cross_session() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let manager = MemoryTierManager::new(pool);

        // Create 3 sessions with turns
        for session_num in 1..=3 {
            let session_id = manager.get_or_create_session(user_id).await.unwrap();

            // Add 5 turns per session
            for turn_num in 1..=5 {
                manager
                    .save_conversation_turn(
                        session_id,
                        user_id,
                        &format!("Session {} Turn {}", session_num, turn_num),
                        &format!("Response {}-{}", session_num, turn_num),
                        None,
                        10,
                        15,
                    )
                    .await
                    .unwrap();
            }

            // End session and transition to cold
            manager.end_session(session_id).await.unwrap();
            manager.transition_warm_to_cold(session_id).await.unwrap();
        }

        // Load cold memory (should get turns from all 3 sessions)
        let cold_memory = manager.load_cold_memory(user_id).await.unwrap();

        // Should have 15 total turns (3 sessions × 5 turns)
        assert_eq!(cold_memory.total_turns, 15);
        assert_eq!(cold_memory.turns.len(), 15);

        // Should have counted 3 sessions
        assert_eq!(cold_memory.total_sessions, 3);

        // Verify we have turns from all 3 sessions (not checking specific order due to timestamp precision)
        let session_1_count = cold_memory
            .turns
            .iter()
            .filter(|t| t.user_message.contains("Session 1"))
            .count();
        let session_2_count = cold_memory
            .turns
            .iter()
            .filter(|t| t.user_message.contains("Session 2"))
            .count();
        let session_3_count = cold_memory
            .turns
            .iter()
            .filter(|t| t.user_message.contains("Session 3"))
            .count();

        assert_eq!(session_1_count, 5);
        assert_eq!(session_2_count, 5);
        assert_eq!(session_3_count, 5);
    }

    #[tokio::test]
    async fn test_search_cold_memory_cross_session() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let manager = MemoryTierManager::new(pool);

        // Create 2 sessions
        for _session_num in 1..=2 {
            let session_id = manager.get_or_create_session(user_id).await.unwrap();

            manager
                .save_conversation_turn(
                    session_id,
                    user_id,
                    "Tell me about quantum computing",
                    "Quantum computing uses superposition...",
                    None,
                    10,
                    50,
                )
                .await
                .unwrap();

            manager
                .save_conversation_turn(
                    session_id,
                    user_id,
                    "What about classical computers?",
                    "Classical computers use binary logic...",
                    None,
                    8,
                    40,
                )
                .await
                .unwrap();

            manager.end_session(session_id).await.unwrap();
            manager.transition_warm_to_cold(session_id).await.unwrap();
        }

        // Search for "quantum" across all sessions
        let results = manager
            .search_cold_memory(user_id, "quantum", 10)
            .await
            .unwrap();

        // Should find 2 turns (one from each session)
        assert_eq!(results.len(), 2);
        assert!(
            results[0].user_message.contains("quantum")
                || results[0].ai_response.contains("quantum")
        );
    }

    #[tokio::test]
    async fn test_update_memory_tier() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let manager = MemoryTierManager::new(pool.clone());

        let session_id = manager.get_or_create_session(user_id).await.unwrap();

        let turn_id = manager
            .save_conversation_turn(
                session_id,
                user_id,
                "Test message",
                "Test response",
                None,
                10,
                15,
            )
            .await
            .unwrap();

        // Initially in hot tier
        let tier: String =
            sqlx::query_scalar("SELECT memory_tier FROM conversation_turns WHERE id = ?1")
                .bind(turn_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(tier, "hot");

        // Update to warm
        manager.update_memory_tier(turn_id, "warm").await.unwrap();

        let tier: String =
            sqlx::query_scalar("SELECT memory_tier FROM conversation_turns WHERE id = ?1")
                .bind(turn_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(tier, "warm");

        // Verify transition was recorded
        let transition_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM memory_tier_transitions WHERE turn_id = ?1")
                .bind(turn_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(transition_count, 1);
    }

    #[tokio::test]
    async fn test_cold_memory_format() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let manager = MemoryTierManager::new(pool);

        let session_id = manager.get_or_create_session(user_id).await.unwrap();

        // Add a few turns
        for i in 1..=3 {
            manager
                .save_conversation_turn(
                    session_id,
                    user_id,
                    &format!("Question {}", i),
                    &format!("Answer {}", i),
                    None,
                    5,
                    10,
                )
                .await
                .unwrap();
        }

        manager.end_session(session_id).await.unwrap();
        manager.transition_warm_to_cold(session_id).await.unwrap();

        let cold_memory = manager.load_cold_memory(user_id).await.unwrap();
        let formatted = cold_memory.format_for_llm();

        // Verify structure includes dates and turn numbers
        assert!(formatted.contains("# Past Conversations"));
        assert!(formatted.contains("Turn 1"));
        assert!(formatted.contains("Question 1"));
        assert!(formatted.contains("Answer 3"));
    }
}
