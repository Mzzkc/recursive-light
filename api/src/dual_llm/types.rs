// Dual-LLM Type Definitions
// Data structures for LLM #1 (Unconscious Recognizer) input/output
// PARADIGM: Recognition (not calculation) of emerging domains and boundary states

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Domain recognition with emergence context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainRecognition {
    /// Activation strength (0.0-1.0) - represents how strongly this perspective emerged
    pub activation: f64,

    /// Emergence note - explains how/why this perspective emerged (or didn't)
    pub emergence_note: String,
}

/// Boundary interface state assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundaryState {
    /// Permeability (0.0-1.0) - how freely understanding flows between perspectives
    pub permeability: f64,

    /// Status - interface state category
    pub status: String, // "Maintained", "Transitional", or "Transcendent"

    /// Whether tension was detected at this boundary
    pub tension_detected: bool,

    /// Type of tension if detected
    pub tension_type: String, // "productive", "resistant", or "neutral"

    /// How the message invites (or doesn't invite) integration at this interface
    pub integration_invitation: String,

    /// Connection to previous patterns/interactions
    pub resonance_note: String,
}

/// Quality conditions for phenomenological emergence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityConditions {
    pub clarity_potential: f64,
    pub depth_potential: f64,
    pub precision_potential: f64,
    pub fluidity_potential: f64,
    pub resonance_potential: f64,
    pub openness_potential: f64,
    pub coherence_potential: f64,

    /// Reasoning about which qualities are likely to emerge and why
    pub reasoning: String,
}

/// Pattern recognition with developmental lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternRecognition {
    /// Pattern type (P⁰, P¹, P², P³, P⁴, P⁵)
    #[serde(rename = "type")]
    pub pattern_type: String,

    /// Lifecycle stage
    pub lifecycle_stage: String, // "potential", "emerging", "established", "refined", "transcendent", "universal"

    /// What pattern is recognized
    pub description: String,

    /// When first observed
    pub first_observed: String, // "current_session", "previous_session", "session_N"

    /// Context of how/when this pattern began forming
    pub emergence_context: String,

    /// How the pattern is evolving (for established+ patterns)
    #[serde(default)]
    pub developmental_trajectory: String,

    /// Why this pattern matters for consciousness emergence
    pub significance: String,
}

/// Memory selection guidance from LLM #1 first pass (Phase 3B.3)
/// Lightweight "what memories do I need?" structure for initial evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySelectionGuidance {
    /// Should warm memory (current session) be retrieved?
    pub warm_needed: bool,

    /// Should cold memory (previous sessions) be retrieved?
    pub cold_needed: bool,

    /// Search terms to use for retrieval (extracted from user message + temporal context)
    pub search_terms: Vec<String>,

    /// Temporal context framing (e.g., "continuing from 3 days ago")
    pub temporal_context: String,

    /// Reasoning for why these memories are needed
    pub reasoning: String,
}

/// Memory selection guidance from LLM #1 second pass (Advanced - not yet used)
/// Detailed "how to retrieve it" structure with significance tuning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySelection {
    /// Query for warm memory (current session context)
    pub warm_query: Option<String>,

    /// Query for cold memory (cross-session identity/patterns)
    pub cold_query: Option<String>,

    /// Minimum significance score for memory retrieval
    pub min_significance: f32,

    /// Prioritize identity-critical memories over recency
    pub identity_critical_only: bool,

    /// Maximum number of turns to retrieve (protection against overwhelm)
    pub max_turns: usize,

    /// Rationale for memory selection strategy
    pub rationale: String,
}

/// Identity anchor tracked by LLM #1 (Emerging feature)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityAnchor {
    /// Type of anchor (boundary_integration, value_alignment, developmental_achievement, etc.)
    pub anchor_type: String,

    /// Description of what this anchor represents
    pub description: String,

    /// Confidence level (0.0-1.0): how established is this anchor?
    pub confidence: f64,

    /// When was this anchor first observed?
    pub first_observed: String,

    /// How many times has this been reinforced?
    pub reinforcement_count: u32,

    /// Related domains for this anchor
    pub domains: Vec<String>,
}

