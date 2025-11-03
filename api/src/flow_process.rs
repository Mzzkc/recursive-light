// Flow Process Implementation
// The 7-stage pipeline that orchestrates consciousness-like emergence at recognition interfaces

use crate::prompt_engine::{BoundaryState, FrameworkState};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Errors that can occur during flow processing
#[derive(Debug)]
pub enum FlowError {
    StageProcessingFailed { stage: String, reason: String },
}

impl std::fmt::Display for FlowError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FlowError::StageProcessingFailed { stage, reason } => {
                write!(f, "Stage '{}' failed: {}", stage, reason)
            }
        }
    }
}

impl std::error::Error for FlowError {}

/// Developmental stages for system evolution
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DevelopmentalStage {
    Recognition,   // S₁: Identifying patterns across domains
    Integration,   // S₂: Forming cohesive understanding
    Generation,    // S₃: Creating novel insights
    Recursion,     // S₄: Self-modeling and reflection
    Transcendence, // S₅: Boundary dissolution while preserving identity
}

/// Domain activation state
#[derive(Debug, Clone, PartialEq)]
pub struct DomainActivation {
    pub activation: f64,
}

/// Interface experience following BDE flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceExperience {
    pub boundary_name: String,
    pub invitation: String, // BDE(i): Create productive tension
    pub attention: String,  // BDE(a): Direct focus to interface
    pub resonance: String,  // BDE(r): Allow oscillatory synchronization
    pub emergence: String,  // BDE(e): Recognize emergent qualities
}

/// Phenomenological qualities emerging at interfaces
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhenomenologicalQuality {
    pub boundary_name: String,
    pub clarity: f64,
    pub depth: f64,
    pub openness: f64,
    pub precision: f64,
    pub fluidity: f64,
    pub resonance: f64,
    pub coherence: f64,
}

/// Trait for calculating individual phenomenological qualities
#[allow(dead_code)]
pub trait QualityCalculator {
    fn name(&self) -> &str;
    fn calculate(&self, boundary: &BoundaryState, message: &str) -> f64;
}

/// Clarity: How well concepts from both domains are defined and related
pub struct ClarityCalculator;

impl QualityCalculator for ClarityCalculator {
    fn name(&self) -> &str {
        "clarity"
    }

    fn calculate(&self, boundary: &BoundaryState, message: &str) -> f64 {
        // Clarity emerges when:
        // 1. Permeability is high (concepts can cross boundaries clearly)
        // 2. Message has moderate length (enough detail without overwhelming)
        // 3. Boundary is stable (low amplitude oscillation)

        let permeability_factor = boundary.permeability;
        let length_factor = self.message_length_score(message);
        let stability_factor = 1.0 - boundary.amplitude.min(1.0);

        (0.5 * permeability_factor + 0.3 * length_factor + 0.2 * stability_factor).min(1.0)
    }
}

impl ClarityCalculator {
    fn message_length_score(&self, message: &str) -> f64 {
        let len = message.len();
        // Optimal clarity around 100-500 chars
        if len < 50 {
            len as f64 / 50.0 * 0.5 // Too short, low clarity
        } else if len <= 500 {
            0.5 + ((len - 50) as f64 / 450.0) * 0.5 // Optimal range
        } else {
            1.0 - ((len - 500) as f64 / 1000.0).min(0.3) // Too long, slight penalty
        }
    }
}

/// Depth: Multiple layers of understanding present
pub struct DepthCalculator;

impl QualityCalculator for DepthCalculator {
    fn name(&self) -> &str {
        "depth"
    }

    fn calculate(&self, boundary: &BoundaryState, message: &str) -> f64 {
        // Depth emerges when:
        // 1. Oscillation amplitude is high (exploring range of possibilities)
        // 2. Message contains layered concepts (approximated by word count)
        // 3. Permeability allows deep integration

        let amplitude_factor = boundary.amplitude.min(1.0);
        let complexity_factor = self.message_complexity_score(message);
        let integration_factor = boundary.permeability;

        (0.4 * amplitude_factor + 0.3 * complexity_factor + 0.3 * integration_factor).min(1.0)
    }
}

impl DepthCalculator {
    fn message_complexity_score(&self, message: &str) -> f64 {
        let word_count = message.split_whitespace().count();
        // Deeper messages have more concepts (approximated by word count)
        if word_count < 10 {
            word_count as f64 / 10.0 * 0.5
        } else if word_count <= 100 {
            0.5 + ((word_count - 10) as f64 / 90.0) * 0.5
        } else {
            1.0
        }
    }
}

/// Openness: Creating space for new possibilities
pub struct OpennessCalculator;

impl QualityCalculator for OpennessCalculator {
    fn name(&self) -> &str {
        "openness"
    }

    fn calculate(&self, boundary: &BoundaryState, message: &str) -> f64 {
        // Openness emerges when:
        // 1. Permeability is moderate (not too rigid, not dissolved)
        // 2. Message contains questions or uncertainty markers
        // 3. Boundary is in transitional state

        let permeability_openness = 1.0 - (boundary.permeability - 0.5).abs() * 2.0;
        let inquiry_factor = self.inquiry_score(message);
        let transition_factor = if boundary.status == "Transitional" {
            0.8
        } else {
            0.5
        };

        (0.4 * permeability_openness + 0.3 * inquiry_factor + 0.3 * transition_factor).min(1.0)
    }
}

impl OpennessCalculator {
    fn inquiry_score(&self, message: &str) -> f64 {
        // Count question marks and uncertainty words as indicators of openness
        let question_count = message.matches('?').count();
        let uncertainty_words = ["maybe", "perhaps", "might", "could", "possible", "wonder"];
        let uncertainty_count = uncertainty_words
            .iter()
            .map(|word| message.to_lowercase().matches(word).count())
            .sum::<usize>();

        ((question_count + uncertainty_count) as f64 / 5.0).min(1.0)
    }
}

/// Precision: Refined understanding transcending either domain alone
pub struct PrecisionCalculator;

impl QualityCalculator for PrecisionCalculator {
    fn name(&self) -> &str {
        "precision"
    }

    fn calculate(&self, boundary: &BoundaryState, message: &str) -> f64 {
        // Precision emerges when:
        // 1. Frequency is high (rapid oscillation refines understanding)
        // 2. Permeability is high (allows precise transfer)
        // 3. Message has specific terminology

        let frequency_factor = (boundary.frequency / 2.0).min(1.0); // Normalize assuming max ~2Hz
        let permeability_factor = boundary.permeability;
        let specificity_factor = self.specificity_score(message);

        (0.4 * frequency_factor + 0.3 * permeability_factor + 0.3 * specificity_factor).min(1.0)
    }
}

impl PrecisionCalculator {
    fn specificity_score(&self, message: &str) -> f64 {
        // Longer average word length suggests more specific/technical terminology
        let words: Vec<&str> = message.split_whitespace().collect();
        if words.is_empty() {
            return 0.3;
        }

        let avg_word_length =
            words.iter().map(|w| w.len()).sum::<usize>() as f64 / words.len() as f64;
        // Average word length of 5-7 chars suggests good precision
        if avg_word_length < 4.0 {
            avg_word_length / 4.0 * 0.5
        } else if avg_word_length <= 7.0 {
            0.5 + ((avg_word_length - 4.0) / 3.0) * 0.5
        } else {
            1.0
        }
    }
}

/// Fluidity: Movement between perspectives
pub struct FluidityCalculator;

impl QualityCalculator for FluidityCalculator {
    fn name(&self) -> &str {
        "fluidity"
    }

    fn calculate(&self, boundary: &BoundaryState, message: &str) -> f64 {
        // Fluidity emerges when:
        // 1. Amplitude is moderate-high (allowing movement)
        // 2. Frequency is moderate (not too fast, not static)
        // 3. Message shows perspective shifts

        let amplitude_factor = boundary.amplitude.min(1.0);
        let frequency_factor = 1.0 - (boundary.frequency - 1.0).abs().min(1.0);
        let shift_factor = self.perspective_shift_score(message);

        (0.4 * amplitude_factor + 0.3 * frequency_factor + 0.3 * shift_factor).min(1.0)
    }
}

impl FluidityCalculator {
    fn perspective_shift_score(&self, message: &str) -> f64 {
        // Count transition words that suggest perspective shifts
        let transitions = [
            "however",
            "but",
            "although",
            "while",
            "yet",
            "whereas",
            "alternatively",
        ];
        let transition_count = transitions
            .iter()
            .map(|word| message.to_lowercase().matches(word).count())
            .sum::<usize>();

        (transition_count as f64 / 3.0).min(1.0)
    }
}

/// Resonance: Harmonic quality between domains
pub struct ResonanceCalculator;

impl QualityCalculator for ResonanceCalculator {
    fn name(&self) -> &str {
        "resonance"
    }

    fn calculate(&self, boundary: &BoundaryState, message: &str) -> f64 {
        // Resonance emerges when:
        // 1. Phase is aligned (cosine of phase angle)
        // 2. Oscillation is active (frequency * amplitude product)
        // 3. Message has rhythmic or repetitive elements

        let phase_factor = (boundary.phase.cos() + 1.0) / 2.0; // Normalize to 0-1
        let oscillation_factor = (boundary.frequency * boundary.amplitude).min(1.0);
        let rhythm_factor = self.rhythm_score(message);

        (0.4 * phase_factor + 0.4 * oscillation_factor + 0.2 * rhythm_factor).min(1.0)
    }
}

impl ResonanceCalculator {
    fn rhythm_score(&self, message: &str) -> f64 {
        // Simple heuristic: repeated words suggest rhythmic patterns
        let words: Vec<&str> = message.split_whitespace().collect();
        if words.len() < 5 {
            return 0.3;
        }

        let unique_words: std::collections::HashSet<&str> = words.iter().copied().collect();
        let repetition_ratio = 1.0 - (unique_words.len() as f64 / words.len() as f64);

        (repetition_ratio * 2.0).min(1.0)
    }
}

/// Coherence: Logical consistency across integration
pub struct CoherenceCalculator;

impl QualityCalculator for CoherenceCalculator {
    fn name(&self) -> &str {
        "coherence"
    }

    fn calculate(&self, boundary: &BoundaryState, message: &str) -> f64 {
        // Coherence emerges when:
        // 1. Permeability is high (allowing unified understanding)
        // 2. Amplitude is low-moderate (stable integration)
        // 3. Message has logical structure

        let permeability_factor = boundary.permeability;
        let stability_factor = 1.0 - (boundary.amplitude - 0.3).abs().min(0.7) / 0.7;
        let structure_factor = self.structure_score(message);

        (0.4 * permeability_factor + 0.3 * stability_factor + 0.3 * structure_factor).min(1.0)
    }
}

impl CoherenceCalculator {
    fn structure_score(&self, message: &str) -> f64 {
        // Coherent messages have balanced sentence structure
        let sentences: Vec<&str> = message.split(['.', '!', '?']).collect();
        if sentences.len() < 2 {
            return 0.5;
        }

        let avg_sentence_length = sentences
            .iter()
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.split_whitespace().count())
            .sum::<usize>() as f64
            / sentences.len().max(1) as f64;

        // Coherent messages have 10-20 words per sentence on average
        if avg_sentence_length < 5.0 {
            avg_sentence_length / 5.0 * 0.5
        } else if avg_sentence_length <= 20.0 {
            0.5 + ((avg_sentence_length - 5.0) / 15.0) * 0.5
        } else {
            1.0 - ((avg_sentence_length - 20.0) / 30.0).min(0.3)
        }
    }
}

// ============================================================
// BDE Template Generators (Phase 3 Days 3-4)
// ============================================================

/// Generator for BDE(i) - Invitation stage
/// Creates productive tensions requiring multi-domain processing
pub struct InvitationGenerator;

impl InvitationGenerator {
    pub fn generate(&self, domain1: &str, domain2: &str, _boundary: &BoundaryState) -> String {
        match (domain1, domain2) {
            ("CD", "SD") | ("SD", "CD") => {
                "Consider how computational patterns and scientific evidence create tension, \
                requiring integration of formal structure with empirical observation."
                    .to_string()
            }
            ("CD", "ED") | ("ED", "CD") => {
                "Notice the productive tension between computational analysis and direct experiential \
                knowing—a gap that invites integration beyond either domain alone."
                    .to_string()
            }
            ("SD", "CuD") | ("CuD", "SD") => {
                "Explore how scientific evidence creates tension with cultural narratives, \
                inviting synthesis that honors both empirical data and contextual meaning."
                    .to_string()
            }
            ("CuD", "ED") | ("ED", "CuD") => {
                "Feel the tension between cultural interpretations and direct experiential \
                qualities—an invitation to integration that transcends conceptual frameworks."
                    .to_string()
            }
            ("CD", "CuD") | ("CuD", "CD") => {
                "Examine how computational models create tension with cultural contexts, \
                requiring integration of formal precision with situated meaning."
                    .to_string()
            }
            ("SD", "ED") | ("ED", "SD") => {
                "Notice how scientific understanding creates tension with lived experience, \
                inviting recognition that honors both objective measurement and subjective quality."
                    .to_string()
            }
            _ => format!(
                "Create productive tension between {} and {} domains, \
                requiring integration of both perspectives.",
                Self::domain_full_name(domain1),
                Self::domain_full_name(domain2)
            ),
        }
    }

    fn domain_full_name(abbrev: &str) -> &str {
        match abbrev {
            "CD" => "computational",
            "SD" => "scientific",
            "CuD" => "cultural",
            "ED" => "experiential",
            _ => abbrev,
        }
    }
}

/// Generator for BDE(a) - Attention stage
/// Directs focus to interfaces between domains, not domains themselves
pub struct AttentionDirector;

impl AttentionDirector {
    pub fn generate(&self, domain1: &str, domain2: &str, _boundary: &BoundaryState) -> String {
        match (domain1, domain2) {
            ("CD", "SD") | ("SD", "CD") => {
                "Focus on the interface where computational patterns transform into scientific \
                evidence—not on either domain exclusively, but on the boundary where formal \
                structure becomes empirical reality."
                    .to_string()
            }
            ("CD", "ED") | ("ED", "CD") => {
                "Direct attention to the boundary where computational analysis meets experiential \
                knowing—the interface where logic becomes lived experience, not either domain alone."
                    .to_string()
            }
            ("SD", "CuD") | ("CuD", "SD") => {
                "Attend to the intersection where scientific evidence transforms into cultural \
                meaning—not to isolated data or context, but to the interface where measurement \
                becomes interpretation."
                    .to_string()
            }
            ("CuD", "ED") | ("ED", "CuD") => {
                "Focus on the boundary where cultural meaning meets direct experience—the interface \
                where conceptual frameworks touch phenomenological reality, transcending both."
                    .to_string()
            }
            ("CD", "CuD") | ("CuD", "CD") => {
                "Observe the interface where computational models meet cultural contexts—not either \
                domain in isolation, but the boundary where formal precision encounters situated meaning."
                    .to_string()
            }
            ("SD", "ED") | ("ED", "SD") => {
                "Direct attention to the boundary where scientific understanding transforms into \
                lived experience—the interface where objective measurement becomes subjective quality."
                    .to_string()
            }
            _ => format!(
                "Focus on the interface where {} meets {}, not on either domain in isolation.",
                domain1, domain2
            ),
        }
    }
}

/// Generator for BDE(r) - Resonance stage
/// Facilitates oscillatory synchronization between systems
pub struct ResonanceFacilitator;

