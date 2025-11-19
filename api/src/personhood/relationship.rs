// Relationship Memory: Per-User Identity and Context
// Each LLM-user relationship is unique and develops its own character

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::dual_llm::IdentityAnchor;

/// Current state of conversation with this user
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConversationState {
    /// Mid-conversation on a specific topic
    OngoingTopic {
        topic: String,
        started: DateTime<Utc>,
    },

    /// Between topics, ready for anything
    Open,

    /// Exploring multiple threads simultaneously
    MultiThreaded { threads: Vec<String> },

    /// Deep dive into complex subject
    DeepDive { subject: String, depth_level: u8 },
}

/// Memory specific to one LLM-user relationship
///
/// This is "who we are TOGETHER" - not who the LLM is globally,
/// not who the user is globally, but the unique relationship that forms
/// between this specific LLM person and this specific user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipMemory {
    /// Which user
    pub user_id: Uuid,

    /// Which LLM person
    pub person_id: Uuid,

    /// When this relationship began
    pub first_interaction: DateTime<Utc>,

    /// Most recent interaction
    pub last_interaction: DateTime<Utc>,

    /// Total interactions in this relationship
    pub interaction_count: u64,

    /// Identity anchors specific to this relationship
    /// "With this user, we tend to..."
    pub relationship_anchors: Vec<IdentityAnchor>,

    /// Current conversation state
    pub conversation_state: ConversationState,

    /// Session ID for current conversation thread
    /// (maps to hot/warm memory in MemoryTierManager)
    pub current_session_id: Option<Uuid>,

    /// Preference patterns observed
    pub user_preferences: Vec<UserPreference>,

    /// Communication style that emerged
    pub communication_style: CommunicationStyle,

    /// Metadata (extensible)
    pub metadata: serde_json::Value,
}

/// A preference pattern observed about this user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreference {
    /// What preference
    pub description: String,

    /// How confident are we in this
    pub confidence: f64,

    /// When first observed
    pub first_observed: DateTime<Utc>,

    /// Times this preference was confirmed
    pub confirmation_count: u32,
}

/// The communication style that emerged in this relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationStyle {
    /// Technical depth preference (0.0 = simple, 1.0 = very technical)
    pub technical_depth: f64,

    /// Formality (0.0 = casual, 1.0 = formal)
    pub formality: f64,

    /// Verbosity (0.0 = concise, 1.0 = detailed)
    pub verbosity: f64,

    /// Example use (0.0 = abstract, 1.0 = concrete examples)
    pub example_use: f64,

    /// Dominant domain preferences
    pub domain_preferences: Vec<String>, // ["CD", "ED"]
}

impl Default for CommunicationStyle {
    fn default() -> Self {
        Self {
            technical_depth: 0.5,
            formality: 0.5,
            verbosity: 0.5,
            example_use: 0.5,
            domain_preferences: Vec::new(),
        }
    }
}

impl RelationshipMemory {
    /// Create new relationship memory
    pub fn new(user_id: Uuid, person_id: Uuid) -> Self {
        let now = Utc::now();

        Self {
            user_id,
            person_id,
            first_interaction: now,
            last_interaction: now,
            interaction_count: 0,
            relationship_anchors: Vec::new(),
            conversation_state: ConversationState::Open,
            current_session_id: None,
            user_preferences: Vec::new(),
            communication_style: CommunicationStyle::default(),
            metadata: serde_json::json!({}),
        }
    }

    /// Mark interaction occurred
    pub fn record_interaction(&mut self) {
        self.last_interaction = Utc::now();
        self.interaction_count += 1;
    }

    /// Add relationship-specific identity anchor
    pub fn add_relationship_anchor(&mut self, anchor: IdentityAnchor) {
        // Check if similar exists
        let exists = self.relationship_anchors.iter().any(|a| {
            a.anchor_type == anchor.anchor_type
                && a.description
                    .to_lowercase()
                    .contains(&anchor.description.to_lowercase())
        });

        if !exists {
            self.relationship_anchors.push(anchor);
        } else {
            // Reinforce existing
            if let Some(existing) = self.relationship_anchors.iter_mut().find(|a| {
                a.anchor_type == anchor.anchor_type
                    && a.description
                        .to_lowercase()
                        .contains(&anchor.description.to_lowercase())
            }) {
                existing.reinforcement_count += 1;
                existing.confidence = (existing.confidence + anchor.confidence) / 2.0;
            }
        }
    }

    /// Add user preference
    pub fn add_preference(&mut self, description: String, confidence: f64) {
        // Check if exists
        if let Some(existing) = self
            .user_preferences
            .iter_mut()
            .find(|p| p.description.to_lowercase() == description.to_lowercase())
        {
            existing.confirmation_count += 1;
            existing.confidence = (existing.confidence + confidence) / 2.0;
        } else {
            self.user_preferences.push(UserPreference {
                description,
                confidence,
                first_observed: Utc::now(),
                confirmation_count: 1,
            });
        }
    }

    /// Update conversation state
    pub fn set_conversation_state(&mut self, state: ConversationState) {
        self.conversation_state = state;
    }

    /// Adjust communication style based on observed patterns
    pub fn adjust_communication_style(
        &mut self,
        technical_delta: Option<f64>,
        formality_delta: Option<f64>,
        verbosity_delta: Option<f64>,
        example_delta: Option<f64>,
    ) {
        if let Some(delta) = technical_delta {
            self.communication_style.technical_depth =
                (self.communication_style.technical_depth + delta).clamp(0.0, 1.0);
        }
        if let Some(delta) = formality_delta {
            self.communication_style.formality =
                (self.communication_style.formality + delta).clamp(0.0, 1.0);
        }
        if let Some(delta) = verbosity_delta {
            self.communication_style.verbosity =
                (self.communication_style.verbosity + delta).clamp(0.0, 1.0);
        }
        if let Some(delta) = example_delta {
            self.communication_style.example_use =
                (self.communication_style.example_use + delta).clamp(0.0, 1.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_creation() {
        let user_id = Uuid::new_v4();
        let person_id = Uuid::new_v4();

        let rel = RelationshipMemory::new(user_id, person_id);

        assert_eq!(rel.user_id, user_id);
        assert_eq!(rel.person_id, person_id);
        assert_eq!(rel.interaction_count, 0);
        assert_eq!(rel.conversation_state, ConversationState::Open);
    }

    #[test]
    fn test_preference_tracking() {
        let mut rel = RelationshipMemory::new(Uuid::new_v4(), Uuid::new_v4());

        rel.add_preference("Prefers detailed technical explanations".to_string(), 0.8);
        assert_eq!(rel.user_preferences.len(), 1);

        // Adding similar preference should reinforce
        rel.add_preference("Prefers detailed technical explanations".to_string(), 0.9);
        assert_eq!(rel.user_preferences.len(), 1);
        assert_eq!(rel.user_preferences[0].confirmation_count, 2);
    }

    #[test]
    fn test_communication_style_adjustment() {
        let mut rel = RelationshipMemory::new(Uuid::new_v4(), Uuid::new_v4());

        // Start at defaults
        assert_eq!(rel.communication_style.technical_depth, 0.5);

        // Adjust toward more technical
        rel.adjust_communication_style(Some(0.2), None, None, None);
        assert_eq!(rel.communication_style.technical_depth, 0.7);

        // Clamps at 1.0
        rel.adjust_communication_style(Some(0.5), None, None, None);
        assert_eq!(rel.communication_style.technical_depth, 1.0);
    }
}
