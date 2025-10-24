// Token Optimization Implementation

use crate::memory::CompactStateSnapshot;

pub struct TokenOptimizer {
    token_budget: usize,
}

impl TokenOptimizer {
    pub fn new(token_budget: usize) -> Self {
        Self { token_budget }
    }

    pub fn optimize(&self, compact_state_snapshot: &CompactStateSnapshot) -> String {
        let mut context = String::new();
        let mut used_tokens = 0;

        // Add minimal context
        context.push_str(&self.add_minimal_context(compact_state_snapshot));
        used_tokens += self.count_tokens(&context);

        if used_tokens < self.token_budget {
            let identity =
                self.add_identity_context(compact_state_snapshot, self.token_budget - used_tokens);
            context.push_str(&identity);
            used_tokens += self.count_tokens(&identity);

            if used_tokens < self.token_budget {
                let interfaces = self
                    .add_interface_context(compact_state_snapshot, self.token_budget - used_tokens);
                context.push_str(&interfaces);
                // Continue adding more context as budget allows...
            }
        }

        context
    }

    fn add_minimal_context(&self, compact_state_snapshot: &CompactStateSnapshot) -> String {
        // Implement minimal context creation
        format!(
            "<state_snapshot id='{}' timestamp='{}'/>",
            compact_state_snapshot.id(),
            compact_state_snapshot.timestamp()
        )
    }

    fn add_identity_context(
        &self,
        compact_state_snapshot: &CompactStateSnapshot,
        _remaining_budget: usize,
    ) -> String {
        // Implement identity context creation within budget
        format!(
            "<identity anchors='{}'/>",
            compact_state_snapshot.identity_anchor_ids().join(",")
        )
    }

    fn add_interface_context(
        &self,
        compact_state_snapshot: &CompactStateSnapshot,
        _remaining_budget: usize,
    ) -> String {
        // Implement interface context creation within budget
        let mut interfaces = String::new();
        for interface in compact_state_snapshot.interface_states() {
            interfaces.push_str(&format!(
                "<interface domains='{}' permeability='{}'/>",
                interface.domains().0,
                interface.permeability()
            ));
        }
        interfaces
    }

    fn count_tokens(&self, text: &str) -> usize {
        // Simple token counting implementation
        text.split_whitespace().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt_engine::{BoundaryState, DomainState};

    #[tokio::test]
    async fn test_token_optimizer() {
        let token_optimizer = TokenOptimizer::new(1024);

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let memory_manager = crate::memory::MemoryManager::new(&database_url)
            .await
            .unwrap();

        // Create a test user first (required by foreign key constraint)
        let user_id = uuid::Uuid::new_v4();
        sqlx::query(
            "INSERT INTO users (id, provider, provider_id, email, name, created_at, last_login)
             VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        )
        .bind(user_id.as_bytes().to_vec())
        .bind("test")
        .bind(user_id.to_string())
        .bind("test@example.com")
        .bind("Test User")
        .execute(&memory_manager.db_pool)
        .await
        .unwrap();

        let domains = vec![
            DomainState {
                name: "CD".to_string(),
                state: "0.8,0.9,0.7,0.6,0.5".to_string(),
            },
            DomainState {
                name: "SD".to_string(),
                state: "0.7,0.6,0.5,0.4,0.3".to_string(),
            },
        ];

        let boundaries = vec![
            BoundaryState {
                name: "CD-SD".to_string(),
                permeability: 0.8,
                status: "Active".to_string(),
            },
            BoundaryState {
                name: "SD-CuD".to_string(),
                permeability: 0.5,
                status: "Active".to_string(),
            },
        ];

        let patterns = vec!["Pattern 1".to_string(), "Pattern 2".to_string()];

        let user_input = "Sample query for token optimization testing";
        memory_manager
            .create_snapshot(domains, boundaries, patterns, user_id, user_input)
            .await
            .unwrap();
        let compact_state_snapshot = memory_manager
            .get_latest_snapshot(user_id)
            .await
            .unwrap()
            .unwrap();

        let context = token_optimizer.optimize(&compact_state_snapshot);
        assert!(!context.is_empty());
    }
}