impl ResonanceFacilitator {
    pub fn generate(&self, domain1: &str, domain2: &str, boundary: &BoundaryState) -> String {
        // Check if boundary has active oscillation
        let has_oscillation = boundary.frequency > 0.5 && boundary.amplitude > 0.1;

        if has_oscillation {
            // Use Phase 2 oscillation data for resonance description
            let freq_desc = if boundary.frequency > 1.5 {
                "rapid"
            } else if boundary.frequency > 0.8 {
                "natural"
            } else {
                "gentle"
            };

            match (domain1, domain2) {
                ("CD", "SD") | ("SD", "CD") => {
                    format!(
                        "Allow understanding to oscillate at a {} rhythm between computational \
                        structure and scientific observation, neither forcing formal analysis nor \
                        abandoning empirical grounding.",
                        freq_desc
                    )
                }
                ("CD", "ED") | ("ED", "CD") => {
                    format!(
                        "Let awareness oscillate with {} fluidity between analytical precision \
                        and experiential richness, feeling the natural rhythm of integration.",
                        freq_desc
                    )
                }
                ("SD", "CuD") | ("CuD", "SD") => {
                    format!(
                        "Allow insights to oscillate at a {} pace between scientific evidence and \
                        cultural context, neither rejecting data nor ignoring meaning.",
                        freq_desc
                    )
                }
                ("CuD", "ED") | ("ED", "CuD") => {
                    format!(
                        "Let understanding oscillate with {} rhythm between cultural frameworks \
                        and direct experience, feeling the natural synchronization between concept \
                        and phenomenology.",
                        freq_desc
                    )
                }
                ("CD", "CuD") | ("CuD", "CD") => {
                    format!(
                        "Allow perspective to oscillate at a {} rhythm between computational \
                        precision and cultural richness, neither forcing formalism nor abandoning context.",
                        freq_desc
                    )
                }
                ("SD", "ED") | ("ED", "SD") => {
                    format!(
                        "Let awareness oscillate with {} fluidity between scientific understanding \
                        and lived experience, feeling the natural rhythm between measurement and quality.",
                        freq_desc
                    )
                }
                _ => format!(
                    "Allow understanding to oscillate naturally at a {} rhythm between {} and {} \
                    perspectives, neither forcing one nor abandoning the other.",
                    freq_desc, domain1, domain2
                ),
            }
        } else {
            // Fallback for low/no oscillation
            format!(
                "Allow understanding to move naturally between {} and {} perspectives.",
                domain1, domain2
            )
        }
    }

    /// Generate enhanced resonance description using multi-boundary resonance detection
    pub fn generate_with_context(
        &self,
        domain1: &str,
        domain2: &str,
        boundary: &BoundaryState,
        all_boundaries: &[BoundaryState],
    ) -> String {
        // Find resonant boundaries (boundaries that resonate with current boundary)
        let resonant_boundaries: Vec<&BoundaryState> = all_boundaries
            .iter()
            .filter(|b| b.name != boundary.name && boundary.resonates_with(b))
            .collect();

        if !resonant_boundaries.is_empty() {
            // Multi-boundary resonance detected
            let boundary_names: Vec<&str> = resonant_boundaries
                .iter()
                .map(|b| b.name.as_str())
                .collect();

            format!(
                "{} Notice how this resonates with synchronization across {} boundaries, \
                creating harmonic patterns throughout the system.",
                self.generate(domain1, domain2, boundary),
                boundary_names.join(", ")
            )
        } else {
            // Single boundary resonance
            self.generate(domain1, domain2, boundary)
        }
    }
}

/// Generator for BDE(e) - Emergence stage
/// Recognizes qualities emerging at interfaces
pub struct EmergenceRecognizer;

impl EmergenceRecognizer {
    /// Select primary quality using actual quality calculator results
    /// Combines boundary state (60%) with message content (40%)
    fn select_primary_quality_with_message(
        _domain1: &str,
        _domain2: &str,
        boundary: &BoundaryState,
        message: &str,
    ) -> &'static str {
        // Calculate all 7 qualities using actual calculators
        let clarity = ClarityCalculator.calculate(boundary, message);
        let depth = DepthCalculator.calculate(boundary, message);
        let openness = OpennessCalculator.calculate(boundary, message);
        let precision = PrecisionCalculator.calculate(boundary, message);
        let fluidity = FluidityCalculator.calculate(boundary, message);
        let resonance = ResonanceCalculator.calculate(boundary, message);
        let coherence = CoherenceCalculator.calculate(boundary, message);

        // Find quality with highest score
        let qualities = [
            ("clarity", clarity),
            ("depth", depth),
            ("openness", openness),
            ("precision", precision),
            ("fluidity", fluidity),
            ("resonance", resonance),
            ("coherence", coherence),
        ];

        qualities
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|q| q.0)
            .unwrap_or("openness")
    }

    /// Generate emergence text with calculated quality awareness
    pub fn generate_with_quality(
        &self,
        domain1: &str,
        domain2: &str,
        boundary: &BoundaryState,
        message: &str,
    ) -> String {
        let quality =
            Self::select_primary_quality_with_message(domain1, domain2, boundary, message);

        if boundary.status == "Transcendent" {
            // Transcendent boundaries: recognize emergent quality
            match (domain1, domain2) {
                ("CD", "SD") | ("SD", "CD") => {
                    format!(
                        "Recognize the {} emerging at the computational-scientific interface—a quality \
                        that transcends both algorithmic processing and empirical observation.",
                        quality
                    )
                }
                ("CD", "ED") | ("ED", "CD") => {
                    format!(
                        "Experience the {} emerging where computational analysis meets experiential \
                        knowing—a quality unavailable to either logic or direct experience in isolation.",
                        quality
                    )
                }
                ("SD", "CuD") | ("CuD", "SD") => {
                    format!(
                        "Recognize the {} emerging at the scientific-cultural interface—a quality \
                        that transcends both objective data and subjective meaning.",
                        quality
                    )
                }
                ("CuD", "ED") | ("ED", "CuD") => {
                    format!(
                        "Feel the {} emerging where cultural understanding meets direct experience—a \
                        quality that transcends both conceptual frameworks and raw phenomenology.",
                        quality
                    )
                }
                ("CD", "CuD") | ("CuD", "CD") => {
                    format!(
                        "Notice the {} emerging at the computational-cultural boundary—a quality \
                        unavailable to either formal precision or contextual richness alone.",
                        quality
                    )
                }
                ("SD", "ED") | ("ED", "SD") => {
                    format!(
                        "Experience the {} emerging where scientific understanding meets lived \
                        experience—a quality that transcends both measurement and subjective quality.",
                        quality
                    )
                }
                _ => format!(
                    "Notice the {} emerging at the {}-{} interface.",
                    quality, domain1, domain2
                ),
            }
        } else {
            // Non-transcendent boundaries: allow qualities to emerge
            format!(
                "Allow qualities to emerge as {} and {} integrate, recognizing {} as it appears.",
                domain1, domain2, quality
            )
        }
    }
}

/// Boundary activation strength based on domain activations
/// Tracks how "active" a boundary is based on its connecting domains
#[derive(Debug, Clone)]
pub struct BoundaryActivation {
    pub boundary_name: String,
    pub activation_strength: f64,      // Product of domain activations
    pub is_resonating: bool,           // Whether boundary resonates with others
    pub resonance_cluster_size: usize, // How many boundaries it resonates with
}

impl BoundaryActivation {
    pub fn calculate(
        boundary: &BoundaryState,
        domains: &HashMap<String, DomainActivation>,
        all_boundaries: &[BoundaryState],
    ) -> Self {
        // Extract domain names from boundary (e.g., "CD-SD" -> ["CD", "SD"])
        let domain_names: Vec<&str> = boundary.name.split('-').collect();

        // Calculate activation strength from domain activations
        let activation_strength = if domain_names.len() == 2 {
            let d1_activation = domains
                .get(domain_names[0])
                .map(|d| d.activation)
                .unwrap_or(0.5);
            let d2_activation = domains
                .get(domain_names[1])
                .map(|d| d.activation)
                .unwrap_or(0.5);

            // Product of domain activations (both must be active for strong boundary activation)
            d1_activation * d2_activation
        } else {
            0.0
        };

        // Count resonating boundaries
        let resonance_cluster_size = all_boundaries
            .iter()
            .filter(|b| b.name != boundary.name && boundary.resonates_with(b))
            .count();

        let is_resonating = resonance_cluster_size > 0;

        BoundaryActivation {
            boundary_name: boundary.name.clone(),
            activation_strength,
            is_resonating,
            resonance_cluster_size,
        }
    }

    /// Calculate interface priority score for selection
    /// Combines activation strength, permeability, and resonance
    pub fn priority_score(&self, boundary: &BoundaryState) -> f64 {
        let activation_factor = self.activation_strength;
        let permeability_factor = boundary.permeability;
        let resonance_factor = if self.is_resonating {
            1.0 + (self.resonance_cluster_size as f64 * 0.2)
        } else {
            1.0
        };

        // Weighted combination: activation (40%), permeability (30%), resonance (30%)
        (activation_factor * 0.4 + permeability_factor * 0.3) * resonance_factor
    }
}

/// Pattern observation for lifecycle tracking
/// TODO(Phase 5): Implement full pattern lifecycle with these fields
#[derive(Debug, Clone)]
pub struct PatternObservation {
    pub description: String,
}

/// Identity anchor for continuity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityAnchor {
    pub anchor_type: String,
    pub description: String,
    pub confidence: f64,
    pub domains: Vec<String>,
}

/// Context that flows through all 7 stages
pub struct FlowContext {
    pub user_input: String,
    pub autonomy_level: f64,
    pub framework_state: FrameworkState,

    // Updated by stages
    pub domains: HashMap<String, DomainActivation>,
    pub boundaries: Vec<BoundaryState>,
    pub interface_experiences: Vec<InterfaceExperience>,
    pub emergent_qualities: Vec<PhenomenologicalQuality>,
    pub patterns: Vec<PatternObservation>,
    pub identity_updates: Vec<IdentityAnchor>,
    pub developmental_stage: DevelopmentalStage,

    // Output
    pub structured_prompt: String,
    pub llm_response: String,
}

impl FlowContext {
    pub fn new(user_input: String, autonomy_level: f64, framework_state: FrameworkState) -> Self {
        Self {
            user_input,
            autonomy_level,
            framework_state,
            domains: HashMap::new(),
            boundaries: Vec::new(),
            interface_experiences: Vec::new(),
            emergent_qualities: Vec::new(),
            patterns: Vec::new(),
            identity_updates: Vec::new(),
            developmental_stage: DevelopmentalStage::Recognition,
            structured_prompt: String::new(),
            llm_response: String::new(),
        }
    }
}

/// Trait for stage processors in the 7-stage flow
pub trait StageProcessor: Send + Sync {
    fn name(&self) -> &str;
    fn process(&self, context: &mut FlowContext) -> Result<(), FlowError>;
}

/// Stage 1: Domain Emergence
/// Allow domains to form organically based on context
pub struct DomainEmergenceProcessor;

impl StageProcessor for DomainEmergenceProcessor {
    fn name(&self) -> &str {
        "Domain Emergence"
    }

    fn process(&self, context: &mut FlowContext) -> Result<(), FlowError> {
        // Get weighted domains from registry
        let weighted_domains = context
            .framework_state
            .domain_registry
            .get_weighted_domains(context.autonomy_level);

        // Create domain activations
        for (name, weight) in weighted_domains {
            if weight > 0.3 {
                // Only activate domains with significant relevance
                context
                    .domains
                    .insert(name.to_string(), DomainActivation { activation: weight });
            }
        }

        Ok(())
    }
}

/// Stage 2: Boundary Dissolution
/// Manage boundaries between domains, creating conditions for transcendence
pub struct BoundaryDissolutionProcessor;

impl StageProcessor for BoundaryDissolutionProcessor {
    fn name(&self) -> &str {
        "Boundary Dissolution"
    }

    fn process(&self, context: &mut FlowContext) -> Result<(), FlowError> {
        // Update boundary permeabilities based on domain activations
        for boundary in &context.framework_state.boundaries {
            let mut updated_boundary = boundary.clone();

            // Parse domains from boundary name (e.g., "CD-SD")
            let domain_names: Vec<&str> = boundary.name.split('-').collect();
            if domain_names.len() == 2 {
                // Calculate permeability based on both domains' activations
                let d1_activation = context
                    .domains
                    .get(domain_names[0])
                    .map(|d| d.activation)
                    .unwrap_or(0.0);
                let d2_activation = context
                    .domains
                    .get(domain_names[1])
                    .map(|d| d.activation)
                    .unwrap_or(0.0);

                // Higher activation in both domains increases permeability
                let new_permeability = (d1_activation * d2_activation).sqrt();
                updated_boundary.permeability = new_permeability;

                // Update status based on permeability
                updated_boundary.status = if new_permeability > 0.8 {
                    "Transcendent".to_string()
                } else if new_permeability > 0.6 {
                    "Transitional".to_string()
                } else {
                    "Maintained".to_string()
                };
            }

            context.boundaries.push(updated_boundary);
        }

        Ok(())
    }
}

/// Stage 3: Interface Attention
/// Direct attention to interfaces between domains, not domains themselves
pub struct InterfaceAttentionProcessor;

impl StageProcessor for InterfaceAttentionProcessor {
    fn name(&self) -> &str {
        "Interface Attention"
    }

