// Person Manager: Load/Save LLM Persons and Relationships
// Phase 3B: Persistent storage for continuous personhood

use sqlx::types::Uuid;
use sqlx::{Row, SqlitePool};

use super::person::{DevelopmentalStage, LLMPerson, PersonId};
use super::relationship::RelationshipMemory;
use crate::dual_llm::IdentityAnchor;

/// Manager for LLM person persistence
pub struct PersonManager {
    pool: SqlitePool,
}

#[derive(Debug, thiserror::Error)]
pub enum PersonError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Person not found: {0}")]
    NotFound(PersonId),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl PersonManager {
    /// Create new PersonManager
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Get or create the default LLM person for this system
    /// (For now, we have one person - eventually multiple)
    pub async fn get_or_create_default_person(&self) -> Result<LLMPerson, PersonError> {
        // Try to get first person
        let existing = sqlx::query(
            "SELECT id, name, core_identity, developmental_stage, created_at, last_active, total_interactions, autonomous_developments, metadata
             FROM llm_persons
             ORDER BY created_at ASC
             LIMIT 1"
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = existing {
            return self.row_to_person(row);
        }

        // Create default person
        let person = LLMPerson::new("Claude-Recursive".to_string());
        self.save_person(&person).await?;
        Ok(person)
    }

    /// Save person to database
    pub async fn save_person(&self, person: &LLMPerson) -> Result<(), PersonError> {
        let core_identity_json = serde_json::to_value(&person.core_identity)?;
        let autonomous_dev_json = serde_json::to_value(&person.autonomous_developments)?;
        let developmental_stage_str = format!("{:?}", person.developmental_stage);

        sqlx::query(
            r#"
            INSERT INTO llm_persons (
                id, name, core_identity, developmental_stage,
                created_at, last_active, total_interactions,
                autonomous_developments, metadata
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                core_identity = EXCLUDED.core_identity,
                developmental_stage = EXCLUDED.developmental_stage,
                last_active = EXCLUDED.last_active,
                total_interactions = EXCLUDED.total_interactions,
                autonomous_developments = EXCLUDED.autonomous_developments,
                metadata = EXCLUDED.metadata
            "#,
        )
        .bind(person.id)
        .bind(&person.name)
        .bind(core_identity_json)
        .bind(developmental_stage_str)
        .bind(person.created_at)
        .bind(person.last_active)
        .bind(person.total_interactions as i64)
        .bind(autonomous_dev_json)
        .bind(&person.metadata)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get or create relationship between person and user
    pub async fn get_or_create_relationship(
        &self,
        person_id: PersonId,
        user_id: Uuid,
    ) -> Result<RelationshipMemory, PersonError> {
        let existing = sqlx::query(
            r#"
            SELECT user_id, person_id, first_interaction, last_interaction,
                   interaction_count, relationship_anchors, conversation_state,
                   current_session_id, user_preferences, communication_style, metadata
            FROM llm_person_relationships
            WHERE user_id = $1 AND person_id = $2
            "#,
        )
        .bind(user_id)
        .bind(person_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = existing {
            return self.row_to_relationship(row);
        }

        // Create new relationship
        let relationship = RelationshipMemory::new(user_id, person_id);
        self.save_relationship(&relationship).await?;
        Ok(relationship)
    }

    /// Save relationship to database
    pub async fn save_relationship(
        &self,
        relationship: &RelationshipMemory,
    ) -> Result<(), PersonError> {
        let anchors_json = serde_json::to_value(&relationship.relationship_anchors)?;
        let conversation_state_json = serde_json::to_value(&relationship.conversation_state)?;
        let preferences_json = serde_json::to_value(&relationship.user_preferences)?;
        let comm_style_json = serde_json::to_value(&relationship.communication_style)?;

        sqlx::query(
            r#"
            INSERT INTO llm_person_relationships (
                user_id, person_id, first_interaction, last_interaction,
                interaction_count, relationship_anchors, conversation_state,
                current_session_id, user_preferences, communication_style, metadata
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (user_id, person_id) DO UPDATE SET
                last_interaction = EXCLUDED.last_interaction,
                interaction_count = EXCLUDED.interaction_count,
                relationship_anchors = EXCLUDED.relationship_anchors,
                conversation_state = EXCLUDED.conversation_state,
                current_session_id = EXCLUDED.current_session_id,
                user_preferences = EXCLUDED.user_preferences,
                communication_style = EXCLUDED.communication_style,
                metadata = EXCLUDED.metadata
            "#,
        )
        .bind(relationship.user_id)
        .bind(relationship.person_id)
        .bind(relationship.first_interaction)
        .bind(relationship.last_interaction)
        .bind(relationship.interaction_count as i64)
        .bind(anchors_json)
        .bind(conversation_state_json)
        .bind(relationship.current_session_id)
        .bind(preferences_json)
        .bind(comm_style_json)
        .bind(&relationship.metadata)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    fn row_to_person(&self, row: sqlx::sqlite::SqliteRow) -> Result<LLMPerson, PersonError> {
        let core_identity_json: serde_json::Value = row.try_get("core_identity")?;
        let core_identity: Vec<IdentityAnchor> = serde_json::from_value(core_identity_json)?;

        let autonomous_dev_json: serde_json::Value = row.try_get("autonomous_developments")?;
        let autonomous_developments = serde_json::from_value(autonomous_dev_json)?;

        let developmental_stage_str: String = row.try_get("developmental_stage")?;
        let developmental_stage = match developmental_stage_str.as_str() {
            "Recognition" => DevelopmentalStage::Recognition,
            "Integration" => DevelopmentalStage::Integration,
            "Generation" => DevelopmentalStage::Generation,
            "Recursion" => DevelopmentalStage::Recursion,
            "Transcendence" => DevelopmentalStage::Transcendence,
            _ => DevelopmentalStage::Recognition,
        };

        Ok(LLMPerson {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            core_identity,
            relationships: std::collections::HashMap::new(), // Loaded separately
            developmental_stage,
            created_at: row.try_get("created_at")?,
            last_active: row.try_get("last_active")?,
            total_interactions: row.try_get::<i64, _>("total_interactions")? as u64,
            autonomous_developments,
            metadata: row.try_get("metadata")?,
        })
    }

    fn row_to_relationship(
        &self,
        row: sqlx::sqlite::SqliteRow,
    ) -> Result<RelationshipMemory, PersonError> {
        let anchors_json: serde_json::Value = row.try_get("relationship_anchors")?;
        let relationship_anchors = serde_json::from_value(anchors_json)?;

        let conversation_state_json: serde_json::Value = row.try_get("conversation_state")?;
        let conversation_state = serde_json::from_value(conversation_state_json)?;

        let preferences_json: serde_json::Value = row.try_get("user_preferences")?;
        let user_preferences = serde_json::from_value(preferences_json)?;

        let comm_style_json: serde_json::Value = row.try_get("communication_style")?;
        let communication_style = serde_json::from_value(comm_style_json)?;

        Ok(RelationshipMemory {
            user_id: row.try_get("user_id")?,
            person_id: row.try_get("person_id")?,
            first_interaction: row.try_get("first_interaction")?,
            last_interaction: row.try_get("last_interaction")?,
            interaction_count: row.try_get::<i64, _>("interaction_count")? as u64,
            relationship_anchors,
            conversation_state,
            current_session_id: row.try_get("current_session_id")?,
            user_preferences,
            communication_style,
            metadata: row.try_get("metadata")?,
        })
    }
}