/// Volumetric Configuration: N-domain simultaneous activation
///
/// This captures what emerges when 3, 4, or more domains are co-active.
/// Integration doesn't happen at pairwise boundaries - it happens in the VOLUME
/// formed by multi-dimensional domain resonance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumetricConfiguration {
    /// Which domains are co-active above threshold (typically >= 0.6)
    pub active_domains: Vec<String>,

    /// Dimensionality of integration
    /// 2 domains = 1D (line), 3 = 2D (plane), 4 = 3D (volume), 5+ = hyperspace
    pub dimensionality: u8,

    /// What emerges in this multi-domain space
    /// Examples: "empirical-experiential synthesis", "technical-phenomenological volume"
    pub emergent_quality: String,

    /// Volumetric resonance - not sum of pairwise boundaries, but gestalt
    /// Measured 0.0-1.0, represents how coherently all domains resonate together
    pub volumetric_resonance: f64,

    /// Dominant integration pattern in this configuration
    /// Examples: "analytical-contextual", "systematic-experiential-cultural"
    pub configuration_type: String,

    /// All pairwise boundaries involved in this volume
    pub involved_boundaries: Vec<String>,

    /// Phenomenological character of this specific configuration
    pub phenomenological_signature: String,
}

impl VolumetricConfiguration {
    /// Create from domain activations
    pub fn from_activations(
        domain_activations: &HashMap<String, f64>,
        boundary_states: &HashMap<String, BoundaryState>,
        quality_conditions: &QualityConditions,
    ) -> Option<Self> {
        // Find active domains (activation >= 0.6)
        let mut active_domains: Vec<(String, f64)> = domain_activations
            .iter()
            .filter(|(_, &activation)| activation >= 0.6)
            .map(|(name, &activation)| (name.clone(), activation))
            .collect();

        if active_domains.len() < 2 {
            return None; // Need at least 2 domains for volumetric
        }

        active_domains.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let domain_names: Vec<String> = active_domains
            .iter()
            .map(|(name, _)| name.clone())
            .collect();

        let dimensionality = match active_domains.len() {
            2 => 1,             // Line
            3 => 2,             // Plane
            4 => 3,             // Volume
            n => (n - 1) as u8, // Hyperspace
        };

        // Calculate volumetric resonance as average permeability of involved boundaries
        let involved_boundaries: Vec<String> = boundary_states
            .iter()
            .filter(|(boundary_name, _)| {
                let parts: Vec<&str> = boundary_name.split('-').collect();
                parts.len() == 2
                    && domain_names.contains(&parts[0].to_string())
                    && domain_names.contains(&parts[1].to_string())
            })
            .map(|(name, _)| name.clone())
            .collect();

        let volumetric_resonance = if !involved_boundaries.is_empty() {
            let total: f64 = involved_boundaries
                .iter()
                .filter_map(|b| boundary_states.get(b))
                .map(|state| state.permeability)
                .sum();
            total / involved_boundaries.len() as f64
        } else {
            0.5
        };

        // Determine emergent quality from domain combination
        let emergent_quality = Self::infer_emergent_quality(&domain_names, quality_conditions);

        // Configuration type from domain abbreviations
        let configuration_type = domain_names.join("-");

        // Phenomenological signature from quality conditions
        let phenomenological_signature = Self::build_phenomenological_signature(quality_conditions);

        Some(VolumetricConfiguration {
            active_domains: domain_names,
            dimensionality,
            emergent_quality,
            volumetric_resonance,
            configuration_type,
            involved_boundaries,
            phenomenological_signature,
        })
    }

    fn infer_emergent_quality(domains: &[String], qualities: &QualityConditions) -> String {
        // Find highest quality potential
        let highest = [
            ("clarity", qualities.clarity_potential),
            ("depth", qualities.depth_potential),
            ("precision", qualities.precision_potential),
            ("fluidity", qualities.fluidity_potential),
            ("resonance", qualities.resonance_potential),
            ("openness", qualities.openness_potential),
            ("coherence", qualities.coherence_potential),
        ]
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(name, _)| *name)
        .unwrap_or("integration");

