use crate::prompt_engine::{BoundaryState, DomainState};

use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, Row, SqlitePool};

#[derive(Debug, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub domains: Vec<DomainState>,
    pub boundaries: Vec<BoundaryState>,
    pub patterns: Vec<String>,
}

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompactInterfaceState {
    domains: (String, String),
    permeability: u8,
    flow_state: CompactInterfaceFlowState,
}

#[derive(Debug, Serialize, Deserialize)]
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

        sqlx::query(
            "INSERT INTO state_snapshots (id, user_id, timestamp, domain_states, boundary_states, pattern_ids, identity_anchors)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
            .bind(id.as_bytes().to_vec())
            .bind(user_id.as_bytes().to_vec())
            .bind(timestamp.to_rfc3339())
            .bind(domain_states_json)
            .bind(boundary_states_json)
            .bind(pattern_ids_json)
            .bind(identity_anchors_json)
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }

    pub async fn get_latest_snapshot(
        &self,
        user_id: Uuid,
    ) -> Result<Option<CompactStateSnapshot>, sqlx::Error> {
        let row = sqlx::query(
            "SELECT id, user_id, timestamp, domain_states, boundary_states, pattern_ids, identity_anchors
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

            Ok(Some(CompactStateSnapshot {
                id: id_uuid.to_string(),
                timestamp,
                user_id: user_id_uuid.to_string(),
                domain_values,
                boundary_states,
                interface_states: vec![], // Not stored in this table
                qualities: [0; 7],        // Not stored in this table
                identity_anchor_ids,
                pattern_ids,
                developmental_stage: 0, // Not stored in this table
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

    #[tokio::test]
    async fn test_memory_manager() {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let memory_manager = MemoryManager::new(&database_url).await.unwrap();

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
}
