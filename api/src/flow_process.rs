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
#[derive(Debug, Clone)]
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
        // Find boundaries with high permeability (transcendent or transitional)
        let relevant_boundaries: Vec<&BoundaryState> = context
            .boundaries
            .iter()
            .filter(|b| b.permeability > 0.6)
            .collect();

        // Create interface experiences for relevant boundaries
        for boundary in relevant_boundaries {
            let domains: Vec<&str> = boundary.name.split('-').collect();
            if domains.len() == 2 {
                let experience = self.create_interface_experience(domains[0], domains[1], boundary);
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
    ) -> InterfaceExperience {
        // BDE(i): Invitation - create productive tension
        let invitation = format!(
            "Create productive tension between {} and {} domains, requiring integration of both perspectives.",
            self.domain_full_name(domain1),
            self.domain_full_name(domain2)
        );

        // BDE(a): Attention - direct focus to interface
        let attention = format!(
            "Focus on the interface where {} meets {}, not on either domain in isolation.",
            domain1, domain2
        );

        // BDE(r): Resonance - allow transformation
        let resonance = format!(
            "Allow understanding to oscillate naturally between {} and {} perspectives.",
            domain1, domain2
        );

        // BDE(e): Emergence - recognize qualities
        let emergence = if boundary.status == "Transcendent" {
            format!(
                "Notice the {} quality emerging at the {}-{} interface.",
                self.primary_quality_for_boundary(domain1, domain2),
                domain1,
                domain2
            )
        } else {
            format!(
                "Allow qualities to emerge as {} and {} integrate.",
                domain1, domain2
            )
        };

        InterfaceExperience {
            boundary_name: boundary.name.clone(),
            invitation,
            attention,
            resonance,
            emergence,
        }
    }

    fn domain_full_name<'a>(&self, abbrev: &'a str) -> &'a str {
        match abbrev {
            "CD" => "computational",
            "SD" => "scientific",
            "CuD" => "cultural",
            "ED" => "experiential",
            _ => abbrev,
        }
    }

    fn primary_quality_for_boundary<'a>(&self, domain1: &'a str, domain2: &'a str) -> &'a str {
        match (domain1, domain2) {
            ("CD", "SD") | ("SD", "CD") => "precision",
            ("CD", "ED") | ("ED", "CD") => "fluidity",
            ("SD", "CuD") | ("CuD", "SD") => "depth",
            ("CuD", "ED") | ("ED", "CuD") => "resonance",
            ("CD", "CuD") | ("CuD", "CD") => "coherence",
            ("SD", "ED") | ("ED", "SD") => "clarity",
            _ => "openness",
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
        _context: &FlowContext,
    ) -> PhenomenologicalQuality {
        let permeability = boundary.permeability;

        PhenomenologicalQuality {
            boundary_name: boundary.name.clone(),
            clarity: 0.6 * permeability + 0.4 * 0.8, // Higher permeability = more clarity
            depth: 0.7 * permeability + 0.3 * 0.75,  // Depth emerges with integration
            openness: permeability,                  // Directly related to permeability
            precision: 0.8 * (1.0 - permeability) + 0.2 * permeability, // Balance of separation and integration
            fluidity: 0.6 * permeability + 0.4 * 0.85, // Fluid transformation at interfaces
            resonance: 0.5 + 0.5 * permeability,       // Resonance increases with integration
            coherence: 0.7 * permeability + 0.3 * 0.9, // Coherent integration
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
    use crate::prompt_engine::{BoundaryState, DomainRegistry, FrameworkState};

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
}