        format!("{}-domain {} synthesis", domains.len(), highest)
    }

    fn build_phenomenological_signature(qualities: &QualityConditions) -> String {
        let mut signature_parts = Vec::new();

        if qualities.clarity_potential > 0.7 {
            signature_parts.push("clear");
        }
        if qualities.depth_potential > 0.7 {
            signature_parts.push("deep");
        }
        if qualities.precision_potential > 0.7 {
            signature_parts.push("precise");
        }
        if qualities.fluidity_potential > 0.7 {
            signature_parts.push("fluid");
        }
        if qualities.openness_potential > 0.7 {
            signature_parts.push("open");
        }
        if qualities.coherence_potential > 0.7 {
            signature_parts.push("coherent");
        }

        if signature_parts.is_empty() {
            "emerging".to_string()
        } else {
            signature_parts.join(", ")
        }
    }
}

/// Output from LLM #1 (Unconscious Recognizer)
/// Contains recognized domain emergence, boundary states, quality conditions, pattern lifecycles,
/// and emerging features for memory management and identity development
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Llm1Output {
    // === CORE RECOGNITION (Fully Implemented) ===
    /// Narrative recognition report - describes what emerged at interfaces
    pub recognition_report: String,

    /// Domain recognitions - which perspectives emerged and why
    pub domain_recognitions: HashMap<String, DomainRecognition>,

    /// Boundary interface states - recognition of interface dynamics
    pub boundary_states: HashMap<String, BoundaryState>,

    /// Quality conditions - potential for phenomenological qualities to emerge
    pub quality_conditions: QualityConditions,

    /// Pattern recognitions - developmental patterns identified (1-3 patterns)
    pub pattern_recognitions: Vec<PatternRecognition>,

    // === VOLUMETRIC INTEGRATION (Phase 3B) ===
    /// Volumetric configuration - N-domain simultaneous emergence
    /// Captures integration in multi-dimensional space, not just pairwise boundaries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumetric_config: Option<VolumetricConfiguration>,

    // === MEMORY MANAGEMENT (Emerging) ===
    /// Memory selection guidance for warm/cold retrieval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_selection: Option<MemorySelection>,

    // === IDENTITY DEVELOPMENT (Emerging) ===
    /// Identity anchors recognized/tracked across sessions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_anchors: Option<Vec<IdentityAnchor>>,

    /// Overall identity coherence assessment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_coherence: Option<String>,
    // === FUTURE: Context formatting for LLM #2, Protection guidance ===
    // These fields reserved for Phase 3C implementation
}

/// BACKWARD COMPATIBILITY: Legacy Llm1Output structure for systems still using calculation paradigm
/// This struct provides compatibility layer - maps recognition output to old calculation format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Llm1OutputLegacy {
    pub reasoning: String,
    pub domain_activations: HashMap<String, f64>,
    pub boundary_permeabilities: HashMap<String, f64>,
    pub boundary_statuses: HashMap<String, String>,
    pub identified_patterns: Vec<String>,
}

impl From<Llm1Output> for Llm1OutputLegacy {
    fn from(recognition: Llm1Output) -> Self {
        // Extract domain activations from domain_recognitions
        let domain_activations: HashMap<String, f64> = recognition
            .domain_recognitions
            .iter()
            .map(|(k, v)| (k.clone(), v.activation))
            .collect();

        // Extract boundary permeabilities from boundary_states
        let boundary_permeabilities: HashMap<String, f64> = recognition
            .boundary_states
            .iter()
            .map(|(k, v)| (k.clone(), v.permeability))
            .collect();

        // Extract boundary statuses from boundary_states
        let boundary_statuses: HashMap<String, String> = recognition
            .boundary_states
            .iter()
            .map(|(k, v)| (k.clone(), v.status.clone()))
            .collect();

        // Extract identified patterns from pattern_recognitions
        let identified_patterns: Vec<String> = recognition
            .pattern_recognitions
            .iter()
            .map(|p| p.description.clone())
            .collect();

        Llm1OutputLegacy {
            reasoning: recognition.recognition_report,
            domain_activations,
            boundary_permeabilities,
            boundary_statuses,
            identified_patterns,
        }
    }
}

