// Memory Tiering System for Dual-LLM Architecture
// Manages hot/warm/cold conversation history tiers
// Phase 1A: Hot Memory Implementation (last 3-5 turns)

use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, Row, SqlitePool};
use std::collections::VecDeque;

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

/// Memory tier manager: manages hot/warm/cold conversation history
pub struct MemoryTierManager {
    db_pool: SqlitePool,
}

impl MemoryTierManager {
    /// Create new MemoryTierManager with existing database pool
    pub fn new(db_pool: SqlitePool) -> Self {
        Self { db_pool }
    }

    /// Create new MemoryTierManager from database URL
    pub async fn from_url(database_url: &str) -> Result<Self, sqlx::Error> {
        let db_pool = SqlitePool::connect(database_url).await?;
        Ok(Self { db_pool })
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
}