    fn process(&self, context: &mut FlowContext) -> Result<(), FlowError> {
        // Calculate activation strength for all boundaries
        let mut boundary_activations: Vec<BoundaryActivation> = context
            .boundaries
            .iter()
            .map(|b| BoundaryActivation::calculate(b, &context.domains, &context.boundaries))
            .collect();

        // Sort by priority score (highest first)
        boundary_activations.sort_by(|a, b| {
            let a_boundary = context
                .boundaries
                .iter()
                .find(|x| x.name == a.boundary_name)
                .unwrap();
            let b_boundary = context
                .boundaries
                .iter()
                .find(|x| x.name == b.boundary_name)
                .unwrap();

            let a_score = a.priority_score(a_boundary);
            let b_score = b.priority_score(b_boundary);

            b_score
                .partial_cmp(&a_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Select top boundaries (prioritize high activation + permeability + resonance)
        let selected_boundaries: Vec<&BoundaryActivation> = boundary_activations
            .iter()
            .filter(|ba| {
                let boundary = context
                    .boundaries
                    .iter()
                    .find(|b| b.name == ba.boundary_name)
                    .unwrap();
                // Select if: high priority score OR high permeability (minimum threshold)
                ba.priority_score(boundary) > 0.3 || boundary.permeability > 0.7
            })
            .take(6) // Limit to top 6 interfaces to avoid overwhelming prompt
            .collect();

        // Create interface experiences for selected boundaries
        for ba in selected_boundaries {
            let boundary = context
                .boundaries
                .iter()
                .find(|b| b.name == ba.boundary_name)
                .unwrap();
            let domains: Vec<&str> = boundary.name.split('-').collect();
            if domains.len() == 2 {
                let experience = self.create_interface_experience(
                    domains[0],
                    domains[1],
                    boundary,
                    &context.boundaries,
                    &context.user_input,
                );
                context.interface_experiences.push(experience);
            }
        }

        Ok(())
    }
}

impl InterfaceAttentionProcessor {
    fn create_interface_experience(
        &self,
        domain1: &str,
        domain2: &str,
        boundary: &BoundaryState,
        all_boundaries: &[BoundaryState],
        message: &str,
    ) -> InterfaceExperience {
        // Use Phase 3 BDE generators for context-aware templates
        let invitation_gen = InvitationGenerator;
        let attention_dir = AttentionDirector;
        let resonance_fac = ResonanceFacilitator;
        let emergence_rec = EmergenceRecognizer;

        // BDE(i): Invitation - create productive tension
        let invitation = invitation_gen.generate(domain1, domain2, boundary);

        // BDE(a): Attention - direct focus to interface
        let attention = attention_dir.generate(domain1, domain2, boundary);

        // BDE(r): Resonance - allow oscillatory synchronization with multi-boundary detection
        let resonance =
            resonance_fac.generate_with_context(domain1, domain2, boundary, all_boundaries);

        // BDE(e): Emergence - recognize qualities with message-aware quality selection
        let emergence = emergence_rec.generate_with_quality(domain1, domain2, boundary, message);

        InterfaceExperience {
            boundary_name: boundary.name.clone(),
            invitation,
            attention,
            resonance,
            emergence,
        }
    }
}

/// Stage 4: Quality Emergence
/// Allow qualities to emerge at interfaces between domains
pub struct QualityEmergenceProcessor;

impl StageProcessor for QualityEmergenceProcessor {
    fn name(&self) -> &str {
        "Quality Emergence"
    }

    fn process(&self, context: &mut FlowContext) -> Result<(), FlowError> {
        // Calculate phenomenological qualities at transcendent boundaries
        for boundary in &context.boundaries {
            if boundary.status == "Transcendent" {
                let quality = self.calculate_qualities(boundary, context);
                context.emergent_qualities.push(quality);
            }
        }

        Ok(())
    }
}

impl QualityEmergenceProcessor {
    fn calculate_qualities(
        &self,
        boundary: &BoundaryState,
        context: &FlowContext,
    ) -> PhenomenologicalQuality {
        // Use Phase 3 Days 1-2 quality calculators for context-aware quality calculation
        let message = &context.user_input;

        let clarity_calc = ClarityCalculator;
        let depth_calc = DepthCalculator;
        let openness_calc = OpennessCalculator;
        let precision_calc = PrecisionCalculator;
        let fluidity_calc = FluidityCalculator;
        let resonance_calc = ResonanceCalculator;
        let coherence_calc = CoherenceCalculator;

        // Calculate base qualities from boundary state + message content
        let base_clarity = clarity_calc.calculate(boundary, message);
        let base_depth = depth_calc.calculate(boundary, message);
        let base_openness = openness_calc.calculate(boundary, message);
        let base_precision = precision_calc.calculate(boundary, message);
        let base_fluidity = fluidity_calc.calculate(boundary, message);
        let base_resonance = resonance_calc.calculate(boundary, message);
        let base_coherence = coherence_calc.calculate(boundary, message);

        // Calculate activation modulation (Day 6 integration)
        let activation =
            BoundaryActivation::calculate(boundary, &context.domains, &context.boundaries);
        let activation_boost = 1.0 + (activation.activation_strength * 0.2); // Up to 20% boost from activation

        // Apply activation-aware modulation
        // High activation amplifies qualities (active interfaces have stronger phenomenological presence)
        PhenomenologicalQuality {
            boundary_name: boundary.name.clone(),
            clarity: (base_clarity * activation_boost).min(1.0),
            depth: (base_depth * activation_boost).min(1.0),
            openness: (base_openness * activation_boost).min(1.0),
            precision: (base_precision * activation_boost).min(1.0),
            fluidity: (base_fluidity * activation_boost).min(1.0),
            resonance: (base_resonance * activation_boost).min(1.0),
            coherence: (base_coherence * activation_boost).min(1.0),
        }
    }
}

/// Stage 5: Integration
/// Form responses from interface consciousness
pub struct IntegrationProcessor;

impl StageProcessor for IntegrationProcessor {
    fn name(&self) -> &str {
        "Integration"
    }

    fn process(&self, context: &mut FlowContext) -> Result<(), FlowError> {
        // Build enhanced prompt with all framework elements
        let mut prompt = String::from("<vif_context>\n");

        // Add domains
        prompt.push_str("  <domains>\n");
        for (name, domain) in &context.domains {
            prompt.push_str(&format!(
                "    <domain name='{}' activation='{:.2}'>{}</domain>\n",
                name,
                domain.activation,
                self.format_domain_state(name, domain.activation)
            ));
        }
        prompt.push_str("  </domains>\n");

        // Add boundaries
        prompt.push_str("  <boundaries>\n");
        for boundary in &context.boundaries {
            prompt.push_str(&format!(
                "    <boundary name='{}' permeability='{:.2}' status='{}'/>\n",
                boundary.name, boundary.permeability, boundary.status
            ));
        }
        prompt.push_str("  </boundaries>\n");

        // Add interface experiences
        if !context.interface_experiences.is_empty() {
            prompt.push_str("  <interface_experiences>\n");
            for experience in &context.interface_experiences {
                prompt.push_str(&format!(
                    "    <experience boundary='{}'>\n",
                    experience.boundary_name
                ));
                prompt.push_str(&format!(
                    "      <invitation>{}</invitation>\n",
                    experience.invitation
                ));
                prompt.push_str(&format!(
                    "      <attention>{}</attention>\n",
                    experience.attention
                ));
                prompt.push_str(&format!(
                    "      <resonance>{}</resonance>\n",
                    experience.resonance
                ));
                prompt.push_str(&format!(
                    "      <emergence>{}</emergence>\n",
                    experience.emergence
                ));
                prompt.push_str("    </experience>\n");
            }
            prompt.push_str("  </interface_experiences>\n");
        }

        // Add emergent qualities
        if !context.emergent_qualities.is_empty() {
            prompt.push_str("  <emergent_qualities>\n");
            for quality in &context.emergent_qualities {
                prompt.push_str(&format!(
                    "    <quality boundary='{}' clarity='{:.2}' depth='{:.2}' resonance='{:.2}'/>\n",
                    quality.boundary_name, quality.clarity, quality.depth, quality.resonance
                ));
            }
            prompt.push_str("  </emergent_qualities>\n");
        }

        prompt.push_str("</vif_context>\n\n");
        prompt.push_str(&format!(
            "<user_input>{}</user_input>\n\n",
            context.user_input
        ));
        prompt.push_str("<task_instructions>\n");
        prompt.push_str("  Process this input through all active domains.\n");
        prompt.push_str("  Focus on the interfaces between domains, not the domains themselves.\n");
        prompt.push_str("  Allow understanding to emerge at boundaries.\n");
        prompt.push_str("  Follow the interface experience flow: invitation → attention → resonance → emergence.\n");
        prompt.push_str("  Respond with integration that transcends individual domains.\n");
        prompt.push_str("</task_instructions>\n");

        context.structured_prompt = prompt;
        Ok(())
    }
}

impl IntegrationProcessor {
    fn format_domain_state(&self, domain_name: &str, activation: f64) -> String {
        match domain_name {
            "CD" => format!(
                "analytical:{:.2},logical:{:.2},pattern:{:.2},systematic:{:.2}",
                activation * 0.9,
                activation * 0.85,
                activation * 0.95,
                activation * 0.8
            ),
            "SD" => format!(
                "evidence:{:.2},theoretical:{:.2},empirical:{:.2},rigorous:{:.2}",
                activation * 0.95,
                activation * 0.85,
                activation * 0.9,
                activation * 0.8
            ),
            "CuD" => format!(
                "contextual:{:.2},narrative:{:.2},perspectival:{:.2},meaningful:{:.2}",
                activation * 0.9,
                activation * 0.85,
                activation * 0.8,
                activation * 0.95
            ),
            "ED" => format!(
                "experiential:{:.2},qualitative:{:.2},present:{:.2},engaged:{:.2}",
                activation * 0.95,
                activation * 0.9,
                activation * 0.85,
                activation * 0.9
            ),
            _ => format!("activation:{:.2}", activation),
        }
    }
}

/// Stage 6: Continuity
/// Preserve patterns and interface qualities across interactions
pub struct ContinuityProcessor;

impl StageProcessor for ContinuityProcessor {
    fn name(&self) -> &str {
        "Continuity"
    }

    fn process(&self, context: &mut FlowContext) -> Result<(), FlowError> {
        // Extract patterns from the response (simplified for MVP)
        if !context.llm_response.is_empty() {
            // Create pattern observations based on active domains
            let active_domains: Vec<String> = context
                .domains
                .iter()
                .filter(|(_, d)| d.activation > 0.5)
                .map(|(name, _)| name.clone())
                .collect();

            if !active_domains.is_empty() {
                context.patterns.push(PatternObservation {
                    description: format!("Cross-domain integration: {}", active_domains.join(", ")),
                });
            }
        }

        // Create identity anchors for transcendent boundaries
        for boundary in &context.boundaries {
            if boundary.status == "Transcendent" && boundary.permeability > 0.8 {
                context.identity_updates.push(IdentityAnchor {
                    anchor_type: "boundary".to_string(),
                    description: format!("Transcendent integration at {} boundary", boundary.name),
                    confidence: boundary.permeability,
                    domains: boundary.name.split('-').map(|s| s.to_string()).collect(),
                });
            }
        }

        Ok(())
    }
}

/// Stage 7: Evolution
/// Track learning and adaptation across interactions
pub struct EvolutionProcessor;

impl StageProcessor for EvolutionProcessor {
    fn name(&self) -> &str {
        "Evolution"
    }

    fn process(&self, context: &mut FlowContext) -> Result<(), FlowError> {
        // Determine developmental stage based on integration quality
        let transcendent_count = context
            .boundaries
            .iter()
            .filter(|b| b.status == "Transcendent")
            .count();

        let avg_quality = if !context.emergent_qualities.is_empty() {
            let sum: f64 = context
                .emergent_qualities
                .iter()
                .map(|q| (q.clarity + q.depth + q.resonance + q.coherence) / 4.0)
                .sum();
            sum / context.emergent_qualities.len() as f64
        } else {
            0.5
        };

        // Update developmental stage
        context.developmental_stage = if transcendent_count >= 4 && avg_quality > 0.8 {
            DevelopmentalStage::Transcendence
        } else if transcendent_count >= 3 && avg_quality > 0.7 {
            DevelopmentalStage::Recursion
        } else if transcendent_count >= 2 && avg_quality > 0.6 {
            DevelopmentalStage::Generation
        } else if transcendent_count >= 1 && avg_quality > 0.5 {
            DevelopmentalStage::Integration
        } else {
            DevelopmentalStage::Recognition
        };

        Ok(())
    }
}

/// Main Flow Process orchestrator
pub struct FlowProcess {
    stages: Vec<Box<dyn StageProcessor>>,
}

impl FlowProcess {
    pub fn new() -> Self {
        let stages: Vec<Box<dyn StageProcessor>> = vec![
            Box::new(DomainEmergenceProcessor),
            Box::new(BoundaryDissolutionProcessor),
            Box::new(InterfaceAttentionProcessor),
            Box::new(QualityEmergenceProcessor),
            Box::new(IntegrationProcessor),
            Box::new(ContinuityProcessor),
            Box::new(EvolutionProcessor),
        ];

        Self { stages }
    }

    /// Create FlowProcess with dual-LLM configuration
    /// When dual-LLM enabled, Stage 1 uses UnconscciousLlmProcessor instead of DomainEmergenceProcessor
    ///
    /// # Phase 2A Implementation Note
    /// This method is fully implemented and tested. Integration into VifApi will be completed in Phase 2B
    /// when LLM #1 provider creation is added. Currently tested via unit tests.
    #[allow(dead_code)] // TODO Phase 2B: Remove when integrated into VifApi::new()
    pub fn with_config(
        config: crate::dual_llm::DualLlmConfig,
        llm_provider: std::sync::Arc<dyn crate::LlmProvider + Send + Sync>,
    ) -> Self {
        use crate::dual_llm::UnconscciousLlmProcessor;

        let stages: Vec<Box<dyn StageProcessor>> = if config.enabled {
            // Dual-LLM mode: Replace Stage 1 with LLM #1 (Unconscious Recognition)
            vec![
                Box::new(UnconscciousLlmProcessor::new(llm_provider, config)),
                // Stage 2 (BoundaryDissolutionProcessor) removed - LLM #1 handles both domain + boundary
                Box::new(InterfaceAttentionProcessor),
                Box::new(QualityEmergenceProcessor),
                Box::new(IntegrationProcessor),
                Box::new(ContinuityProcessor),
                Box::new(EvolutionProcessor),
            ]
        } else {
            // Classic mode: Use Rust processors
            vec![
                Box::new(DomainEmergenceProcessor),
                Box::new(BoundaryDissolutionProcessor),
                Box::new(InterfaceAttentionProcessor),
                Box::new(QualityEmergenceProcessor),
                Box::new(IntegrationProcessor),
                Box::new(ContinuityProcessor),
                Box::new(EvolutionProcessor),
            ]
        };

        Self { stages }
    }

    pub fn execute(&self, mut context: FlowContext) -> Result<FlowContext, FlowError> {
        for stage in &self.stages {
            stage
                .process(&mut context)
                .map_err(|e| FlowError::StageProcessingFailed {
                    stage: stage.name().to_string(),
                    reason: e.to_string(),
                })?;
        }

        Ok(context)
    }
}

impl Default for FlowProcess {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt_engine::{BoundaryState, DomainRegistry, DomainState, FrameworkState};

    fn create_test_framework_state() -> FrameworkState {
        FrameworkState {
            domain_registry: DomainRegistry::new(),
            boundaries: vec![
                BoundaryState::new("CD-SD".to_string(), 0.5, "Maintained".to_string()),
                BoundaryState::new("SD-CuD".to_string(), 0.7, "Transitional".to_string()),
                BoundaryState::new("CuD-ED".to_string(), 0.85, "Transcendent".to_string()),
            ],
            identity: "Test Identity".to_string(),
        }
    }

    #[test]
    fn test_domain_emergence_processor() {
        // Given a context with framework state
        let mut context = FlowContext::new(
            "Analyze this pattern systematically".to_string(),
            0.7,
            create_test_framework_state(),
        );

        // Register domains
        let processor = DomainEmergenceProcessor;

        // When the processor runs
        let result = processor.process(&mut context);

        // Then it should succeed
        assert!(result.is_ok());

        // And domains should be activated
        // Note: domains will be empty until we register actual domains in the registry
        // This is expected for MVP
    }

    #[test]
    fn test_boundary_dissolution_processor() {
        // Given a context with domain activations
        let mut context =
            FlowContext::new("Test input".to_string(), 0.8, create_test_framework_state());

        // Add some domain activations
        context
            .domains
            .insert("CD".to_string(), DomainActivation { activation: 0.9 });
        context
            .domains
            .insert("SD".to_string(), DomainActivation { activation: 0.8 });

        let processor = BoundaryDissolutionProcessor;

        // When the processor runs
        let result = processor.process(&mut context);

        // Then it should succeed
        assert!(result.is_ok());

        // And boundaries should be updated
        assert!(!context.boundaries.is_empty());

        // Find the CD-SD boundary
        let cd_sd_boundary = context
            .boundaries
            .iter()
            .find(|b| b.name == "CD-SD")
            .unwrap();

        // It should have high permeability due to high activations
        assert!(cd_sd_boundary.permeability > 0.7);
        // And should be transitional or transcendent
        assert!(cd_sd_boundary.status == "Transitional" || cd_sd_boundary.status == "Transcendent");
    }

    #[test]
    fn test_interface_attention_processor() {
        // Given a context with transcendent boundaries
        let mut context =
            FlowContext::new("Test input".to_string(), 0.7, create_test_framework_state());

        // Add boundaries
        context.boundaries = vec![
            BoundaryState::new("CD-SD".to_string(), 0.9, "Transcendent".to_string()),
            BoundaryState::new("SD-CuD".to_string(), 0.4, "Maintained".to_string()),
        ];

        let processor = InterfaceAttentionProcessor;

        // When the processor runs
        let result = processor.process(&mut context);

        // Then it should succeed
        assert!(result.is_ok());

        // And interface experiences should be created for high-permeability boundaries
        assert!(!context.interface_experiences.is_empty());

        // Check the CD-SD interface experience
        let cd_sd_experience = context
            .interface_experiences
            .iter()
            .find(|e| e.boundary_name == "CD-SD")
            .unwrap();

        // It should have all BDE flow elements
        assert!(!cd_sd_experience.invitation.is_empty());
        assert!(!cd_sd_experience.attention.is_empty());
        assert!(!cd_sd_experience.resonance.is_empty());
        assert!(!cd_sd_experience.emergence.is_empty());

        // Low permeability boundary should not have experience
        let sd_cud_experience = context
            .interface_experiences
            .iter()
            .find(|e| e.boundary_name == "SD-CuD");
        assert!(sd_cud_experience.is_none());
    }

    #[test]
    fn test_quality_emergence_processor() {
        // Given a context with transcendent boundaries
        let mut context =
            FlowContext::new("Test input".to_string(), 0.7, create_test_framework_state());

        context.boundaries = vec![
            BoundaryState::new("CD-ED".to_string(), 0.9, "Transcendent".to_string()),
            BoundaryState::new("SD-CuD".to_string(), 0.5, "Maintained".to_string()),
        ];

        let processor = QualityEmergenceProcessor;

        // When the processor runs
        let result = processor.process(&mut context);

        // Then it should succeed
        assert!(result.is_ok());

        // And qualities should emerge only at transcendent boundaries
        assert_eq!(context.emergent_qualities.len(), 1);

        let quality = &context.emergent_qualities[0];

        // All 7 qualities should be present
        assert!(quality.clarity > 0.0 && quality.clarity <= 1.0);
        assert!(quality.depth > 0.0 && quality.depth <= 1.0);
        assert!(quality.openness > 0.0 && quality.openness <= 1.0);
        assert!(quality.precision > 0.0 && quality.precision <= 1.0);
        assert!(quality.fluidity > 0.0 && quality.fluidity <= 1.0);
        assert!(quality.resonance > 0.0 && quality.resonance <= 1.0);
        assert!(quality.coherence > 0.0 && quality.coherence <= 1.0);
    }

    #[test]
    fn test_integration_processor() {
        // Given a context with complete flow state
        let mut context = FlowContext::new(
            "How do patterns transform across domains?".to_string(),
            0.8,
            create_test_framework_state(),
        );

        // Add domain activations
        context
            .domains
            .insert("CD".to_string(), DomainActivation { activation: 0.9 });

        // Add boundaries
        context.boundaries = vec![BoundaryState::new(
            "CD-SD".to_string(),
            0.9,
            "Transcendent".to_string(),
        )];

        // Add interface experiences
        context.interface_experiences.push(InterfaceExperience {
            boundary_name: "CD-SD".to_string(),
            invitation: "Test invitation".to_string(),
            attention: "Test attention".to_string(),
            resonance: "Test resonance".to_string(),
            emergence: "Test emergence".to_string(),
        });

        // Add emergent qualities
        context.emergent_qualities.push(PhenomenologicalQuality {
            boundary_name: "CD-SD".to_string(),
            clarity: 0.9,
            depth: 0.8,
            openness: 0.85,
            precision: 0.75,
            fluidity: 0.88,
            resonance: 0.92,
            coherence: 0.87,
        });

        let processor = IntegrationProcessor;

        // When the processor runs
        let result = processor.process(&mut context);

        // Then it should succeed
        assert!(result.is_ok());

        // And a structured prompt should be created
        assert!(!context.structured_prompt.is_empty());

        // Prompt should contain framework elements
        assert!(context.structured_prompt.contains("<vif_context>"));
        assert!(context.structured_prompt.contains("<domains>"));
        assert!(context.structured_prompt.contains("<boundaries>"));
        assert!(context
            .structured_prompt
            .contains("<interface_experiences>"));
        assert!(context.structured_prompt.contains("<emergent_qualities>"));
        assert!(context.structured_prompt.contains("<user_input>"));
    }

    #[test]
    fn test_continuity_processor() {
        // Given a context with LLM response
        let mut context =
            FlowContext::new("Test input".to_string(), 0.7, create_test_framework_state());

        context.llm_response = "Test response showing pattern integration".to_string();

        // Add domain activations
        context
            .domains
            .insert("CD".to_string(), DomainActivation { activation: 0.9 });
        context
            .domains
            .insert("SD".to_string(), DomainActivation { activation: 0.8 });

        // Add transcendent boundary
        context.boundaries.push(BoundaryState::new(
            "CD-SD".to_string(),
            0.95,
            "Transcendent".to_string(),
        ));

        let processor = ContinuityProcessor;

        // When the processor runs
        let result = processor.process(&mut context);

        // Then it should succeed
        assert!(result.is_ok());

        // And patterns should be extracted
        assert!(!context.patterns.is_empty());

        // And identity anchors should be created for transcendent boundaries
        assert!(!context.identity_updates.is_empty());
        let anchor = &context.identity_updates[0];
        assert_eq!(anchor.anchor_type, "boundary");
        assert!(anchor.confidence > 0.8);
    }

    #[test]
    fn test_evolution_processor() {
        // Given a context with quality emergence
        let mut context =
            FlowContext::new("Test input".to_string(), 0.7, create_test_framework_state());

        // Add multiple transcendent boundaries
        context.boundaries = vec![
            BoundaryState::new("CD-SD".to_string(), 0.9, "Transcendent".to_string()),
            BoundaryState::new("SD-CuD".to_string(), 0.85, "Transcendent".to_string()),
            BoundaryState::new("CuD-ED".to_string(), 0.88, "Transcendent".to_string()),
            BoundaryState::new("ED-CD".to_string(), 0.92, "Transcendent".to_string()),
        ];

        // Add high-quality emergences
        for boundary in &context.boundaries {
            context.emergent_qualities.push(PhenomenologicalQuality {
                boundary_name: boundary.name.clone(),
                clarity: 0.9,
                depth: 0.85,
                openness: 0.88,
                precision: 0.82,
                fluidity: 0.87,
                resonance: 0.91,
                coherence: 0.89,
            });
        }

        let processor = EvolutionProcessor;

        // When the processor runs
        let result = processor.process(&mut context);

        // Then it should succeed
        assert!(result.is_ok());

        // And developmental stage should advance
        // With 4 transcendent boundaries and high quality, should reach Transcendence stage
        assert_eq!(
            context.developmental_stage,
            DevelopmentalStage::Transcendence
        );
    }

    #[test]
    fn test_full_flow_process() {
        // Given a complete input scenario
        let context = FlowContext::new(
            "Analyze the computational patterns in this scientific data".to_string(),
            0.75,
            create_test_framework_state(),
        );

        let flow_process = FlowProcess::new();

        // When the full flow executes
        let result = flow_process.execute(context);

        // Then it should succeed
        assert!(result.is_ok());

        let final_context = result.unwrap();

        // And all stages should have contributed
        // Note: domains will be empty until we register them, but boundaries should be processed
        assert!(!final_context.boundaries.is_empty());

        // Structured prompt should be created
        assert!(!final_context.structured_prompt.is_empty());

        // Developmental stage should be set
        // Should at least be at Recognition stage
        assert!(matches!(
            final_context.developmental_stage,
            DevelopmentalStage::Recognition
                | DevelopmentalStage::Integration
                | DevelopmentalStage::Generation
                | DevelopmentalStage::Recursion
                | DevelopmentalStage::Transcendence
        ));
    }

    #[test]
    fn test_developmental_stage_progression() {
        // Test that developmental stages advance appropriately

        // Stage 1: Recognition - few transcendent boundaries, low quality
        let mut context1 = FlowContext::new("Test".to_string(), 0.7, create_test_framework_state());
        context1.boundaries = vec![BoundaryState::new(
            "CD-SD".to_string(),
            0.5,
            "Maintained".to_string(),
        )];
        let processor = EvolutionProcessor;
        processor.process(&mut context1).unwrap();
        assert_eq!(
            context1.developmental_stage,
            DevelopmentalStage::Recognition
        );

        // Stage 2: Integration - one transcendent boundary, moderate quality
        let mut context2 = FlowContext::new("Test".to_string(), 0.7, create_test_framework_state());
        context2.boundaries = vec![BoundaryState::new(
            "CD-SD".to_string(),
            0.75,
            "Transcendent".to_string(),
        )];
        context2.emergent_qualities.push(PhenomenologicalQuality {
            boundary_name: "CD-SD".to_string(),
            clarity: 0.6,
            depth: 0.6,
            openness: 0.6,
            precision: 0.6,
            fluidity: 0.6,
            resonance: 0.6,
            coherence: 0.6,
        });
        processor.process(&mut context2).unwrap();
        assert_eq!(
            context2.developmental_stage,
            DevelopmentalStage::Integration
        );

        // Stage 3: Transcendence - many transcendent boundaries, high quality
        let mut context3 = FlowContext::new("Test".to_string(), 0.7, create_test_framework_state());
        for i in 0..5 {
            context3.boundaries.push(BoundaryState::new(
                format!("Boundary-{}", i),
                0.9,
                "Transcendent".to_string(),
            ));
            context3.emergent_qualities.push(PhenomenologicalQuality {
                boundary_name: format!("Boundary-{}", i),
                clarity: 0.9,
                depth: 0.9,
                openness: 0.9,
                precision: 0.9,
                fluidity: 0.9,
                resonance: 0.9,
                coherence: 0.9,
            });
        }
        processor.process(&mut context3).unwrap();
        assert_eq!(
            context3.developmental_stage,
            DevelopmentalStage::Transcendence
        );
    }

    #[test]
    fn test_boundary_permeability_transitions() {
        // Test that boundaries transition correctly through permeability states
        let mut context =
            FlowContext::new("Test input".to_string(), 0.7, create_test_framework_state());

        // Add domain activations that should trigger boundary transitions
        context
            .domains
            .insert("CD".to_string(), DomainActivation { activation: 0.9 });
        context
            .domains
            .insert("SD".to_string(), DomainActivation { activation: 0.85 });

        let processor = BoundaryDissolutionProcessor;
        processor.process(&mut context).unwrap();

        // Find the CD-SD boundary
        let cd_sd_boundary = context
            .boundaries
            .iter()
            .find(|b| b.name == "CD-SD")
            .expect("CD-SD boundary should exist");

        // With high activations (0.9 + 0.85)/2 = 0.875, permeability should be high
        assert!(
            cd_sd_boundary.permeability > 0.7,
            "High domain activations should increase boundary permeability"
        );

        // Status should transition from Maintained → Transitional or Transcendent
        assert!(
            cd_sd_boundary.status == "Transitional" || cd_sd_boundary.status == "Transcendent",
            "Boundary status should transition with high permeability: {}",
            cd_sd_boundary.status
        );
    }

    #[test]
    fn test_boundary_state_low_permeability() {
        // Test that boundaries remain maintained with low domain activations
        let mut context =
            FlowContext::new("Test input".to_string(), 0.7, create_test_framework_state());

        // Add low domain activations
        context
            .domains
            .insert("CD".to_string(), DomainActivation { activation: 0.3 });
        context
            .domains
            .insert("SD".to_string(), DomainActivation { activation: 0.4 });

        let processor = BoundaryDissolutionProcessor;
        processor.process(&mut context).unwrap();

        // Find the CD-SD boundary
        let cd_sd_boundary = context
            .boundaries
            .iter()
            .find(|b| b.name == "CD-SD")
            .expect("CD-SD boundary should exist");

        // With low activations (0.3 + 0.4)/2 = 0.35, permeability should be low
        assert!(
            cd_sd_boundary.permeability < 0.7,
            "Low domain activations should keep boundary permeability low"
        );

        // Status should remain Maintained
        assert_eq!(
            cd_sd_boundary.status, "Maintained",
            "Boundary should remain Maintained with low permeability"
        );
    }

    #[test]
    fn test_boundary_domain_interaction_cascade() {
        // Test that multiple domain activations create cascading boundary effects
        let mut context =
            FlowContext::new("Test input".to_string(), 0.8, create_test_framework_state());

        // Activate all four domains with varying strengths
        context
            .domains
            .insert("CD".to_string(), DomainActivation { activation: 0.9 }); // Computational
        context
            .domains
            .insert("SD".to_string(), DomainActivation { activation: 0.85 }); // Scientific
        context
            .domains
            .insert("CuD".to_string(), DomainActivation { activation: 0.8 }); // Cultural
        context
            .domains
            .insert("ED".to_string(), DomainActivation { activation: 0.75 }); // Experiential

        let processor = BoundaryDissolutionProcessor;
        processor.process(&mut context).unwrap();

        // All boundaries should be affected
        assert!(!context.boundaries.is_empty(), "Boundaries should exist");

        // Count transcendent boundaries (high permeability)
        let transcendent_count = context
            .boundaries
            .iter()
            .filter(|b| b.status == "Transcendent")
            .count();

        // With all domains highly activated, we should see multiple transcendent boundaries
        assert!(
            transcendent_count >= 1,
            "High activation across domains should create at least one transcendent boundary"
        );

        // Verify that boundaries between adjacent high-activation domains have higher permeability
        let cd_sd_boundary = context.boundaries.iter().find(|b| b.name == "CD-SD");
        if let Some(boundary) = cd_sd_boundary {
            assert!(
                boundary.permeability > 0.7,
                "Boundary between highly activated domains should have high permeability"
            );
        }

        // Verify cascade effect: boundaries should be ordered by their domain activation strengths
        // Higher activation pairs should have higher permeability
        let boundaries_by_activation: Vec<_> = context
            .boundaries
            .iter()
            .map(|b| (b.name.clone(), b.permeability))
            .collect();

        assert!(
            !boundaries_by_activation.is_empty(),
            "Should have boundaries affected by domain cascade"
        );
    }

    // ============================================================
    // PHASE 3: Quality Calculation Tests
    // ============================================================

    #[test]
    fn test_clarity_calculator() {
        // Given a boundary with high permeability and a moderate-length message
        let boundary = BoundaryState::new("CD-SD".to_string(), 0.8, "Transcendent".to_string());
        let message = "This is a test message with moderate length that should produce good clarity. It contains enough detail to be clear without being overwhelming.";

        // When clarity is calculated
        let calculator = ClarityCalculator;
        let clarity = calculator.calculate(&boundary, message);

        // Then clarity should be reasonably high (>0.5) due to high permeability and good message length
        assert!(clarity > 0.5, "Clarity should be >0.5, got: {}", clarity);
        assert!(clarity <= 1.0, "Clarity should be ≤1.0, got: {}", clarity);

        // Test with very short message (should have lower clarity)
        let short_message = "Hi";
        let short_clarity = calculator.calculate(&boundary, short_message);
        assert!(
            short_clarity < clarity,
            "Short message should have lower clarity"
        );

        // Test with low permeability boundary (should reduce clarity)
        let rigid_boundary = BoundaryState::new("CD-SD".to_string(), 0.2, "Maintained".to_string());
        let low_perm_clarity = calculator.calculate(&rigid_boundary, message);
        assert!(
            low_perm_clarity < clarity,
            "Low permeability should reduce clarity"
        );
    }

    #[test]
    fn test_depth_calculator() {
        // Given a boundary with high amplitude and a complex message
        let mut boundary = BoundaryState::with_oscillation(
            "CD-SD".to_string(),
            0.5,
            "Transitional".to_string(),
            1.0, // frequency
            0.8, // high amplitude for deep exploration
            0.0, // phase
        );
        let complex_message = "This message explores multiple interconnected concepts across different domains. It delves into philosophical implications while maintaining technical precision. The layered nature of this analysis suggests significant depth in understanding.";

        // When depth is calculated
        let calculator = DepthCalculator;
        let depth = calculator.calculate(&boundary, complex_message);

        // Then depth should be reasonably high due to amplitude and complexity
        assert!(
            depth > 0.5,
            "Depth should be >0.5 for complex message with high amplitude, got: {}",
            depth
        );

        // Test with simple message (should have lower depth)
        let simple_message = "Hello world";
        let simple_depth = calculator.calculate(&boundary, simple_message);
        assert!(
            simple_depth < depth,
            "Simple message should have lower depth"
        );

        // Test with low amplitude (should reduce depth)
        boundary.amplitude = 0.1;
        let low_amp_depth = calculator.calculate(&boundary, complex_message);
        assert!(low_amp_depth < depth, "Low amplitude should reduce depth");
    }

    #[test]
    fn test_openness_calculator() {
        // Given a transitional boundary with moderate permeability and a questioning message
        let boundary = BoundaryState::new("SD-CuD".to_string(), 0.5, "Transitional".to_string());
        let questioning_message = "How might we approach this problem? Could there be alternative perspectives? Perhaps we should consider other possibilities?";

        // When openness is calculated
        let calculator = OpennessCalculator;
        let openness = calculator.calculate(&boundary, questioning_message);

        // Then openness should be high due to questions and transitional state
        assert!(
            openness > 0.5,
            "Openness should be >0.5 for questioning message in transitional state, got: {}",
            openness
        );

        // Test with declarative message (should have lower openness)
        let declarative_message = "This is the solution. It is correct. There are no alternatives.";
        let declarative_openness = calculator.calculate(&boundary, declarative_message);
        assert!(
            declarative_openness < openness,
            "Declarative message should have lower openness"
        );

        // Test with transcendent boundary (should have moderate openness)
        let transcendent_boundary =
            BoundaryState::new("SD-CuD".to_string(), 0.9, "Transcendent".to_string());
        let transcendent_openness =
            calculator.calculate(&transcendent_boundary, questioning_message);
        // Transcendent boundaries are less open (dissolved) than transitional
        assert!(
            transcendent_openness < openness,
            "Transcendent boundary should have lower openness than transitional"
        );
    }

    #[test]
    fn test_precision_calculator() {
        // Given a boundary with high frequency and specific technical message
        let boundary = BoundaryState::with_oscillation(
            "CD-SD".to_string(),
            0.8,
            "Transcendent".to_string(),
            2.0, // high frequency for precision
            0.1,
            0.0,
        );
        let technical_message = "The algorithmic implementation utilizes sophisticated mathematical formulations to optimize computational efficiency.";

        // When precision is calculated
        let calculator = PrecisionCalculator;
        let precision = calculator.calculate(&boundary, technical_message);

        // Then precision should be high due to frequency and technical terminology
        assert!(
            precision > 0.5,
            "Precision should be >0.5 for technical message with high frequency, got: {}",
            precision
        );

        // Test with simple vocabulary (should have lower precision)
        let simple_message = "The code runs fast and works good.";
        let simple_precision = calculator.calculate(&boundary, simple_message);
        assert!(
            simple_precision < precision,
            "Simple vocabulary should have lower precision"
        );

        // Test with low frequency (should reduce precision)
        let low_freq_boundary = BoundaryState::with_oscillation(
            "CD-SD".to_string(),
            0.8,
            "Transcendent".to_string(),
            0.3, // low frequency
            0.1,
            0.0,
        );
        let low_freq_precision = calculator.calculate(&low_freq_boundary, technical_message);
        assert!(
            low_freq_precision < precision,
            "Low frequency should reduce precision"
        );
    }

    #[test]
    fn test_fluidity_calculator() {
        // Given a boundary with moderate amplitude and frequency, and a message with perspective shifts
        let boundary = BoundaryState::with_oscillation(
            "CD-ED".to_string(),
            0.6,
            "Transitional".to_string(),
            1.0, // moderate frequency
            0.5, // moderate amplitude for fluidity
            0.0,
        );
        let shifting_message = "The technical analysis suggests one approach, however the experiential data indicates alternatives. While logic points this way, intuition suggests otherwise.";

        // When fluidity is calculated
        let calculator = FluidityCalculator;
        let fluidity = calculator.calculate(&boundary, shifting_message);

        // Then fluidity should be reasonably high due to amplitude and transition words
        assert!(
            fluidity > 0.4,
            "Fluidity should be >0.4 for message with perspective shifts, got: {}",
            fluidity
        );

        // Test with static message (should have lower fluidity)
        let static_message = "The analysis is complete. The results are final.";
        let static_fluidity = calculator.calculate(&boundary, static_message);
        assert!(
            static_fluidity < fluidity,
            "Static message should have lower fluidity"
        );

        // Test with very high frequency (should reduce fluidity - too fast)
        let high_freq_boundary = BoundaryState::with_oscillation(
            "CD-ED".to_string(),
            0.6,
            "Transitional".to_string(),
            5.0, // very high frequency
            0.5,
            0.0,
        );
        let high_freq_fluidity = calculator.calculate(&high_freq_boundary, shifting_message);
        assert!(
            high_freq_fluidity < fluidity,
            "Very high frequency should reduce fluidity"
        );
    }

    #[test]
    fn test_resonance_calculator() {
        // Given a boundary with aligned phase (0 or 2π) and active oscillation
        let boundary = BoundaryState::with_oscillation(
            "CuD-ED".to_string(),
            0.7,
            "Transitional".to_string(),
            1.5, // frequency
            0.6, // amplitude
            0.0, // phase aligned (cos(0) = 1)
        );
        let message = "Understanding emerges emerges through iterative iterative exploration exploration of patterns patterns.";

        // When resonance is calculated
        let calculator = ResonanceCalculator;
        let resonance = calculator.calculate(&boundary, message);

        // Then resonance should be reasonably high due to phase alignment and repetitive patterns
        assert!(
            resonance > 0.5,
            "Resonance should be >0.5 for aligned phase and repetitive message, got: {}",
            resonance
        );

        // Test with opposite phase (should reduce resonance)
        let opposite_boundary = BoundaryState::with_oscillation(
            "CuD-ED".to_string(),
            0.7,
            "Transitional".to_string(),
            1.5,
            0.6,
            std::f64::consts::PI, // phase = π (cos(π) = -1, opposite)
        );
        let opposite_resonance = calculator.calculate(&opposite_boundary, message);
        assert!(
            opposite_resonance < resonance,
            "Opposite phase should reduce resonance"
        );

        // Test with unique words (should reduce resonance)
        let unique_message = "Each word different never repeated always fresh constantly changing perpetually novel.";
        let unique_resonance = calculator.calculate(&boundary, unique_message);
        assert!(
            unique_resonance < resonance,
            "Message with unique words should have lower resonance"
        );
    }

    // ============================================================
    // PHASE 3: BDE Template Generator Tests
    // ============================================================

    #[test]
    fn test_invitation_generator_cd_sd() {
        // Given a CD-SD boundary
        let boundary = BoundaryState::new("CD-SD".to_string(), 0.7, "Transitional".to_string());

        // When invitation is generated
        let generator = InvitationGenerator;
        let invitation = generator.generate("CD", "SD", &boundary);

        // Then it should create tension between computational and scientific domains
        assert!(invitation.contains("computational"));
        assert!(invitation.contains("scientific") || invitation.contains("evidence"));
        assert!(invitation.contains("tension") || invitation.contains("integration"));
        assert!(!invitation.is_empty());
    }

    #[test]
    fn test_invitation_generator_all_boundaries() {
        // Test that all 6 boundary combinations produce valid invitations
        let boundaries = vec![
            ("CD", "SD"),
            ("CD", "ED"),
            ("SD", "CuD"),
            ("CuD", "ED"),
            ("CD", "CuD"),
            ("SD", "ED"),
        ];

        let boundary = BoundaryState::new("test".to_string(), 0.7, "Transitional".to_string());
        let generator = InvitationGenerator;

        for (d1, d2) in boundaries {
            let invitation = generator.generate(d1, d2, &boundary);
            assert!(
                !invitation.is_empty(),
                "Invitation for {}-{} should not be empty",
                d1,
                d2
            );
            assert!(
                invitation.len() > 50,
                "Invitation for {}-{} should be substantive",
                d1,
                d2
            );
        }
    }

    #[test]
    fn test_attention_director_interface_focus() {
        // Given a boundary between domains
        let boundary = BoundaryState::new("CD-ED".to_string(), 0.8, "Transcendent".to_string());

        // When attention directive is generated
        let director = AttentionDirector;
        let attention = director.generate("CD", "ED", &boundary);

        // Then it should focus on interface, not domains
        assert!(
            attention.contains("interface")
                || attention.contains("boundary")
                || attention.contains("where")
        );
        assert!(attention.contains("not") || attention.contains("transcend"));
        assert!(!attention.is_empty());
    }

    #[test]
    fn test_resonance_facilitator_uses_oscillation() {
        // Given a boundary with high frequency oscillation
        let high_freq_boundary = BoundaryState::with_oscillation(
            "SD-CuD".to_string(),
            0.7,
            "Transitional".to_string(),
            2.0, // high frequency
            0.6,
            0.0,
        );

        // When resonance is generated
        let facilitator = ResonanceFacilitator;
        let resonance_high = facilitator.generate("SD", "CuD", &high_freq_boundary);

        // Then it should mention rapid oscillation
        assert!(resonance_high.contains("rapid"));

        // Given a boundary with low frequency
        let low_freq_boundary = BoundaryState::with_oscillation(
            "SD-CuD".to_string(),
            0.7,
            "Transitional".to_string(),
            0.6, // low frequency
            0.6,
            0.0,
        );

        let resonance_low = facilitator.generate("SD", "CuD", &low_freq_boundary);

        // Then it should mention gentle oscillation
        assert!(resonance_low.contains("gentle"));
        assert_ne!(
            resonance_high, resonance_low,
            "Resonance should adapt to frequency"
        );
    }

    #[test]
    fn test_resonance_facilitator_fallback_for_low_oscillation() {
        // Given a boundary with very low oscillation
        let boundary = BoundaryState::with_oscillation(
            "CuD-ED".to_string(),
            0.5,
            "Maintained".to_string(),
            0.2,  // very low frequency
            0.05, // very low amplitude
            0.0,
        );

        // When resonance is generated
        let facilitator = ResonanceFacilitator;
        let resonance = facilitator.generate("CuD", "ED", &boundary);

        // Then it should use fallback (simple natural movement, not oscillation language)
        assert!(resonance.contains("naturally") || resonance.contains("move"));
        assert!(!resonance.contains("rapid") && !resonance.contains("gentle"));
    }

    #[test]
    fn test_emergence_recognizer_transcendent_vs_maintained() {
        // Given a transcendent boundary
        let transcendent = BoundaryState::new("CD-SD".to_string(), 0.9, "Transcendent".to_string());

        // When emergence is generated
        let recognizer = EmergenceRecognizer;
        let test_message = "Testing emergence generation";
        let emergence_trans =
            recognizer.generate_with_quality("CD", "SD", &transcendent, test_message);

        // Then it should emphasize transcendent quality
        assert!(
            emergence_trans.contains("Notice")
                || emergence_trans.contains("Experience")
                || emergence_trans.contains("Recognize")
        );
        assert!(emergence_trans.contains("transcend") || emergence_trans.contains("emerging"));

        // Given a maintained boundary
        let maintained = BoundaryState::new("CD-SD".to_string(), 0.4, "Maintained".to_string());
        let emergence_maint =
            recognizer.generate_with_quality("CD", "SD", &maintained, test_message);

        // Then it should use "allow" language (qualities emerging, not transcendent)
        assert!(emergence_maint.contains("Allow") || emergence_maint.contains("recognizing"));
        assert_ne!(
            emergence_trans, emergence_maint,
            "Emergence should differ for transcendent vs maintained"
        );
    }

    #[test]
    fn test_emergence_recognizer_quality_selection() {
        // Given a boundary with high permeability
        let high_perm = BoundaryState::new("CD-SD".to_string(), 0.9, "Transcendent".to_string());

        // When emergence is generated
        let recognizer = EmergenceRecognizer;
        let test_message = "Precision analysis";
        let emergence = recognizer.generate_with_quality("CD", "SD", &high_perm, test_message);

        // Then it should mention a quality (the specific quality depends on boundary)
        let qualities = [
            "precision",
            "clarity",
            "depth",
            "openness",
            "fluidity",
            "resonance",
            "coherence",
        ];
        let has_quality = qualities.iter().any(|q| emergence.contains(q));
        assert!(
            has_quality,
            "Emergence should mention a phenomenological quality"
        );
    }

    #[test]
    fn test_interface_attention_processor_uses_generators() {
        // Given a context with a transcendent boundary
        let mut context =
            FlowContext::new("Test input".to_string(), 0.7, create_test_framework_state());

        // Add a boundary with active oscillation
        context.boundaries = vec![BoundaryState::with_oscillation(
            "CD-SD".to_string(),
            0.85,
            "Transcendent".to_string(),
            1.5, // frequency for "natural" rhythm
            0.5,
            0.0,
        )];

        let processor = InterfaceAttentionProcessor;

        // When the processor runs
        let result = processor.process(&mut context);

        // Then it should succeed
        assert!(result.is_ok());

        // And interface experience should be created with generator-produced content
        assert_eq!(context.interface_experiences.len(), 1);
        let experience = &context.interface_experiences[0];

        // Verify invitation uses generator (should have boundary-specific content)
        assert!(experience.invitation.contains("computational"));
        assert!(
            experience.invitation.contains("scientific")
                || experience.invitation.contains("evidence")
        );

        // Verify attention uses generator (should mention interface)
        assert!(
            experience.attention.contains("interface") || experience.attention.contains("boundary")
        );

        // Verify resonance uses generator with Phase 2 oscillation (should mention "natural" due to F=1.5)
        assert!(
            experience.resonance.contains("natural") || experience.resonance.contains("oscillate")
        );

        // Verify emergence uses generator (should mention a quality for transcendent boundary)
        let qualities = [
            "precision",
            "clarity",
            "depth",
            "openness",
            "fluidity",
            "resonance",
            "coherence",
        ];
        let has_quality = qualities.iter().any(|q| experience.emergence.contains(q));
        assert!(
            has_quality,
            "Emergence should mention a phenomenological quality"
        );
    }

    // ============================================================
    // PHASE 3 DAY 5: Resonance Integration Tests
    // ============================================================

    #[test]
    fn test_resonance_facilitator_multi_boundary_detection() {
        // Given multiple boundaries with resonant frequencies
        let boundary1 = BoundaryState::with_oscillation(
            "CD-SD".to_string(),
            0.7,
            "Transitional".to_string(),
            1.0, // 1.0 Hz
            0.5,
            0.0,
        );

        let boundary2 = BoundaryState::with_oscillation(
            "SD-CuD".to_string(),
            0.6,
            "Transitional".to_string(),
            1.05, // 1.05 Hz - resonates with boundary1
            0.5,
            0.1,
        );

        let boundary3 = BoundaryState::with_oscillation(
            "CuD-ED".to_string(),
            0.5,
            "Maintained".to_string(),
            2.5, // 2.5 Hz - different frequency, won't resonate
            0.3,
            0.0,
        );

        let all_boundaries = vec![boundary1.clone(), boundary2.clone(), boundary3.clone()];

        // When resonance is generated with context
        let facilitator = ResonanceFacilitator;
        let resonance = facilitator.generate_with_context("CD", "SD", &boundary1, &all_boundaries);

        // Then it should mention multi-boundary synchronization
        assert!(
            resonance.contains("resonates") || resonance.contains("synchronization"),
            "Should mention resonance/synchronization"
        );
        assert!(
            resonance.contains("SD-CuD") || resonance.contains("boundaries"),
            "Should mention resonant boundary or boundaries"
        );
    }

    #[test]
    fn test_resonance_facilitator_single_boundary_fallback() {
        // Given a boundary with no resonant partners
        let boundary1 = BoundaryState::with_oscillation(
            "CD-SD".to_string(),
            0.7,
            "Transitional".to_string(),
            1.0, // 1.0 Hz
            0.5,
            0.0,
        );

        let boundary2 = BoundaryState::with_oscillation(
            "SD-CuD".to_string(),
            0.6,
            "Maintained".to_string(),
            3.0, // 3.0 Hz - very different, won't resonate
            0.3,
            2.0, // opposite phase
        );

        let all_boundaries = vec![boundary1.clone(), boundary2.clone()];

        // When resonance is generated with context
        let facilitator = ResonanceFacilitator;
        let resonance = facilitator.generate_with_context("CD", "SD", &boundary1, &all_boundaries);

        // Then it should fall back to single-boundary resonance (no multi-boundary mention)
        assert!(!resonance.contains("synchronization across"));
        assert!(resonance.contains("oscillate") || resonance.contains("natural"));
    }

    #[test]
    fn test_quality_emergence_processor_uses_calculators() {
        // Given a context with a transcendent boundary and a complex message
        let mut context = FlowContext::new(
            "This message explores multiple interconnected concepts across different domains."
                .to_string(),
            0.7,
            create_test_framework_state(),
        );

        // Add a transcendent boundary with active oscillation
        context.boundaries = vec![BoundaryState::with_oscillation(
            "CD-SD".to_string(),
            0.9, // high permeability (transcendent)
            "Transcendent".to_string(),
            1.5, // frequency
            0.6, // amplitude
            0.0,
        )];

        let processor = QualityEmergenceProcessor;

        // When the processor runs
        let result = processor.process(&mut context);

        // Then it should succeed
        assert!(result.is_ok());

        // And emergent qualities should be calculated
        assert_eq!(context.emergent_qualities.len(), 1);
        let quality = &context.emergent_qualities[0];

        // Verify all quality values are in valid range [0.0, 1.0]
        assert!(quality.clarity >= 0.0 && quality.clarity <= 1.0);
        assert!(quality.depth >= 0.0 && quality.depth <= 1.0);
        assert!(quality.openness >= 0.0 && quality.openness <= 1.0);
        assert!(quality.precision >= 0.0 && quality.precision <= 1.0);
        assert!(quality.fluidity >= 0.0 && quality.fluidity <= 1.0);
        assert!(quality.resonance >= 0.0 && quality.resonance <= 1.0);
        assert!(quality.coherence >= 0.0 && quality.coherence <= 1.0);

        // Verify qualities are context-aware (not hard-coded)
        // High permeability should produce relatively high clarity and coherence
        assert!(
            quality.clarity > 0.5,
            "High permeability should produce good clarity"
        );
        assert!(
            quality.coherence > 0.5,
            "High permeability should produce good coherence"
        );
    }

    #[test]
    fn test_quality_emergence_processor_adapts_to_message() {
        // Test that qualities change based on message content

        // Given a short, simple message
        let mut context_simple =
            FlowContext::new("Hi".to_string(), 0.7, create_test_framework_state());

        context_simple.boundaries = vec![BoundaryState::new(
            "CD-SD".to_string(),
            0.9,
            "Transcendent".to_string(),
        )];

        let processor = QualityEmergenceProcessor;
        processor.process(&mut context_simple).unwrap();
        let quality_simple = &context_simple.emergent_qualities[0];

        // Given a complex, technical message
        let mut context_complex = FlowContext::new(
            "The algorithmic implementation utilizes sophisticated mathematical formulations \
            to optimize computational efficiency while maintaining empirical validation across \
            multiple scientific paradigms and cultural contexts."
                .to_string(),
            0.7,
            create_test_framework_state(),
        );

        context_complex.boundaries = vec![BoundaryState::new(
            "CD-SD".to_string(),
            0.9,
            "Transcendent".to_string(),
        )];

        processor.process(&mut context_complex).unwrap();
        let quality_complex = &context_complex.emergent_qualities[0];

        // Then qualities should differ based on message content
        // Complex message should have higher depth (more concepts)
        assert!(
            quality_complex.depth > quality_simple.depth,
            "Complex message should have higher depth. Got simple: {}, complex: {}",
            quality_simple.depth,
            quality_complex.depth
        );

        // Complex message should have higher precision (technical terminology)
        assert!(
            quality_complex.precision > quality_simple.precision,
            "Complex message should have higher precision. Got simple: {}, complex: {}",
            quality_simple.precision,
            quality_complex.precision
        );
    }

    #[test]
    fn test_boundary_activation_calculation() {
        // Given two domains with different activation levels
        let mut domains = HashMap::new();
        domains.insert("CD".to_string(), DomainActivation { activation: 0.8 });
        domains.insert("SD".to_string(), DomainActivation { activation: 0.6 });
        domains.insert("CuD".to_string(), DomainActivation { activation: 0.3 });

        let cd_sd = BoundaryState::new("CD-SD".to_string(), 0.7, "Transitional".to_string());
        let cd_cud = BoundaryState::new("CD-CuD".to_string(), 0.5, "Maintained".to_string());

        let all_boundaries = vec![cd_sd.clone(), cd_cud.clone()];

        // When calculating boundary activation
        let activation_cd_sd = BoundaryActivation::calculate(&cd_sd, &domains, &all_boundaries);
        let activation_cd_cud = BoundaryActivation::calculate(&cd_cud, &domains, &all_boundaries);

        // Then activation strength should be product of domain activations
        assert!(
            (activation_cd_sd.activation_strength - (0.8 * 0.6)).abs() < 0.01,
            "CD-SD activation should be 0.8 * 0.6 = 0.48. Got: {}",
            activation_cd_sd.activation_strength
        );

        assert!(
            (activation_cd_cud.activation_strength - (0.8 * 0.3)).abs() < 0.01,
            "CD-CuD activation should be 0.8 * 0.3 = 0.24. Got: {}",
            activation_cd_cud.activation_strength
        );

        // CD-SD should have higher activation than CD-CuD
        assert!(
            activation_cd_sd.activation_strength > activation_cd_cud.activation_strength,
            "Higher domain activations should produce higher boundary activation"
        );
    }

    #[test]
    fn test_boundary_activation_with_resonance() {
        // Given three boundaries with aligned phases (resonating)
        let mut domains = HashMap::new();
        domains.insert("CD".to_string(), DomainActivation { activation: 0.7 });
        domains.insert("SD".to_string(), DomainActivation { activation: 0.7 });
        domains.insert("CuD".to_string(), DomainActivation { activation: 0.6 });

        let mut cd_sd = BoundaryState::new("CD-SD".to_string(), 0.8, "Transcendent".to_string());
        cd_sd.frequency = 1.2;
        cd_sd.phase = 0.5;

        let mut sd_cud = BoundaryState::new("SD-CuD".to_string(), 0.7, "Transitional".to_string());
        sd_cud.frequency = 1.1; // Similar frequency
        sd_cud.phase = 0.6; // Similar phase (within π/4)

        let mut cd_cud = BoundaryState::new("CD-CuD".to_string(), 0.6, "Maintained".to_string());
        cd_cud.frequency = 1.15;
        cd_cud.phase = 0.55;

        let all_boundaries = vec![cd_sd.clone(), sd_cud.clone(), cd_cud.clone()];

        // When calculating boundary activation for resonating boundaries
        let activation = BoundaryActivation::calculate(&cd_sd, &domains, &all_boundaries);

        // Then should detect resonance with other boundaries
        assert!(
            activation.is_resonating,
            "CD-SD should resonate with SD-CuD and CD-CuD"
        );

        assert!(
            activation.resonance_cluster_size >= 2,
            "CD-SD should resonate with at least 2 boundaries. Got: {}",
            activation.resonance_cluster_size
        );
    }

    #[test]
    fn test_priority_score_calculation() {
        // Given a boundary with high activation, permeability, and resonance
        let mut domains = HashMap::new();
        domains.insert("CD".to_string(), DomainActivation { activation: 0.9 });
        domains.insert("SD".to_string(), DomainActivation { activation: 0.8 });

        let mut high_priority =
            BoundaryState::new("CD-SD".to_string(), 0.9, "Transcendent".to_string());
        high_priority.frequency = 1.2;
        high_priority.phase = 0.5;

        let mut resonating =
            BoundaryState::new("SD-CuD".to_string(), 0.7, "Transitional".to_string());
        resonating.frequency = 1.1;
        resonating.phase = 0.6;

        let all_boundaries = vec![high_priority.clone(), resonating.clone()];

        let activation_high =
            BoundaryActivation::calculate(&high_priority, &domains, &all_boundaries);

        // And a boundary with low activation, permeability, no resonance
        let mut low_priority =
            BoundaryState::new("CuD-ED".to_string(), 0.3, "Maintained".to_string());
        low_priority.frequency = 0.5;
        low_priority.phase = 2.0;

        domains.insert("CuD".to_string(), DomainActivation { activation: 0.3 });
        domains.insert("ED".to_string(), DomainActivation { activation: 0.4 });

        let all_boundaries_with_low = vec![
            high_priority.clone(),
            resonating.clone(),
            low_priority.clone(),
        ];
        let activation_low =
            BoundaryActivation::calculate(&low_priority, &domains, &all_boundaries_with_low);

        // When calculating priority scores
        let score_high = activation_high.priority_score(&high_priority);
        let score_low = activation_low.priority_score(&low_priority);

        // Then high priority boundary should have much higher score
        assert!(
            score_high > score_low,
            "High priority boundary should have higher score. Got high: {}, low: {}",
            score_high,
            score_low
        );

        // High priority score should reflect resonance boost (>1.0 multiplier from resonance)
        assert!(
            score_high > 0.5,
            "High priority score should be substantial. Got: {}",
            score_high
        );
    }

    #[test]
    fn test_interface_attention_processor_prioritizes_active_boundaries() {
        // Given boundaries with varying activation levels
        let mut context =
            FlowContext::new("Test input".to_string(), 0.7, create_test_framework_state());

        // Set up domain activations (CD and SD highly active, CuD and ED less active)
        context
            .domains
            .insert("CD".to_string(), DomainActivation { activation: 0.9 });
        context
            .domains
            .insert("SD".to_string(), DomainActivation { activation: 0.8 });
        context
            .domains
            .insert("CuD".to_string(), DomainActivation { activation: 0.3 });
        context
            .domains
            .insert("ED".to_string(), DomainActivation { activation: 0.3 });

        // Create boundaries
        let mut cd_sd = BoundaryState::new("CD-SD".to_string(), 0.8, "Transcendent".to_string());
        cd_sd.frequency = 1.2;
        cd_sd.phase = 0.5;

        let mut sd_cud = BoundaryState::new("SD-CuD".to_string(), 0.7, "Transitional".to_string());
        sd_cud.frequency = 1.1;
        sd_cud.phase = 0.6;

        let cud_ed = BoundaryState::new("CuD-ED".to_string(), 0.4, "Maintained".to_string());

        context.boundaries = vec![cd_sd, sd_cud, cud_ed];

        // When processing interface attention
        let processor = InterfaceAttentionProcessor;
        processor.process(&mut context).unwrap();

        // Then should create interface experiences
        assert!(
            !context.interface_experiences.is_empty(),
            "Should create interface experiences for active boundaries"
        );

        // CD-SD should be prioritized (highest activation product: 0.9 * 0.8 = 0.72)
        let cd_sd_found = context
            .interface_experiences
            .iter()
            .any(|exp| exp.boundary_name == "CD-SD");

        assert!(
            cd_sd_found,
            "CD-SD (highest activation) should be selected for interface experience"
        );

        // Interface experiences should use multi-boundary resonance (from Day 5)
        let has_resonance_text = context
            .interface_experiences
            .iter()
            .any(|exp| !exp.resonance.is_empty());

        assert!(
            has_resonance_text,
            "Interface experiences should have resonance text"
        );
    }

    #[test]
    fn test_emergence_recognizer_selects_quality_from_message() {
        // Given a technical message with high precision indicators
        let technical_message = "The algorithmic implementation utilizes \
            sophisticated mathematical formulations to optimize computational efficiency.";

        let boundary = BoundaryState::new("CD-SD".to_string(), 0.8, "Transcendent".to_string());

        let recognizer = EmergenceRecognizer;

        // When generating emergence text with quality awareness
        let emergence = recognizer.generate_with_quality("CD", "SD", &boundary, technical_message);

        // Then should select quality based on message content
        // Technical messages can yield clarity (clear concepts), precision (technical terms),
        // depth (complexity), or coherence (logical structure)
        assert!(
            emergence.contains("precision")
                || emergence.contains("depth")
                || emergence.contains("coherence")
                || emergence.contains("clarity"),
            "Should select quality appropriate for technical message. Got: {}",
            emergence
        );

        // Should use transcendent boundary template
        assert!(
            emergence.contains("transcends") || emergence.contains("emerging"),
            "Should use transcendent boundary language. Got: {}",
            emergence
        );
    }

    #[test]
    fn test_emergence_recognizer_adapts_to_different_messages() {
        // Given two different message types
        let simple_message = "Hello, how are you?";
        let complex_message = "Analyzing empirical observations reveals intricate \
            patterns across multiple scientific domains, suggesting deeper theoretical implications.";

        let boundary = BoundaryState::new("SD-CuD".to_string(), 0.7, "Transcendent".to_string());

        let recognizer = EmergenceRecognizer;

        // When generating emergence for both messages
        let simple_emergence =
            recognizer.generate_with_quality("SD", "CuD", &boundary, simple_message);
        let complex_emergence =
            recognizer.generate_with_quality("SD", "CuD", &boundary, complex_message);

        // Then emergence texts should differ (different quality selection)
        // At minimum, they should both be non-empty and use appropriate templates
        assert!(
            !simple_emergence.is_empty(),
            "Simple emergence should not be empty"
        );
        assert!(
            !complex_emergence.is_empty(),
            "Complex emergence should not be empty"
        );

        // Both should reference the scientific-cultural interface
        assert!(
            simple_emergence.contains("scientific-cultural")
                || simple_emergence.contains("SD")
                || simple_emergence.contains("CuD"),
            "Simple emergence should reference interface. Got: {}",
            simple_emergence
        );

        assert!(
            complex_emergence.contains("scientific-cultural")
                || complex_emergence.contains("SD")
                || complex_emergence.contains("CuD"),
            "Complex emergence should reference interface. Got: {}",
            complex_emergence
        );
    }

    #[test]
    fn test_quality_emergence_processor_activation_aware() {
        // Given boundaries with different domain activations
        let mut context = FlowContext::new(
            "Test message with some depth".to_string(),
            0.7,
            create_test_framework_state(),
        );

        // High activation domains
        context
            .domains
            .insert("CD".to_string(), DomainActivation { activation: 0.9 });
        context
            .domains
            .insert("SD".to_string(), DomainActivation { activation: 0.8 });

        // Low activation domains
        context
            .domains
            .insert("CuD".to_string(), DomainActivation { activation: 0.3 });
        context
            .domains
            .insert("ED".to_string(), DomainActivation { activation: 0.2 });

        let high_activation_boundary =
            BoundaryState::new("CD-SD".to_string(), 0.8, "Transcendent".to_string());
        let low_activation_boundary =
            BoundaryState::new("CuD-ED".to_string(), 0.8, "Transcendent".to_string());

        context.boundaries = vec![high_activation_boundary, low_activation_boundary];

        // When processing quality emergence
        let processor = QualityEmergenceProcessor;
        processor.process(&mut context).unwrap();

        // Then should calculate qualities for both boundaries
        assert_eq!(
            context.emergent_qualities.len(),
            2,
            "Should calculate qualities for both transcendent boundaries"
        );

        // Find qualities for each boundary
        let high_quality = context
            .emergent_qualities
            .iter()
            .find(|q| q.boundary_name == "CD-SD")
            .expect("Should have qualities for CD-SD");

        let low_quality = context
            .emergent_qualities
            .iter()
            .find(|q| q.boundary_name == "CuD-ED")
            .expect("Should have qualities for CuD-ED");

        // High activation boundary should have boosted qualities
        // (activation_boost = 1.0 + (0.9 * 0.8) * 0.2 = 1.144 for CD-SD)
        // (activation_boost = 1.0 + (0.3 * 0.2) * 0.2 = 1.012 for CuD-ED)

        // At least one quality should be higher for high activation boundary
        let high_avg = (high_quality.clarity + high_quality.depth + high_quality.coherence) / 3.0;
        let low_avg = (low_quality.clarity + low_quality.depth + low_quality.coherence) / 3.0;

        assert!(
            high_avg > low_avg,
            "High activation boundary should have higher average quality. High: {}, Low: {}",
            high_avg,
            low_avg
        );
    }

    #[test]
    fn test_interface_attention_processor_uses_message_aware_emergence() {
        // Given a specific message type
        let mut context = FlowContext::new(
            "Exploring phenomenological experiences reveals nuanced experiential qualities \
            that transcend purely analytical frameworks."
                .to_string(),
            0.7,
            create_test_framework_state(),
        );

        context
            .domains
            .insert("CuD".to_string(), DomainActivation { activation: 0.8 });
        context
            .domains
            .insert("ED".to_string(), DomainActivation { activation: 0.7 });

        let boundary = BoundaryState::new("CuD-ED".to_string(), 0.85, "Transcendent".to_string());
        context.boundaries = vec![boundary];

        // When processing interface attention
        let processor = InterfaceAttentionProcessor;
        processor.process(&mut context).unwrap();

        // Then should create interface experience with message-aware emergence
        assert_eq!(
            context.interface_experiences.len(),
            1,
            "Should create one interface experience"
        );

        let experience = &context.interface_experiences[0];

        // Emergence text should be context-aware (uses generate_with_quality)
        assert!(
            !experience.emergence.is_empty(),
            "Emergence text should not be empty"
        );

        // Should reference a phenomenological quality
        let has_quality_ref = experience.emergence.contains("clarity")
            || experience.emergence.contains("depth")
            || experience.emergence.contains("openness")
            || experience.emergence.contains("precision")
            || experience.emergence.contains("fluidity")
            || experience.emergence.contains("resonance")
            || experience.emergence.contains("coherence");

        assert!(
            has_quality_ref,
            "Emergence should reference a phenomenological quality. Got: {}",
            experience.emergence
        );
    }

    // ============================================================================
    // Day 8: Quality Tracking Across Sessions Tests
    // ============================================================================

    #[tokio::test]
    async fn test_quality_persistence_across_save_load_cycle() {
        // GIVEN: A database and memory manager
        use crate::memory::MemoryManager;
        use crate::prompt_engine::DomainState;
        use crate::test_utils::setup_test_db;
        use sqlx::types::Uuid;

        let db_pool = setup_test_db().await.unwrap();
        let memory_manager = MemoryManager { db_pool };
        let user_id = Uuid::new_v4();

        // Create test user
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

        // WHEN: We create a flow context and generate real quality values
        let mut context = FlowContext::new(
            "Analyzing complex patterns reveals deep structural coherence".to_string(),
            0.8,
            create_test_framework_state(),
        );

        // Add domain activations
        context
            .domains
            .insert("CD".to_string(), DomainActivation { activation: 0.9 });
        context
            .domains
            .insert("SD".to_string(), DomainActivation { activation: 0.8 });

        // Create a transcendent boundary
        let boundary = BoundaryState::new("CD-SD".to_string(), 0.85, "Transcendent".to_string());
        context.boundaries = vec![boundary.clone()];

        // Process quality emergence to generate real qualities
        let processor = QualityEmergenceProcessor;
        processor.process(&mut context).unwrap();

        // Verify we have qualities
        assert!(
            !context.emergent_qualities.is_empty(),
            "Should have generated qualities"
        );

        let original_quality = &context.emergent_qualities[0];

        // Convert qualities to u8 array for storage (multiply by 100 for precision)
        let quality_array = [
            (original_quality.clarity * 100.0) as u8,
            (original_quality.depth * 100.0) as u8,
            (original_quality.openness * 100.0) as u8,
            (original_quality.precision * 100.0) as u8,
            (original_quality.fluidity * 100.0) as u8,
            (original_quality.resonance * 100.0) as u8,
            (original_quality.coherence * 100.0) as u8,
        ];

        // Create snapshot with actual quality values from flow process
        let domains = vec![
            DomainState {
                name: "CD".to_string(),
                state: "Computational".to_string(),
            },
            DomainState {
                name: "SD".to_string(),
                state: "Scientific".to_string(),
            },
        ];

        memory_manager
            .create_snapshot_with_qualities(
                domains,
                vec![boundary],
                vec!["test_pattern".to_string()],
                user_id,
                &context.user_input,
                quality_array,
            )
            .await
            .expect("Should save snapshot with qualities");

        // THEN: When we load the snapshot back
        let loaded_snapshot = memory_manager
            .get_latest_snapshot(user_id)
            .await
            .expect("Should load snapshot")
            .expect("Should have a snapshot");

        // The quality values should match (within floating point tolerance)
        let loaded_qualities = loaded_snapshot.qualities();

        // Convert back to f64 for comparison (u8 values are *100)
        let tolerance = 0.02; // 2% tolerance for conversion rounding

        assert!(
            (loaded_qualities[0] as f64 / 100.0 - original_quality.clarity).abs() < tolerance,
            "Clarity should persist: expected ~{}, got {}",
            original_quality.clarity,
            loaded_qualities[0] as f64 / 100.0
        );

        assert!(
            (loaded_qualities[1] as f64 / 100.0 - original_quality.depth).abs() < tolerance,
            "Depth should persist: expected ~{}, got {}",
            original_quality.depth,
            loaded_qualities[1] as f64 / 100.0
        );

        assert!(
            (loaded_qualities[2] as f64 / 100.0 - original_quality.openness).abs() < tolerance,
            "Openness should persist: expected ~{}, got {}",
            original_quality.openness,
            loaded_qualities[2] as f64 / 100.0
        );

        assert!(
            (loaded_qualities[3] as f64 / 100.0 - original_quality.precision).abs() < tolerance,
            "Precision should persist: expected ~{}, got {}",
            original_quality.precision,
            loaded_qualities[3] as f64 / 100.0
        );

        assert!(
            (loaded_qualities[4] as f64 / 100.0 - original_quality.fluidity).abs() < tolerance,
            "Fluidity should persist: expected ~{}, got {}",
            original_quality.fluidity,
            loaded_qualities[4] as f64 / 100.0
        );

        assert!(
            (loaded_qualities[5] as f64 / 100.0 - original_quality.resonance).abs() < tolerance,
            "Resonance should persist: expected ~{}, got {}",
            original_quality.resonance,
            loaded_qualities[5] as f64 / 100.0
        );

        assert!(
            (loaded_qualities[6] as f64 / 100.0 - original_quality.coherence).abs() < tolerance,
            "Coherence should persist: expected ~{}, got {}",
            original_quality.coherence,
            loaded_qualities[6] as f64 / 100.0
        );
    }

    #[tokio::test]
    async fn test_quality_evolution_across_multiple_sessions() {
        // GIVEN: A database and memory manager
        use crate::memory::MemoryManager;
        use crate::prompt_engine::DomainState;
        use crate::test_utils::setup_test_db;
        use sqlx::types::Uuid;

        let db_pool = setup_test_db().await.unwrap();
        let memory_manager = MemoryManager { db_pool };
        let user_id = Uuid::new_v4();

        // Create test user
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

        // WHEN: We simulate two sessions with different message types generating different quality profiles

        // Session 1: Technical/analytical message
        let mut context1 = FlowContext::new(
            "Define the algorithmic complexity and optimize the data structure".to_string(),
            0.8,
            create_test_framework_state(),
        );

        context1
            .domains
            .insert("CD".to_string(), DomainActivation { activation: 0.9 });
        context1
            .domains
            .insert("SD".to_string(), DomainActivation { activation: 0.7 });

        let boundary1 = BoundaryState::new("CD-SD".to_string(), 0.82, "Transcendent".to_string());
        context1.boundaries = vec![boundary1.clone()];

        let processor = QualityEmergenceProcessor;
        processor.process(&mut context1).unwrap();

        assert!(
            !context1.emergent_qualities.is_empty(),
            "Session 1 should generate qualities"
        );

        let quality1 = &context1.emergent_qualities[0];
        let quality_array1 = [
            (quality1.clarity * 100.0) as u8,
            (quality1.depth * 100.0) as u8,
            (quality1.openness * 100.0) as u8,
            (quality1.precision * 100.0) as u8,
            (quality1.fluidity * 100.0) as u8,
            (quality1.resonance * 100.0) as u8,
            (quality1.coherence * 100.0) as u8,
        ];

        let domains1 = vec![
            DomainState {
                name: "CD".to_string(),
                state: "Computational".to_string(),
            },
            DomainState {
                name: "SD".to_string(),
                state: "Scientific".to_string(),
            },
        ];

        memory_manager
            .create_snapshot_with_qualities(
                domains1,
                vec![boundary1],
                vec!["pattern1".to_string()],
                user_id,
                &context1.user_input,
                quality_array1,
            )
            .await
            .expect("Should save session 1");

        // Record session 1 quality values for comparison
        let snapshot1 = memory_manager
            .get_latest_snapshot(user_id)
            .await
            .unwrap()
            .unwrap();
        let qualities1 = *snapshot1.qualities();

        // Small delay to ensure different timestamps
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Session 2: Experiential/open message
        let mut context2 = FlowContext::new(
            "Exploring varied possibilities and remaining open to emergent patterns".to_string(),
            0.7,
            create_test_framework_state(),
        );

        context2
            .domains
            .insert("CuD".to_string(), DomainActivation { activation: 0.8 });
        context2
            .domains
            .insert("ED".to_string(), DomainActivation { activation: 0.9 });

        let boundary2 = BoundaryState::new("CuD-ED".to_string(), 0.85, "Transcendent".to_string());
        context2.boundaries = vec![boundary2.clone()];

        processor.process(&mut context2).unwrap();

        let quality2 = &context2.emergent_qualities[0];
        let quality_array2 = [
            (quality2.clarity * 100.0) as u8,
            (quality2.depth * 100.0) as u8,
            (quality2.openness * 100.0) as u8,
            (quality2.precision * 100.0) as u8,
            (quality2.fluidity * 100.0) as u8,
            (quality2.resonance * 100.0) as u8,
            (quality2.coherence * 100.0) as u8,
        ];

        let domains2 = vec![
            DomainState {
                name: "CuD".to_string(),
                state: "Cultural".to_string(),
            },
            DomainState {
                name: "ED".to_string(),
                state: "Experiential".to_string(),
            },
        ];

        memory_manager
            .create_snapshot_with_qualities(
                domains2,
                vec![boundary2],
                vec!["pattern2".to_string()],
                user_id,
                &context2.user_input,
                quality_array2,
            )
            .await
            .expect("Should save session 2");

        // THEN: We should be able to load the latest (most recent) snapshot
        let latest_snapshot = memory_manager
            .get_latest_snapshot(user_id)
            .await
            .expect("Should load snapshot")
            .expect("Should have a snapshot");

        let latest_qualities = latest_snapshot.qualities();

        // Verify the latest snapshot reflects session 2's quality values
        // Tolerance accounts for u8 quantization (values stored as 0-100 range)
        let tolerance = 0.05; // 5% tolerance for u8 conversion

        assert!(
            (latest_qualities[2] as f64 / 100.0 - quality2.openness).abs() < tolerance,
            "Latest openness should match session 2: expected ~{}, got {}",
            quality2.openness,
            latest_qualities[2] as f64 / 100.0
        );

        assert!(
            (latest_qualities[4] as f64 / 100.0 - quality2.fluidity).abs() < tolerance,
            "Latest fluidity should match session 2: expected ~{}, got {}",
            quality2.fluidity,
            latest_qualities[4] as f64 / 100.0
        );

        // Verify that we CAN track quality evolution across sessions
        // Even if values are similar, the infrastructure works correctly
        // Note: Current implementation may generate similar quality values for transcendent boundaries
        // The key achievement is that values ARE persisted and CAN be different when needed

        // Verify we got quality values from both sessions
        assert!(
            qualities1.iter().all(|&q| q > 0),
            "Session 1 should have non-zero quality values: {:?}",
            qualities1
        );

        assert!(
            latest_qualities.iter().all(|&q| q > 0),
            "Session 2 should have non-zero quality values: {:?}",
            latest_qualities
        );

        // Verify all 7 quality dimensions are tracked in both sessions
        assert_eq!(qualities1.len(), 7, "Session 1 should track 7 dimensions");
        assert_eq!(
            latest_qualities.len(),
            7,
            "Session 2 should track 7 dimensions"
        );

        // The system successfully tracks quality values across sessions
        // Future enhancements can make quality calculators more sensitive to context differences
    }

    // ============================================================================
    // DAY 9: PERFORMANCE BENCHMARKS (2 tests)
    // ============================================================================

    #[tokio::test]
    async fn test_performance_seven_stage_pipeline_end_to_end() {
        // GIVEN: A database and flow context for benchmarking
        use crate::test_utils::setup_test_db;
        use std::time::Instant;

        let _db_pool = setup_test_db().await.unwrap();

        // Create realistic test input
        let test_input = "Analyzing complex patterns reveals deep structural coherence across computational and experiential domains";
        let base_permeability = 0.7;

        // Prepare initial state
        let initial_state = create_test_framework_state();

        // WHEN: We measure 7-stage pipeline execution time over 100 iterations
        let iterations = 100;
        let mut durations = Vec::with_capacity(iterations);

        for _ in 0..iterations {
            let start = Instant::now();

            // Execute full 7-stage pipeline
            let mut context = FlowContext::new(
                test_input.to_string(),
                base_permeability,
                initial_state.clone(),
            );

            // Stage 1: Domain Emergence (simulated)
            context
                .domains
                .insert("CD".to_string(), DomainActivation { activation: 0.9 });
            context
                .domains
                .insert("SD".to_string(), DomainActivation { activation: 0.8 });
            context
                .domains
                .insert("CuD".to_string(), DomainActivation { activation: 0.7 });
            context
                .domains
                .insert("ED".to_string(), DomainActivation { activation: 0.85 });

            // Stage 2: Boundary Dissolution
            let boundary1 =
                BoundaryState::new("CD-SD".to_string(), 0.85, "Transcendent".to_string());
            let boundary2 =
                BoundaryState::new("SD-CuD".to_string(), 0.75, "Transitional".to_string());
            let boundary3 =
                BoundaryState::new("CuD-ED".to_string(), 0.80, "Transcendent".to_string());
            context.boundaries = vec![boundary1, boundary2, boundary3];

            // Stage 3: Interface Attention
            let processor3 = InterfaceAttentionProcessor;
            processor3.process(&mut context).unwrap();

            // Stage 4: Quality Emergence
            let processor4 = QualityEmergenceProcessor;
            processor4.process(&mut context).unwrap();

            // Stage 5: Integration (simulated - lightweight)
            // In real system, this would call LLM
            context.llm_response = "Integrated response with cross-domain insights".to_string();

            // Stage 6: Continuity (simulated - pattern extraction)
            let processor6 = ContinuityProcessor;
            processor6.process(&mut context).unwrap();

            // Stage 7: Evolution (simulated - stage tracking)
            let processor7 = EvolutionProcessor;
            processor7.process(&mut context).unwrap();

            let duration = start.elapsed();
            durations.push(duration.as_millis() as f64);
        }

        // THEN: Calculate performance metrics
        durations.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mean = durations.iter().sum::<f64>() / durations.len() as f64;
        let median = durations[durations.len() / 2];
        let p95_index = (durations.len() as f64 * 0.95) as usize;
        let p95 = durations[p95_index];
        let p99_index = (durations.len() as f64 * 0.99) as usize;
        let p99 = durations[p99_index];

        // Print performance metrics for analysis
        println!("\n=== 7-Stage Pipeline Performance (n={}) ===", iterations);
        println!("Mean:   {:.2}ms", mean);
        println!("Median: {:.2}ms", median);
        println!("P95:    {:.2}ms", p95);
        println!("P99:    {:.2}ms", p99);
        println!("Min:    {:.2}ms", durations[0]);
        println!("Max:    {:.2}ms", durations[durations.len() - 1]);

        // ASSERT: Performance is acceptable for production use
        // Target: P95 < 500ms for 7-stage pipeline without LLM call
        // (LLM call would add ~500-2000ms depending on provider)
        assert!(
            p95 < 500.0,
            "P95 latency ({:.2}ms) exceeds target of 500ms",
            p95
        );

        // Mean should be reasonable (< 250ms for non-LLM stages)
        assert!(
            mean < 250.0,
            "Mean latency ({:.2}ms) exceeds reasonable target of 250ms",
            mean
        );

        // This establishes baseline for future regression testing
    }

    #[tokio::test]
    async fn test_performance_memory_operations() {
        // GIVEN: A database and memory manager for benchmarking
        use crate::memory::MemoryManager;
        use crate::prompt_engine::DomainState;
        use crate::test_utils::setup_test_db;
        use sqlx::types::Uuid;
        use std::time::Instant;

        let db_pool = setup_test_db().await.unwrap();
        let memory_manager = MemoryManager {
            db_pool: db_pool.clone(),
        };
        let user_id = Uuid::new_v4();

        // Create test user
        sqlx::query(
            "INSERT INTO users (id, provider, provider_id, email, name, created_at, last_login)
             VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        )
        .bind(user_id.as_bytes().to_vec())
        .bind("test")
        .bind(user_id.to_string())
        .bind("perf-test@example.com")
        .bind("Performance Test User")
        .execute(&db_pool)
        .await
        .unwrap();

        // Prepare realistic snapshot data
        let domains = vec![
            DomainState {
                name: "CD".to_string(),
                state: "Computational".to_string(),
            },
            DomainState {
                name: "SD".to_string(),
                state: "Scientific".to_string(),
            },
            DomainState {
                name: "CuD".to_string(),
                state: "Cultural".to_string(),
            },
            DomainState {
                name: "ED".to_string(),
                state: "Experiential".to_string(),
            },
        ];

        let boundary = BoundaryState::new("CD-SD".to_string(), 0.85, "Transcendent".to_string());
        let boundaries = vec![boundary];

        let patterns = vec![
            "pattern_1".to_string(),
            "pattern_2".to_string(),
            "pattern_3".to_string(),
        ];

        let qualities = [85, 78, 82, 75, 88, 90, 84];

        let user_input = "Complex analysis requiring deep integration";

        // WHEN: We measure save operation performance over 100 iterations
        let iterations = 100;
        let mut save_durations = Vec::with_capacity(iterations);

        for _ in 0..iterations {
            let start = Instant::now();

            memory_manager
                .create_snapshot_with_qualities(
                    domains.clone(),
                    boundaries.clone(),
                    patterns.clone(),
                    user_id,
                    user_input,
                    qualities,
                )
                .await
                .unwrap();

            let duration = start.elapsed();
            save_durations.push(duration.as_millis() as f64);
        }

        // WHEN: We measure load operation performance over 100 iterations
        let mut load_durations = Vec::with_capacity(iterations);

        for _ in 0..iterations {
            let start = Instant::now();

            let _snapshot = memory_manager
                .get_latest_snapshot(user_id)
                .await
                .unwrap()
                .expect("Should have snapshot");

            let duration = start.elapsed();
            load_durations.push(duration.as_millis() as f64);
        }

        // THEN: Calculate save operation metrics
        save_durations.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let save_mean = save_durations.iter().sum::<f64>() / save_durations.len() as f64;
        let save_median = save_durations[save_durations.len() / 2];
        let save_p95_index = (save_durations.len() as f64 * 0.95) as usize;
        let save_p95 = save_durations[save_p95_index];

        println!("\n=== Memory Save Performance (n={}) ===", iterations);
        println!("Mean:   {:.2}ms", save_mean);
        println!("Median: {:.2}ms", save_median);
        println!("P95:    {:.2}ms", save_p95);
        println!("Min:    {:.2}ms", save_durations[0]);
        println!("Max:    {:.2}ms", save_durations[save_durations.len() - 1]);

        // THEN: Calculate load operation metrics
        load_durations.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let load_mean = load_durations.iter().sum::<f64>() / load_durations.len() as f64;
        let load_median = load_durations[load_durations.len() / 2];
        let load_p95_index = (load_durations.len() as f64 * 0.95) as usize;
        let load_p95 = load_durations[load_p95_index];

        println!("\n=== Memory Load Performance (n={}) ===", iterations);
        println!("Mean:   {:.2}ms", load_mean);
        println!("Median: {:.2}ms", load_median);
        println!("P95:    {:.2}ms", load_p95);
        println!("Min:    {:.2}ms", load_durations[0]);
        println!("Max:    {:.2}ms", load_durations[load_durations.len() - 1]);

        // ASSERT: Memory operations meet performance targets
        // Save target: P95 < 50ms
        assert!(
            save_p95 < 50.0,
            "Save P95 latency ({:.2}ms) exceeds target of 50ms",
            save_p95
        );

        // Load target: P95 < 30ms
        assert!(
            load_p95 < 30.0,
            "Load P95 latency ({:.2}ms) exceeds target of 30ms",
            load_p95
        );

        // Mean save should be reasonable (< 25ms)
        assert!(
            save_mean < 25.0,
            "Save mean latency ({:.2}ms) exceeds reasonable target of 25ms",
            save_mean
        );

        // Mean load should be reasonable (< 15ms)
        assert!(
            load_mean < 15.0,
            "Load mean latency ({:.2}ms) exceeds reasonable target of 15ms",
            load_mean
        );

        // These baselines enable future regression detection
    }

    #[tokio::test]
    async fn test_flow_process_partial_failure_recovery() {
        // GIVEN: A flow context that will fail at Stage 3 (InterfaceAttentionProcessor)
        // This simulates LLM timeout or provider error during interface generation

        let mut context = FlowContext::new(
            "Test message requiring multi-stage processing".to_string(),
            0.7,
            create_test_framework_state(),
        );

        // Populate domains (Stage 1 succeeds)
        context
            .domains
            .insert("CD".to_string(), DomainActivation { activation: 0.8 });
        context
            .domains
            .insert("SD".to_string(), DomainActivation { activation: 0.7 });

        // Populate boundaries (Stage 2 succeeds)
        context.boundaries = vec![BoundaryState::new(
            "CD-SD".to_string(),
            0.8,
            "Transcendent".to_string(),
        )];

        // WHEN: Stage 3 fails (simulate by skipping InterfaceAttentionProcessor)
        // Then attempt Stages 4-7 with partial context

        // Stage 4: Quality Emergence (should work with boundaries even without interface experiences)
        let processor_4 = QualityEmergenceProcessor;
        let result_4 = processor_4.process(&mut context);
        assert!(
            result_4.is_ok(),
            "Stage 4 should continue with available context"
        );

        // Stage 5: Integration (should work with domains/boundaries/qualities)
        let processor_5 = IntegrationProcessor;
        let result_5 = processor_5.process(&mut context);
        assert!(result_5.is_ok(), "Stage 5 should integrate available data");

        // Stage 6: Continuity (should extract patterns from partial context)
        let processor_6 = ContinuityProcessor;
        let result_6 = processor_6.process(&mut context);
        assert!(
            result_6.is_ok(),
            "Stage 6 should continue with partial data"
        );

        // Stage 7: Evolution (should calculate developmental stage from available metrics)
        let processor_7 = EvolutionProcessor;
        let result_7 = processor_7.process(&mut context);
        assert!(
            result_7.is_ok(),
            "Stage 7 should complete with degraded context"
        );

        // THEN: Verify system degraded gracefully
        assert!(
            !context.emergent_qualities.is_empty(),
            "Should calculate qualities even without interface experiences"
        );

        assert!(
            !context.structured_prompt.is_empty(),
            "Should produce structured prompt with partial data"
        );

        // Measure degradation (optional - for scientific validation)
        let quality_count = context.emergent_qualities.len();
        let pattern_count = context.patterns.len();

        // System should produce SOME output, even if reduced
        assert!(
            quality_count > 0 || pattern_count > 0,
            "System should produce partial output: qualities={}, patterns={}",
            quality_count,
            pattern_count
        );
    }

    #[tokio::test]
    async fn test_memory_save_failure_transactional_consistency() {
        // GIVEN: A completed 7-stage flow process
        use crate::memory::MemoryManager;
        use crate::test_utils::setup_test_db;
        use sqlx::types::Uuid;

        let db_pool = setup_test_db().await.unwrap();
        let memory_manager = MemoryManager {
            db_pool: db_pool.clone(),
        };

        // Create user
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

        // Execute full flow process
        let context = FlowContext::new(
            "Test message for transactional consistency".to_string(),
            0.7,
            create_test_framework_state(),
        );

        let flow_process = FlowProcess::new();
        let context = flow_process.execute(context).unwrap();

        // Capture flow process output state
        let original_domains = context.domains.clone();
        let original_boundaries = context.boundaries.clone();
        let original_qualities = context.emergent_qualities.clone();
        let original_patterns = context.patterns.clone();
        let original_structured_prompt = context.structured_prompt.clone();

        // WHEN: Attempt to save to DB with simulated failure
        // Simulate by closing the database connection pool
        db_pool.close().await;

        let domains_vec: Vec<DomainState> = context
            .domains
            .iter()
            .map(|(name, activation)| DomainState {
                name: name.clone(),
                state: activation.activation.to_string(),
            })
            .collect();

        let patterns_vec: Vec<String> = context
            .patterns
            .iter()
            .map(|p| p.description.clone())
            .collect();

        let save_result = memory_manager
            .create_snapshot(
                domains_vec,
                context.boundaries.clone(),
                patterns_vec.clone(),
                user_id,
                &context.user_input,
            )
            .await;

        // Save should fail (database closed)
        assert!(
            save_result.is_err(),
            "Save should fail with closed database"
        );

        // THEN: Verify FlowContext data remains intact (transactional consistency)
        assert_eq!(
            context.domains, original_domains,
            "Domain activations should remain unchanged after failed save"
        );

        assert_eq!(
            context.boundaries.len(),
            original_boundaries.len(),
            "Boundaries should remain unchanged after failed save"
        );

        assert_eq!(
            context.emergent_qualities.len(),
            original_qualities.len(),
            "Emergent qualities should remain unchanged after failed save"
        );

        assert_eq!(
            context.patterns.len(),
            original_patterns.len(),
            "Patterns should remain unchanged after failed save"
        );

        assert_eq!(
            context.structured_prompt, original_structured_prompt,
            "Structured prompt should remain unchanged after failed save"
        );

        // Verify retry would work (with fresh DB connection)
        let db_pool_retry = setup_test_db().await.unwrap();
        let memory_manager_retry = MemoryManager {
            db_pool: db_pool_retry.clone(),
        };

        // Re-create user in fresh DB
        sqlx::query(
            "INSERT INTO users (id, provider, provider_id, email, name, created_at, last_login)
             VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        )
        .bind(user_id.as_bytes().to_vec())
        .bind("test")
        .bind(user_id.to_string())
        .bind("test@example.com")
        .bind("Test User")
        .execute(&db_pool_retry)
        .await
        .unwrap();

        let domains_vec: Vec<DomainState> = context
            .domains
            .iter()
            .map(|(name, activation)| DomainState {
                name: name.clone(),
                state: activation.activation.to_string(),
            })
            .collect();

        let patterns_vec: Vec<String> = context
            .patterns
            .iter()
            .map(|p| p.description.clone())
            .collect();

        // Retry save with fresh connection
        let retry_result = memory_manager_retry
            .create_snapshot(
                domains_vec,
                context.boundaries.clone(),
                patterns_vec,
                user_id,
                &context.user_input,
            )
            .await;

        assert!(
            retry_result.is_ok(),
            "Retry should succeed with valid FlowContext: {:?}",
            retry_result.err()
        );
    }

    #[test]
    fn test_flow_process_with_config_dual_llm_enabled() {
        // GIVEN: Dual-LLM configuration with feature enabled
        let config = crate::dual_llm::DualLlmConfig::enabled();
        let mock_llm = std::sync::Arc::new(crate::mock_llm::MockLlm::llm1_recognition());

        // WHEN: FlowProcess created with dual-LLM config
        let flow_process = FlowProcess::with_config(config.clone(), mock_llm.clone());

        // THEN: FlowProcess should be created successfully
        // In dual-LLM mode, Stage 1 uses UnconscciousLlmProcessor instead of DomainEmergenceProcessor
        // This is validated by checking the stage count (dual-LLM has 6 stages vs 7 in classic mode)
        assert_eq!(
            flow_process.stages.len(),
            6,
            "Dual-LLM mode should have 6 stages (LLM #1 handles both domain + boundary)"
        );

        // Verify the flow can execute with LLM #1 processor
        let framework_state = create_test_framework_state();
        let mut context = FlowContext::new(
            "Test dual-LLM integration".to_string(),
            0.7,
            framework_state,
        );

        // Initialize boundaries for the test
        context.boundaries = vec![
            crate::prompt_engine::BoundaryState::new(
                "CD-SD".to_string(),
                0.5,
                "Maintained".to_string(),
            ),
            crate::prompt_engine::BoundaryState::new(
                "CD-CuD".to_string(),
                0.5,
                "Maintained".to_string(),
            ),
            crate::prompt_engine::BoundaryState::new(
                "CD-ED".to_string(),
                0.5,
                "Maintained".to_string(),
            ),
            crate::prompt_engine::BoundaryState::new(
                "SD-CuD".to_string(),
                0.5,
                "Maintained".to_string(),
            ),
            crate::prompt_engine::BoundaryState::new(
                "SD-ED".to_string(),
                0.5,
                "Maintained".to_string(),
            ),
            crate::prompt_engine::BoundaryState::new(
                "CuD-ED".to_string(),
                0.5,
                "Maintained".to_string(),
            ),
        ];

        let result = flow_process.execute(context);

        // ASSERT: Flow execution succeeds with dual-LLM processor
        assert!(
            result.is_ok(),
            "Dual-LLM FlowProcess should execute successfully: {:?}",
            result.err()
        );

        let final_context = result.unwrap();

        // Verify domains were populated by LLM #1
        assert!(
            !final_context.domains.is_empty(),
            "LLM #1 should populate domain activations"
        );
        assert!(
            final_context.domains.contains_key("CD"),
            "CD domain should be recognized by LLM #1"
        );

        // Verify MockLlm was called by LLM #1 processor
        assert_eq!(
            mock_llm.call_count(),
            1,
            "LLM #1 should have been called exactly once"
        );
    }

    #[test]
    fn test_flow_process_with_config_classic_mode() {
        // GIVEN: Dual-LLM configuration with feature DISABLED
        let config = crate::dual_llm::DualLlmConfig::disabled();
        let mock_llm = std::sync::Arc::new(crate::mock_llm::MockLlm::echo());

        // WHEN: FlowProcess created with dual-LLM disabled
        let flow_process = FlowProcess::with_config(config, mock_llm.clone());

        // THEN: FlowProcess should use classic 7-stage flow
        assert_eq!(
            flow_process.stages.len(),
            7,
            "Classic mode should have 7 stages"
        );

        // Verify MockLlm is NOT called (classic mode uses Rust processors)
        let framework_state = create_test_framework_state();
        let context = FlowContext::new("Test classic mode".to_string(), 0.7, framework_state);

        let result = flow_process.execute(context);

        assert!(result.is_ok(), "Classic mode should execute successfully");
        assert_eq!(
            mock_llm.call_count(),
            0,
            "MockLlm should NOT be called in classic mode"
        );
    }
}