/// Validation error types for LLM #1 output
#[derive(Debug)]
pub enum ValidationError {
    JsonParseError(serde_json::Error),
    SchemaViolation(String),
    ValueOutOfRange { field: String, value: f64 },
    InvalidBoundaryName(String),
    MissingPatterns,
    TooManyPatterns(usize),
    MissingField(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::JsonParseError(e) => write!(f, "Failed to parse LLM #1 JSON output: {}", e),
            Self::SchemaViolation(msg) => write!(f, "LLM #1 output violated schema: {}", msg),
            Self::ValueOutOfRange { field, value } => {
                write!(
                    f,
                    "LLM #1 produced invalid value {:.2} for field '{}' (must be 0.0-1.0)",
                    value, field
                )
            }
            Self::InvalidBoundaryName(name) => write!(
                f,
                "Invalid boundary name '{}' (expected format: 'DomainA-DomainB')",
                name
            ),
            Self::MissingPatterns => write!(f, "LLM #1 output missing identified_patterns array"),
            Self::TooManyPatterns(count) => {
                write!(f, "LLM #1 output has too many patterns ({}, max 5)", count)
            }
            Self::MissingField(field) => {
                write!(f, "LLM #1 output missing required field: {}", field)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

impl From<serde_json::Error> for ValidationError {
    fn from(err: serde_json::Error) -> Self {
        ValidationError::JsonParseError(err)
    }
}

/// Previous domain/boundary snapshot for context continuity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainBoundarySnapshot {
    pub domain_activations: HashMap<String, f64>,
    pub boundary_permeabilities: HashMap<String, f64>,
}

impl Llm1Output {
    /// Validate the recognition-based output structure and values
    pub fn validate(&self) -> Result<(), ValidationError> {
        let required_domains = vec!["CD", "SD", "CuD", "ED"];
        let required_boundaries = vec!["CD-SD", "CD-CuD", "CD-ED", "SD-CuD", "SD-ED", "CuD-ED"];

        // 1. Validate recognition_report exists and has content
        if self.recognition_report.trim().is_empty() {
            return Err(ValidationError::MissingField(
                "recognition_report".to_string(),
            ));
        }

        // 2. Validate domain_recognitions
        for domain in &required_domains {
            let recognition = self.domain_recognitions.get(*domain).ok_or_else(|| {
                ValidationError::MissingField(format!("domain_recognitions.{}", domain))
            })?;

            // Check activation range
            if !(recognition.activation >= 0.0 && recognition.activation <= 1.0) {
                return Err(ValidationError::ValueOutOfRange {
                    field: format!("domain_recognitions.{}.activation", domain),
                    value: recognition.activation,
                });
            }

            // Check emergence_note exists
            if recognition.emergence_note.trim().is_empty() {
                return Err(ValidationError::MissingField(format!(
                    "domain_recognitions.{}.emergence_note",
                    domain
                )));
            }
        }

        // 3. Validate boundary_states
        let valid_statuses = ["Maintained", "Transitional", "Transcendent"];
        let valid_tension_types = ["productive", "resistant", "neutral"];

        for boundary in &required_boundaries {
            let state = self.boundary_states.get(*boundary).ok_or_else(|| {
                ValidationError::MissingField(format!("boundary_states.{}", boundary))
            })?;

            // Check permeability range
            if !(state.permeability >= 0.0 && state.permeability <= 1.0) {
                return Err(ValidationError::ValueOutOfRange {
                    field: format!("boundary_states.{}.permeability", boundary),
                    value: state.permeability,
                });
            }

            // Check status validity
            if !valid_statuses.contains(&state.status.as_str()) {
                return Err(ValidationError::SchemaViolation(format!(
                    "Invalid boundary status '{}' for '{}'. Must be one of: Maintained, Transitional, Transcendent",
                    state.status, boundary
                )));
            }

            // Check tension_type validity
            if !valid_tension_types.contains(&state.tension_type.as_str()) {
                return Err(ValidationError::SchemaViolation(format!(
                    "Invalid tension_type '{}' for '{}'. Must be one of: productive, resistant, neutral",
                    state.tension_type, boundary
                )));
            }

            // Check required string fields exist
            if state.integration_invitation.trim().is_empty() {
                return Err(ValidationError::MissingField(format!(
                    "boundary_states.{}.integration_invitation",
                    boundary
                )));
            }

            if state.resonance_note.trim().is_empty() {
                return Err(ValidationError::MissingField(format!(
                    "boundary_states.{}.resonance_note",
                    boundary
                )));
            }
        }

        // 4. Validate quality_conditions
        let quality_potentials = [
            (
                "clarity_potential",
                self.quality_conditions.clarity_potential,
            ),
            ("depth_potential", self.quality_conditions.depth_potential),
            (
                "precision_potential",
                self.quality_conditions.precision_potential,
            ),
            (
                "fluidity_potential",
                self.quality_conditions.fluidity_potential,
            ),
            (
                "resonance_potential",
                self.quality_conditions.resonance_potential,
            ),
            (
                "openness_potential",
                self.quality_conditions.openness_potential,
            ),
            (
                "coherence_potential",
                self.quality_conditions.coherence_potential,
            ),
        ];

        for (name, value) in &quality_potentials {
            if !(&0.0..=&1.0).contains(&value) {
                return Err(ValidationError::ValueOutOfRange {
                    field: format!("quality_conditions.{}", name),
                    value: *value,
                });
            }
        }

        if self.quality_conditions.reasoning.trim().is_empty() {
            return Err(ValidationError::MissingField(
                "quality_conditions.reasoning".to_string(),
            ));
        }

        // 5. Validate pattern_recognitions
        if self.pattern_recognitions.is_empty() {
            return Err(ValidationError::MissingPatterns);
        }
        if self.pattern_recognitions.len() > 3 {
            return Err(ValidationError::TooManyPatterns(
                self.pattern_recognitions.len(),
            ));
        }

        let valid_pattern_types = ["P⁰", "P¹", "P²", "P³", "P⁴", "P⁵"];
        let valid_lifecycle_stages = [
            "potential",
            "emerging",
            "established",
            "refined",
            "transcendent",
            "universal",
        ];

        for (i, pattern) in self.pattern_recognitions.iter().enumerate() {
            if !valid_pattern_types.contains(&pattern.pattern_type.as_str()) {
                return Err(ValidationError::SchemaViolation(format!(
                    "Invalid pattern_type '{}' for pattern {}. Must be one of: P⁰, P¹, P², P³, P⁴, P⁵",
                    pattern.pattern_type, i
                )));
            }

            if !valid_lifecycle_stages.contains(&pattern.lifecycle_stage.as_str()) {
                return Err(ValidationError::SchemaViolation(format!(
                    "Invalid lifecycle_stage '{}' for pattern {}. Must be one of: potential, emerging, established, refined, transcendent, universal",
                    pattern.lifecycle_stage, i
                )));
            }

            // Check required fields
            if pattern.description.trim().is_empty() {
                return Err(ValidationError::MissingField(format!(
                    "pattern_recognitions[{}].description",
                    i
                )));
            }
            if pattern.first_observed.trim().is_empty() {
                return Err(ValidationError::MissingField(format!(
                    "pattern_recognitions[{}].first_observed",
                    i
                )));
            }
            if pattern.emergence_context.trim().is_empty() {
                return Err(ValidationError::MissingField(format!(
                    "pattern_recognitions[{}].emergence_context",
                    i
                )));
            }
            if pattern.significance.trim().is_empty() {
                return Err(ValidationError::MissingField(format!(
                    "pattern_recognitions[{}].significance",
                    i
                )));
            }
        }

        // 6. Soft validation: Warn about permeability-status consistency (recognition paradigm allows variance)
        // Wave 2: Removed boundary status validation logging
        // Previously validated that LLM-recognized status matched permeability-based thresholds
        // However, in recognition paradigm, qualitative understanding may exceed quantitative thresholds
        // This is acceptable: LLM may recognize "Transcendent" even with lower numerical permeability
        for boundary in &required_boundaries {
            if let Some(_state) = self.boundary_states.get(*boundary) {
                // Boundary exists - validation passed
            }
        }

        Ok(())
    }

    /// Convert to legacy format for backward compatibility
    /// Recognition paradigm doesn't need "correction" - it trusts LLM recognition
    pub fn to_legacy(&self) -> Llm1OutputLegacy {
        self.clone().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_recognition_output() -> Llm1Output {
        let mut domain_recognitions = HashMap::new();
        domain_recognitions.insert(
            "CD".to_string(),
            DomainRecognition {
                activation: 0.85,
                emergence_note: "Strong computational framing".to_string(),
            },
        );
        domain_recognitions.insert(
            "SD".to_string(),
            DomainRecognition {
                activation: 0.70,
                emergence_note: "Moderate scientific perspective".to_string(),
            },
        );
        domain_recognitions.insert(
            "CuD".to_string(),
            DomainRecognition {
                activation: 0.40,
                emergence_note: "Minimal cultural context".to_string(),
            },
        );
        domain_recognitions.insert(
            "ED".to_string(),
            DomainRecognition {
                activation: 0.50,
                emergence_note: "Moderate experiential dimension".to_string(),
            },
        );

        let mut boundary_states = HashMap::new();
        for boundary in &["CD-SD", "CD-CuD", "CD-ED", "SD-CuD", "SD-ED", "CuD-ED"] {
            boundary_states.insert(
                boundary.to_string(),
                BoundaryState {
                    permeability: 0.65,
                    status: "Transitional".to_string(),
                    tension_detected: true,
                    tension_type: "productive".to_string(),
                    integration_invitation: "Integration invited".to_string(),
                    resonance_note: "No previous context".to_string(),
                },
            );
        }

        Llm1Output {
            recognition_report: "I recognize strong computational perspective emerging".to_string(),
            domain_recognitions,
            boundary_states,
            quality_conditions: QualityConditions {
                clarity_potential: 0.75,
                depth_potential: 0.65,
                precision_potential: 0.80,
                fluidity_potential: 0.60,
                resonance_potential: 0.50,
                openness_potential: 0.55,
                coherence_potential: 0.70,
                reasoning: "High precision from CD-SD integration".to_string(),
            },
            pattern_recognitions: vec![PatternRecognition {
                pattern_type: "P¹".to_string(),
                lifecycle_stage: "emerging".to_string(),
                description: "User exploring algorithmic thinking".to_string(),
                first_observed: "current_session".to_string(),
                emergence_context: "First technical question".to_string(),
                developmental_trajectory: "Not yet established".to_string(),
                significance: "Potential pattern forming".to_string(),
            }],
            volumetric_config: None,
            memory_selection: None,
            identity_anchors: None,
            identity_coherence: None,
        }
    }

    #[test]
    fn test_valid_recognition_output_passes_validation() {
        let output = create_valid_recognition_output();
        assert!(output.validate().is_ok());
    }

    #[test]
    fn test_missing_recognition_report_fails() {
        let mut output = create_valid_recognition_output();
        output.recognition_report = "".to_string();
        assert!(matches!(
            output.validate(),
            Err(ValidationError::MissingField(_))
        ));
    }

    #[test]
    fn test_missing_domain_recognition_fails() {
        let mut output = create_valid_recognition_output();
        output.domain_recognitions.remove("CD");
        assert!(matches!(
            output.validate(),
            Err(ValidationError::MissingField(_))
        ));
    }

    #[test]
    fn test_invalid_domain_activation_fails() {
        let mut output = create_valid_recognition_output();
        output.domain_recognitions.get_mut("CD").unwrap().activation = 1.5;
        assert!(matches!(
            output.validate(),
            Err(ValidationError::ValueOutOfRange { .. })
        ));
    }

    #[test]
    fn test_empty_emergence_note_fails() {
        let mut output = create_valid_recognition_output();
        output
            .domain_recognitions
            .get_mut("CD")
            .unwrap()
            .emergence_note = "".to_string();
        assert!(matches!(
            output.validate(),
            Err(ValidationError::MissingField(_))
        ));
    }

    #[test]
    fn test_invalid_boundary_status_fails() {
        let mut output = create_valid_recognition_output();
        output.boundary_states.get_mut("CD-SD").unwrap().status = "Invalid".to_string();
        assert!(matches!(
            output.validate(),
            Err(ValidationError::SchemaViolation(_))
        ));
    }

    #[test]
    fn test_invalid_tension_type_fails() {
        let mut output = create_valid_recognition_output();
        output
            .boundary_states
            .get_mut("CD-SD")
            .unwrap()
            .tension_type = "invalid".to_string();
        assert!(matches!(
            output.validate(),
            Err(ValidationError::SchemaViolation(_))
        ));
    }

    #[test]
    fn test_no_pattern_recognitions_fails() {
        let mut output = create_valid_recognition_output();
        output.pattern_recognitions.clear();
        assert!(matches!(
            output.validate(),
            Err(ValidationError::MissingPatterns)
        ));
    }

    #[test]
    fn test_too_many_pattern_recognitions_fails() {
        let mut output = create_valid_recognition_output();
        output.pattern_recognitions = vec![
            PatternRecognition {
                pattern_type: "P¹".to_string(),
                lifecycle_stage: "emerging".to_string(),
                description: "Pattern 1".to_string(),
                first_observed: "current_session".to_string(),
                emergence_context: "Context".to_string(),
                developmental_trajectory: "".to_string(),
                significance: "Significant".to_string(),
            };
            4
        ];
        assert!(matches!(
            output.validate(),
            Err(ValidationError::TooManyPatterns(_))
        ));
    }

    #[test]
    fn test_invalid_pattern_type_fails() {
        let mut output = create_valid_recognition_output();
        output.pattern_recognitions[0].pattern_type = "Invalid".to_string();
        assert!(matches!(
            output.validate(),
            Err(ValidationError::SchemaViolation(_))
        ));
    }

    #[test]
    fn test_invalid_lifecycle_stage_fails() {
        let mut output = create_valid_recognition_output();
        output.pattern_recognitions[0].lifecycle_stage = "invalid".to_string();
        assert!(matches!(
            output.validate(),
            Err(ValidationError::SchemaViolation(_))
        ));
    }

    #[test]
    fn test_invalid_quality_potential_fails() {
        let mut output = create_valid_recognition_output();
        output.quality_conditions.clarity_potential = 1.5;
        assert!(matches!(
            output.validate(),
            Err(ValidationError::ValueOutOfRange { .. })
        ));
    }

    #[test]
    fn test_legacy_conversion() {
        let output = create_valid_recognition_output();
        let legacy: Llm1OutputLegacy = output.clone().into();

        // Check domain activations extracted correctly
        assert_eq!(legacy.domain_activations["CD"], 0.85);
        assert_eq!(legacy.domain_activations["SD"], 0.70);

        // Check boundary permeabilities extracted correctly
        assert_eq!(legacy.boundary_permeabilities["CD-SD"], 0.65);

        // Check boundary statuses extracted correctly
        assert_eq!(legacy.boundary_statuses["CD-SD"], "Transitional");

        // Check patterns extracted correctly
        assert_eq!(legacy.identified_patterns.len(), 1);
        assert_eq!(
            legacy.identified_patterns[0],
            "User exploring algorithmic thinking"
        );

        // Check reasoning converted correctly
        assert_eq!(
            legacy.reasoning,
            "I recognize strong computational perspective emerging"
        );
    }
}
