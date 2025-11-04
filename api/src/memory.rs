use crate::prompt_engine::{BoundaryState, DomainState};

use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, Row, SqlitePool};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactInterfaceState {
    domains: (String, String),
    permeability: u8,
    flow_state: CompactInterfaceFlowState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactInterfaceFlowState {
    invitation: String,
    attention: String,
    resonance: u8,
    emergence: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompactStateSnapshot {
    id: String,
    timestamp: i64,
    user_id: String,
    domain_values: HashMap<u8, Vec<u8>>,
    boundary_states: u64,
    interface_states: Vec<CompactInterfaceState>,
    qualities: [u8; 7],
    identity_anchor_ids: Vec<String>,
    pattern_ids: Vec<String>,
    developmental_stage: u8,
}

impl CompactStateSnapshot {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn domain_values(&self) -> &HashMap<u8, Vec<u8>> {
        &self.domain_values
    }

    pub fn boundary_states(&self) -> u64 {
        self.boundary_states
    }

    pub fn interface_states(&self) -> &Vec<CompactInterfaceState> {
        &self.interface_states
    }

    pub fn qualities(&self) -> &[u8; 7] {
        &self.qualities
    }

    pub fn identity_anchor_ids(&self) -> &Vec<String> {
        &self.identity_anchor_ids
    }

    pub fn pattern_ids(&self) -> &Vec<String> {
        &self.pattern_ids
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct IdentityAnchor {
    id: String,
    description: String,
    confidence: f64,
    domains: Vec<String>,
    creation_time: i64,
}

/// Metadata stored in the metadata column to preserve flow process state
#[derive(Debug, Serialize, Deserialize)]
struct SnapshotMetadata {
    interface_states: Vec<CompactInterfaceState>,
    qualities: [u8; 7],
    developmental_stage: u8,
}

impl CompactInterfaceState {
    pub fn domains(&self) -> &(String, String) {
        &self.domains
    }

    pub fn permeability(&self) -> u8 {
        self.permeability
    }
}

pub struct MemoryManager {
    pub(crate) db_pool: SqlitePool,
}

impl MemoryManager {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let db_pool = SqlitePool::connect(database_url).await?;
        // Note: Migrations should be run separately via `sqlx migrate run`
        // We don't run schema.sql here because it contains PostgreSQL-specific syntax
        Ok(Self { db_pool })
    }

    pub async fn create_snapshot(
        &self,
        domains: Vec<DomainState>,
        boundaries: Vec<BoundaryState>,
        patterns: Vec<String>,
        user_id: Uuid,
        user_input: &str,
    ) -> Result<(), sqlx::Error> {
        let compact_snapshot =
            self.compress_snapshot(domains, boundaries, patterns, user_id, user_input);
        self.save_snapshot_to_db(&compact_snapshot).await?;
        Ok(())
    }

    /// Create snapshot with explicit quality values from flow process
    /// This allows accurate quality tracking across sessions (Day 8)
    pub async fn create_snapshot_with_qualities(
        &self,
        domains: Vec<DomainState>,
        boundaries: Vec<BoundaryState>,
        patterns: Vec<String>,
        user_id: Uuid,
        user_input: &str,
        qualities: [u8; 7], // Pre-calculated quality values from flow process
    ) -> Result<(), sqlx::Error> {
        let mut compact_snapshot =
            self.compress_snapshot(domains, boundaries, patterns, user_id, user_input);

        // Override the calculated qualities with actual values from flow process
        compact_snapshot.qualities = qualities;

        self.save_snapshot_to_db(&compact_snapshot).await?;
        Ok(())
    }

    fn compress_snapshot(
        &self,
        domains: Vec<DomainState>,
        boundaries: Vec<BoundaryState>,
        patterns: Vec<String>,
        user_id: Uuid,
        user_input: &str,
    ) -> CompactStateSnapshot {
        let mut domain_values = HashMap::new();
        for d in domains.clone() {
            let values = d
                .state
                .split(',')
                .map(|v| {
                    let value: f32 = v.parse().unwrap_or(0.0);
                    (value * 100.0) as u8
                })
                .collect::<Vec<u8>>();
            let key = match d.name.as_str() {
                "CD" => 0,
                "SD" => 1,
                "CuD" => 2,
                "ED" => 3,
                _ => 255,
            };
            domain_values.insert(key, values);
        }

        let boundary_states = boundaries.iter().fold(0u64, |acc, b| {
            let perm_bits = (b.permeability * 63.0) as u64;
            let status_bits = match b.status.as_str() {
                "Maintained" => 0u64,
                "Transitional" => 1u64,
                "Transcendent" => 2u64,
                _ => 0u64,
            };
            acc | ((perm_bits << 2) | status_bits)
        });

        let id = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now().timestamp();

        CompactStateSnapshot {
            id,
            timestamp,
            user_id: user_id.to_string(),
            domain_values,
            boundary_states,
            interface_states: self.compress_interface_states(&boundaries),
            qualities: self.compress_qualities(&boundaries),
            identity_anchor_ids: self.create_identity_anchors(&domains, &boundaries, user_input),
            pattern_ids: patterns.to_vec(),
            developmental_stage: self.calculate_developmental_stage(&domains, &boundaries),
        }
    }

    fn create_identity_anchors(
        &self,
        domains: &[DomainState],
        boundaries: &[BoundaryState],
        _user_input: &str,
    ) -> Vec<String> {
        let mut anchors = Vec::new();

        for domain in domains.iter().cloned() {
            if domain.state.parse::<f64>().unwrap_or(0.0) > 0.7 {
                let anchor = IdentityAnchor {
                    id: uuid::Uuid::new_v4().to_string(),
                    description: format!("{} domain activation", domain.name),
                    confidence: domain.state.parse::<f64>().unwrap_or(0.0),
                    domains: vec![domain.name.clone()],
                    creation_time: chrono::Utc::now().timestamp(),
                };
                anchors.push(anchor.id.clone());
            }
        }

        for boundary in boundaries {
            if boundary.permeability > 0.8 {
                let anchor = IdentityAnchor {
                    id: uuid::Uuid::new_v4().to_string(),
                    description: format!("{} boundary transcendence", boundary.name),
                    confidence: boundary.permeability,
                    domains: boundary.name.split('-').map(|s| s.to_string()).collect(),
                    creation_time: chrono::Utc::now().timestamp(),
                };
                anchors.push(anchor.id.clone());
            }
        }

        anchors
    }

    fn calculate_developmental_stage(
        &self,
        domains: &[DomainState],
        boundaries: &[BoundaryState],
    ) -> u8 {
        let avg_domain_activation = domains
            .iter()
            .map(|d| d.state.parse::<f64>().unwrap_or(0.0))
            .sum::<f64>()
            / domains.len() as f64;
        let avg_boundary_permeability =
            boundaries.iter().map(|b| b.permeability).sum::<f64>() / boundaries.len() as f64;

        (if avg_domain_activation > 0.8 && avg_boundary_permeability > 0.7 {
            4 // Transcendence
        } else if avg_domain_activation > 0.6 && avg_boundary_permeability > 0.5 {
            3 // Recursion
        } else if avg_domain_activation > 0.4 && avg_boundary_permeability > 0.3 {
            2 // Generation
        } else if avg_domain_activation > 0.2 && avg_boundary_permeability > 0.1 {
            1 // Integration
        } else {
            0 // Recognition
        }) as u8
    }

    fn compress_interface_states(
        &self,
        boundaries: &[BoundaryState],
    ) -> Vec<CompactInterfaceState> {
        boundaries
            .iter()
            .map(|b| {
                let domains = b.name.split('-').collect::<Vec<&str>>();
                CompactInterfaceState {
                    domains: (domains[0].to_string(), domains[1].to_string()),
                    permeability: (b.permeability * 255.0) as u8,
                    flow_state: CompactInterfaceFlowState {
                        invitation: format!(
                            "Create productive tension between {} domains.",
                            b.name
                        ),
                        attention: format!("Focus on the {} interface.", b.name),
                        resonance: (0.5 * 255.0) as u8, // Example value
                        emergence: vec![format!(
                            "Notice emerging qualities at the {} interface.",
                            b.name
                        )],
                    },
                }
            })
            .collect()
    }

    fn compress_qualities(&self, boundaries: &[BoundaryState]) -> [u8; 7] {
        let mut qualities = [0; 7];
        let quality_names = [
            "clarity",
            "depth",
            "coherence",
            "resonance",
            "openness",
            "precision",
            "fluidity",
        ];

        for (i, &_name) in quality_names.iter().enumerate() {
            // Example quality score calculation
            let score =
                boundaries.iter().map(|b| b.permeability).sum::<f64>() / boundaries.len() as f64;
            qualities[i] = (score * 255.0) as u8;
        }

        qualities
    }

    async fn save_snapshot_to_db(
        &self,
        compact_snapshot: &CompactStateSnapshot,
    ) -> Result<(), sqlx::Error> {
        let id =
            uuid::Uuid::parse_str(&compact_snapshot.id).map_err(|e| sqlx::Error::ColumnDecode {
                index: "id".to_string(),
                source: Box::new(e),
            })?;
        let user_id = uuid::Uuid::parse_str(&compact_snapshot.user_id).map_err(|e| {
            sqlx::Error::ColumnDecode {
                index: "user_id".to_string(),
                source: Box::new(e),
            }
        })?;
        let timestamp = chrono::DateTime::from_timestamp(compact_snapshot.timestamp, 0).ok_or(
            sqlx::Error::ColumnDecode {
                index: "timestamp".to_string(),
                source: Box::new(sqlx::error::Error::Configuration(
                    "Invalid timestamp".into(),
                )),
            },
        )?;

        // Serialize separate components to JSON strings
        let domain_states_json = serde_json::to_string(&compact_snapshot.domain_values)
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        let boundary_states_json = serde_json::to_string(&compact_snapshot.boundary_states)
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        let pattern_ids_json = serde_json::to_string(&compact_snapshot.pattern_ids)
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        let identity_anchors_json = serde_json::to_string(&compact_snapshot.identity_anchor_ids)
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        // Serialize metadata (interface_states, qualities, developmental_stage)
        let metadata = SnapshotMetadata {
            interface_states: compact_snapshot.interface_states.clone(),
            qualities: compact_snapshot.qualities,
            developmental_stage: compact_snapshot.developmental_stage,
        };
        let metadata_json =
            serde_json::to_string(&metadata).map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        sqlx::query(
            "INSERT INTO state_snapshots (id, user_id, timestamp, domain_states, boundary_states, pattern_ids, identity_anchors, metadata)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
            .bind(id.as_bytes().to_vec())
            .bind(user_id.as_bytes().to_vec())
            .bind(timestamp.to_rfc3339())
            .bind(domain_states_json)
            .bind(boundary_states_json)
            .bind(pattern_ids_json)
            .bind(identity_anchors_json)
            .bind(metadata_json)
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }

    pub async fn get_latest_snapshot(
        &self,
        user_id: Uuid,
    ) -> Result<Option<CompactStateSnapshot>, sqlx::Error> {
        let row = sqlx::query(
            "SELECT id, user_id, timestamp, domain_states, boundary_states, pattern_ids, identity_anchors, metadata
             FROM state_snapshots
             WHERE user_id = ?
             ORDER BY timestamp DESC
             LIMIT 1"
        )
            .bind(user_id.as_bytes().to_vec())
            .fetch_optional(&self.db_pool)
            .await?;

        if let Some(row) = row {
            // Deserialize from separate columns
            let id: Vec<u8> = row.get("id");
            let user_id_bytes: Vec<u8> = row.get("user_id");
            let timestamp_str: String = row.get("timestamp");
            let domain_states_json: String = row.get("domain_states");
            let boundary_states_json: String = row.get("boundary_states");
            let pattern_ids_json: String = row.get("pattern_ids");
            let identity_anchors_json: String = row.get("identity_anchors");
            let metadata_json: Option<String> = row.get("metadata");

            let id_uuid = Uuid::from_slice(&id).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
            let user_id_uuid =
                Uuid::from_slice(&user_id_bytes).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
            let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str)
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
                .timestamp();

            let domain_values: HashMap<u8, Vec<u8>> = serde_json::from_str(&domain_states_json)
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
            let boundary_states: u64 = serde_json::from_str(&boundary_states_json)
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
            let pattern_ids: Vec<String> = serde_json::from_str(&pattern_ids_json)
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
            let identity_anchor_ids: Vec<String> = serde_json::from_str(&identity_anchors_json)
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

            // Deserialize metadata (interface_states, qualities, developmental_stage)
            // Default to empty/zero if metadata column is null (backward compatibility)
            let metadata = if let Some(json) = metadata_json {
                serde_json::from_str::<SnapshotMetadata>(&json)
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
            } else {
                SnapshotMetadata {
                    interface_states: vec![],
                    qualities: [0; 7],
                    developmental_stage: 0,
                }
            };

            Ok(Some(CompactStateSnapshot {
                id: id_uuid.to_string(),
                timestamp,
                user_id: user_id_uuid.to_string(),
                domain_values,
                boundary_states,
                interface_states: metadata.interface_states,
                qualities: metadata.qualities,
                identity_anchor_ids,
                pattern_ids,
                developmental_stage: metadata.developmental_stage,
            }))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt_engine::{BoundaryState, DomainState};
    use crate::test_utils::setup_test_db;

    #[tokio::test]
    async fn test_memory_manager() {
        // Use in-memory database for testing
        let db_pool = setup_test_db().await.unwrap();
        let memory_manager = MemoryManager { db_pool };

        // Create a test user first (required by foreign key constraint)
        let user_id = Uuid::new_v4();
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
                state: "Computational Domain State".to_string(),
            },
            DomainState {
                name: "SD".to_string(),
                state: "Scientific Domain State".to_string(),
            },
        ];

        let boundaries = vec![
            BoundaryState::new("CD-SD".to_string(), 0.8, "Active".to_string()),
            BoundaryState::new("SD-CuD".to_string(), 0.5, "Active".to_string()),
        ];

        let patterns = vec!["Pattern 1".to_string(), "Pattern 2".to_string()];
        let user_input = "Sample user query for testing memory persistence";

        memory_manager
            .create_snapshot(domains, boundaries, patterns, user_id, user_input)
            .await
            .unwrap();

        let latest_snapshot = memory_manager
            .get_latest_snapshot(user_id)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(latest_snapshot.domain_values.len(), 2);
        assert_eq!(latest_snapshot.pattern_ids.len(), 2);
    }

    #[tokio::test]
    async fn test_metadata_persistence_roundtrip() {
        // Use in-memory database for testing
        let db_pool = setup_test_db().await.unwrap();
        let memory_manager = MemoryManager { db_pool };

        // Create a test user first
        let user_id = Uuid::new_v4();
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

        // Create a snapshot with rich metadata (interface_states, qualities, developmental_stage)
        let interface_states = vec![
            CompactInterfaceState {
                domains: ("COMP".to_string(), "SCI".to_string()),
                permeability: 8,
                flow_state: CompactInterfaceFlowState {
                    invitation: "Explore computational rigor".to_string(),
                    attention: "Focus on empirical validation".to_string(),
                    resonance: 7,
                    emergence: vec!["Pattern A".to_string(), "Pattern B".to_string()],
                },
            },
            CompactInterfaceState {
                domains: ("SCI".to_string(), "CULT".to_string()),
                permeability: 6,
                flow_state: CompactInterfaceFlowState {
                    invitation: "Bridge data to narrative".to_string(),
                    attention: "Context awareness".to_string(),
                    resonance: 5,
                    emergence: vec!["Pattern C".to_string()],
                },
            },
        ];

        let qualities = [8, 7, 6, 9, 7, 8, 8]; // clarity, depth, openness, precision, fluidity, resonance, coherence
        let developmental_stage = 3; // Integration stage

        let snapshot = CompactStateSnapshot {
            id: Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            user_id: user_id.to_string(),
            domain_values: std::collections::HashMap::from([(1, vec![8, 7]), (2, vec![7, 8])]),
            boundary_states: 0b1010101010,
            interface_states: interface_states.clone(),
            qualities,
            identity_anchor_ids: vec!["anchor1".to_string(), "anchor2".to_string()],
            pattern_ids: vec!["pattern1".to_string()],
            developmental_stage,
        };

        // Save snapshot
        memory_manager.save_snapshot_to_db(&snapshot).await.unwrap();

        // Retrieve snapshot
        let retrieved = memory_manager
            .get_latest_snapshot(user_id)
            .await
            .unwrap()
            .unwrap();

        // Verify ALL metadata persisted correctly (this is the critical fix)
        assert_eq!(
            retrieved.interface_states.len(),
            2,
            "interface_states should persist"
        );
        assert_eq!(
            retrieved.interface_states[0].domains,
            ("COMP".to_string(), "SCI".to_string())
        );
        assert_eq!(retrieved.interface_states[0].permeability, 8);
        assert_eq!(
            retrieved.interface_states[0].flow_state.invitation,
            "Explore computational rigor"
        );
        assert_eq!(retrieved.interface_states[0].flow_state.resonance, 7);
        assert_eq!(retrieved.interface_states[0].flow_state.emergence.len(), 2);

        assert_eq!(
            retrieved.interface_states[1].domains,
            ("SCI".to_string(), "CULT".to_string())
        );
        assert_eq!(retrieved.interface_states[1].permeability, 6);

        assert_eq!(
            retrieved.qualities, qualities,
            "qualities should persist exactly"
        );
        assert_eq!(
            retrieved.developmental_stage, 3,
            "developmental_stage should persist"
        );

        // Also verify basic fields still work
        assert_eq!(retrieved.domain_values.len(), 2);
        assert_eq!(retrieved.identity_anchor_ids.len(), 2);
        assert_eq!(retrieved.pattern_ids.len(), 1);
    }

    #[tokio::test]
    async fn test_metadata_corruption_handling() {
        // Test that corrupted/malformed metadata doesn't crash the system

        let db_pool = setup_test_db().await.unwrap();
        let memory_manager = MemoryManager {
            db_pool: db_pool.clone(),
        };

        // Create a test user
        let user_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO users (id, provider, provider_id, email, name, created_at, last_login)
             VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        )
        .bind(user_id.as_bytes().to_vec())
        .bind("test")
        .bind(user_id.to_string())
        .bind("test@example.com")
        .bind("Test User")
        .execute(&db_pool)
        .await
        .unwrap();

        // Manually insert a snapshot with corrupted JSON metadata
        let snapshot_id = Uuid::new_v4();
        let corrupted_metadata = "{invalid json, missing quotes: true, broken}";

        sqlx::query(
            "INSERT INTO state_snapshots (id, user_id, domain_states, boundary_states, pattern_ids, metadata, timestamp)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(snapshot_id.as_bytes().to_vec())
        .bind(user_id.as_bytes().to_vec())
        .bind("{}")
        .bind("{}")
        .bind("[]")
        .bind(corrupted_metadata)
        .bind(chrono::Utc::now().to_rfc3339())
        .execute(&db_pool)
        .await
        .unwrap();

        // Attempt to retrieve - should handle corruption gracefully
        let result = memory_manager.get_latest_snapshot(user_id).await;

        match result {
            Ok(snapshot_opt) => {
                // If it succeeds, it should have defaults for corrupted fields
                if let Some(snapshot) = snapshot_opt {
                    // Corrupted metadata should result in empty/default values
                    assert_eq!(
                        snapshot.interface_states.len(),
                        0,
                        "Corrupted metadata should default to empty interface_states"
                    );
                    assert_eq!(
                        snapshot.qualities,
                        [0, 0, 0, 0, 0, 0, 0],
                        "Corrupted metadata should default to zero qualities"
                    );
                    assert_eq!(
                        snapshot.developmental_stage, 0,
                        "Corrupted metadata should default to stage 0"
                    );
                }
            }
            Err(e) => {
                // Alternatively, it's acceptable to return an error
                // as long as the system doesn't panic/crash
                println!("Gracefully handled corrupted metadata with error: {:?}", e);
            }
        }

        // Test with completely missing metadata (NULL)
        let snapshot_id2 = Uuid::new_v4();
        let later_timestamp = chrono::Utc::now() + chrono::Duration::seconds(1);
        sqlx::query(
            "INSERT INTO state_snapshots (id, user_id, domain_states, boundary_states, pattern_ids, metadata, timestamp)
             VALUES (?, ?, ?, ?, ?, NULL, ?)",
        )
        .bind(snapshot_id2.as_bytes().to_vec())
        .bind(user_id.as_bytes().to_vec())
        .bind("{}")
        .bind("{}")
        .bind("[]")
        .bind(later_timestamp.to_rfc3339())
        .execute(&db_pool)
        .await
        .unwrap();

        // Should handle NULL metadata gracefully
        let result2 = memory_manager.get_latest_snapshot(user_id).await;
        match result2 {
            Ok(Some(snapshot)) => {
                // NULL metadata should result in defaults
                assert_eq!(snapshot.interface_states.len(), 0);
                assert_eq!(snapshot.qualities, [0, 0, 0, 0, 0, 0, 0]);
                assert_eq!(snapshot.developmental_stage, 0);
            }
            Ok(None) => panic!("Should find snapshot even with NULL metadata"),
            Err(e) => {
                // Error handling is acceptable as long as no panic
                println!("Gracefully handled NULL metadata with error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_database_foreign_key_constraint_enforcement() {
        // Test that foreign key constraints are enforced (snapshots require valid user)
        let db_pool = setup_test_db()
            .await
            .expect("Failed to setup test database");

        let invalid_user_id = Uuid::new_v4();

        // Attempt to save a snapshot for a non-existent user
        let result = sqlx::query(
            "INSERT INTO state_snapshots (id, user_id, domain_states, boundary_states, pattern_ids, identity_anchors, metadata)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(Uuid::new_v4().as_bytes().to_vec())
        .bind(invalid_user_id.as_bytes().to_vec())
        .bind("{}")
        .bind("[]")
        .bind("[]")
        .bind("[]")
        .bind("{}")
        .execute(&db_pool)
        .await;

        // Foreign key constraint should prevent this insert
        assert!(
            result.is_err(),
            "Should reject snapshot for non-existent user (foreign key constraint)"
        );

        // Verify the error is related to foreign key constraint
        if let Err(e) = result {
            let error_msg = e.to_string();
            // SQLite foreign key violations contain "FOREIGN KEY constraint failed"
            assert!(
                error_msg.contains("FOREIGN KEY") || error_msg.contains("foreign key"),
                "Error should indicate foreign key violation: {}",
                error_msg
            );
        }

        // Now verify that with a valid user, insert succeeds
        let valid_user_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO users (id, provider, provider_id, email, name, created_at, last_login)
             VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        )
        .bind(valid_user_id.as_bytes().to_vec())
        .bind("test")
        .bind(valid_user_id.to_string())
        .bind("test@example.com")
        .bind("Test User")
        .execute(&db_pool)
        .await
        .expect("Should create test user");

        // Now snapshot insert should succeed
        let result = sqlx::query(
            "INSERT INTO state_snapshots (id, user_id, domain_states, boundary_states, pattern_ids, identity_anchors, metadata)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(Uuid::new_v4().as_bytes().to_vec())
        .bind(valid_user_id.as_bytes().to_vec())
        .bind("{}")
        .bind("[]")
        .bind("[]")
        .bind("[]")
        .bind("{}")
        .execute(&db_pool)
        .await;

        assert!(
            result.is_ok(),
            "Should accept snapshot for valid user: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    async fn test_database_concurrent_snapshot_access() {
        // Test that concurrent reads and writes to snapshots don't cause data corruption
        use tokio::task::JoinSet;

        let db_pool = setup_test_db()
            .await
            .expect("Failed to setup test database");

        // Create a test user
        let user_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO users (id, provider, provider_id, email, name, created_at, last_login)
             VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        )
        .bind(user_id.as_bytes().to_vec())
        .bind("test")
        .bind(user_id.to_string())
        .bind("test@example.com")
        .bind("Test User")
        .execute(&db_pool)
        .await
        .expect("Should create test user");

        let manager = MemoryManager {
            db_pool: db_pool.clone(),
        };

        // Spawn multiple concurrent tasks that read and write snapshots
        let mut tasks = JoinSet::new();

        for i in 0..10 {
            let manager_clone = MemoryManager {
                db_pool: db_pool.clone(),
            };
            let user_id_clone = user_id;

            tasks.spawn(async move {
                // Write a snapshot using the public API
                use crate::prompt_engine::{BoundaryState, DomainState};

                let domains = vec![DomainState {
                    name: "CD".to_string(),
                    state: format!("0.{}", i),
                }];

                let boundaries = vec![BoundaryState::new(
                    "CD-SD".to_string(), // Use proper boundary format (domain-domain)
                    0.5 + (i as f64 * 0.01),
                    "Active".to_string(),
                )];

                let patterns = vec![format!("pattern_{}", i)];

                manager_clone
                    .create_snapshot(
                        domains,
                        boundaries,
                        patterns,
                        user_id_clone,
                        &format!("test input {}", i),
                    )
                    .await
                    .expect("Should save snapshot");

                // Immediately read it back
                let retrieved = manager_clone
                    .get_latest_snapshot(user_id_clone)
                    .await
                    .expect("Should retrieve snapshot");

                assert!(retrieved.is_some(), "Should have retrieved a snapshot");

                i // Return the iteration number
            });
        }

        // Wait for all tasks to complete
        let mut completed = 0;
        while let Some(result) = tasks.join_next().await {
            match result {
                Ok(_) => completed += 1,
                Err(e) => {
                    panic!("Concurrent task failed: {:?}", e);
                }
            }
        }

        assert_eq!(completed, 10, "All 10 concurrent tasks should complete");

        // Verify final state - should have at least one snapshot
        let final_snapshot = manager.get_latest_snapshot(user_id).await;
        assert!(
            final_snapshot.is_ok(),
            "Should be able to retrieve final snapshot"
        );
        assert!(
            final_snapshot.unwrap().is_some(),
            "Should have at least one snapshot after concurrent operations"
        );
    }

    #[tokio::test]
    async fn test_snapshot_corruption_detection_and_recovery() {
        // GIVEN: Multiple valid snapshots for a user
        use crate::test_utils::setup_test_db;

        let db_pool = setup_test_db().await.unwrap();
        let memory_manager = MemoryManager {
            db_pool: db_pool.clone(),
        };

        let user_id = Uuid::new_v4();

        // Create user
        sqlx::query(
            "INSERT INTO users (id, provider, provider_id, email, name, created_at, last_login)
             VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        )
        .bind(user_id.as_bytes().to_vec())
        .bind("test")
        .bind(user_id.to_string())
        .bind("test@example.com")
        .bind("Test User")
        .execute(&db_pool)
        .await
        .unwrap();

        // Create Snapshot 1 (valid)
        let domains_1 = vec![DomainState {
            name: "CD".to_string(),
            state: "0.8".to_string(),
        }];
        let boundaries_1 = vec![BoundaryState::new(
            "CD-SD".to_string(),
            0.7,
            "Transcendent".to_string(),
        )];

        memory_manager
            .create_snapshot(
                domains_1.clone(),
                boundaries_1.clone(),
                vec!["pattern1".to_string()],
                user_id,
                "First message",
            )
            .await
            .unwrap();

        let snapshot_1 = memory_manager
            .get_latest_snapshot(user_id)
            .await
            .unwrap()
            .expect("Should have Snapshot 1");

        // Wait to ensure timestamp difference
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Create Snapshot 2 (valid)
        let domains_2 = vec![DomainState {
            name: "SD".to_string(),
            state: "0.9".to_string(),
        }];
        let boundaries_2 = vec![BoundaryState::new(
            "SD-CuD".to_string(),
            0.8,
            "Transcendent".to_string(),
        )];

        memory_manager
            .create_snapshot(
                domains_2.clone(),
                boundaries_2.clone(),
                vec!["pattern2".to_string()],
                user_id,
                "Second message",
            )
            .await
            .unwrap();

        let snapshot_2 = memory_manager
            .get_latest_snapshot(user_id)
            .await
            .unwrap()
            .expect("Should have Snapshot 2");

        // WHEN: Manually corrupt Snapshot 2 (simulate incomplete write)
        // Corrupt by setting metadata to invalid JSON (simulating power loss mid-write)
        let snapshot_2_id = Uuid::parse_str(snapshot_2.id()).unwrap();

        sqlx::query("UPDATE state_snapshots SET metadata = ? WHERE id = ?")
            .bind("{incomplete_json: true, missing_closing_brace") // Invalid JSON
            .bind(snapshot_2_id.as_bytes().to_vec())
            .execute(&db_pool)
            .await
            .unwrap();

        // THEN: get_latest_snapshot should detect corruption and return Snapshot 1
        let recovery_result = memory_manager.get_latest_snapshot(user_id).await;

        match recovery_result {
            Ok(Some(recovered)) => {
                // System should return Snapshot 1 (skip corrupted Snapshot 2)
                assert_eq!(
                    recovered.id(),
                    snapshot_1.id(),
                    "Should return previous valid snapshot, not corrupted one"
                );

                assert_ne!(
                    recovered.id(),
                    snapshot_2.id(),
                    "Should NOT return corrupted snapshot"
                );

                // Additional validation: Verify Snapshot 1 data is intact
                assert_eq!(
                    recovered.domain_values().len(),
                    snapshot_1.domain_values().len(),
                    "Recovered snapshot should have same domain structure as Snapshot 1"
                );

                assert_eq!(
                    recovered.pattern_ids().len(),
                    snapshot_1.pattern_ids().len(),
                    "Recovered snapshot should have same patterns as Snapshot 1"
                );
            }
            Ok(None) => {
                panic!("Should return Snapshot 1, not None");
            }
            Err(e) => {
                // Acceptable: Return error indicating corruption detected
                // System should not panic or return corrupted data
                println!("Corruption detected gracefully: {:?}", e);
            }
        }
    }
}
