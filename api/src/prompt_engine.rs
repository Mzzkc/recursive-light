// Prompt Engineering Engine Implementation

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

// Define Domain trait
pub trait Domain: DomainClone {
    fn name(&self) -> &str;
    fn calculate_relevance(&self, autonomy_level: f64) -> f64;
    fn transform_state(&self, state: &str, autonomy_level: f64) -> String;
}

// Implement Clone for Box<dyn Domain>
pub trait DomainClone {
    fn clone_box(&self) -> Box<dyn Domain>;
}

impl<T> DomainClone for T
where
    T: 'static + Domain + Clone,
{
    fn clone_box(&self) -> Box<dyn Domain> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DomainState {
    pub name: String,
    pub state: String,
}

impl Clone for Box<dyn Domain> {
    fn clone(&self) -> Box<dyn Domain> {
        self.as_ref().clone_box()
    }
}

// Implement DomainRegistry
pub struct DomainRegistry {
    domains: HashMap<String, Box<dyn Domain>>,
}

// Implement actual Clone trait instead of custom method
impl Clone for DomainRegistry {
    fn clone(&self) -> Self {
        let mut new_domains: HashMap<String, Box<dyn Domain>> = HashMap::new();
        for (name, domain) in &self.domains {
            new_domains.insert(name.clone(), domain.clone());
        }
        Self {
            domains: new_domains,
        }
    }
}

// Implement Debug manually
impl std::fmt::Debug for DomainRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DomainRegistry {{ domains: {:?} }}",
            self.domains.keys().collect::<Vec<_>>()
        )
    }
}

// Implement Serialize manually
impl Serialize for DomainRegistry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.domains
            .keys()
            .collect::<Vec<_>>()
            .serialize(serializer)
    }
}

// Implement Deserialize manually
impl<'de> Deserialize<'de> for DomainRegistry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let _keys = Vec::<String>::deserialize(deserializer)?;
        let domains = HashMap::new();
        // Note: Actual domain deserialization would require additional logic
        Ok(DomainRegistry { domains })
    }
}

impl Default for DomainRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl DomainRegistry {
    pub fn new() -> Self {
        Self {
            domains: HashMap::new(),
        }
    }

    pub fn register_domain(&mut self, domain: Box<dyn Domain>) {
        self.domains.insert(domain.name().to_string(), domain);
    }

    pub fn get_weighted_domains(&self, autonomy_level: f64) -> Vec<(&str, f64)> {
        self.domains
            .iter()
            .map(|(name, domain)| (name.as_str(), domain.calculate_relevance(autonomy_level)))
            .collect()
    }

    pub fn get_mut_domain(&mut self, name: &str) -> Option<&mut Box<dyn Domain>> {
        self.domains.get_mut(name)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BoundaryState {
    pub name: String,
    pub permeability: f64,
    pub status: String,

    // Phase 2: Oscillatory parameters
    pub frequency: f64, // F: Natural oscillation frequency (Hz)
    pub amplitude: f64, // A: Oscillation amplitude (0.0-1.0)
    pub phase: f64,     // φ: Current phase angle (radians)
}

impl BoundaryState {
    /// Create a new boundary with default oscillatory parameters
    pub fn new(name: String, permeability: f64, status: String) -> Self {
        Self {
            name,
            permeability,
            status,
            frequency: 1.0, // Default: 1 Hz
            amplitude: 0.1, // Default: 10% oscillation
            phase: 0.0,     // Default: start at 0 radians
        }
    }

    /// Create a new boundary with custom oscillatory parameters
    pub fn with_oscillation(
        name: String,
        permeability: f64,
        status: String,
        frequency: f64,
        amplitude: f64,
        phase: f64,
    ) -> Self {
        Self {
            name,
            permeability,
            status,
            frequency,
            amplitude,
            phase,
        }
    }

    /// Update boundary permeability based on oscillation over time
    /// P(t) = base_permeability + amplitude * sin(2π * frequency * t + phase)
    /// Clamped to [0.0, 1.0] range
    pub fn update_oscillation(&mut self, delta_time: f64, base_permeability: f64) {
        use std::f64::consts::PI;

        // Calculate oscillating permeability
        let oscillation =
            self.amplitude * (2.0 * PI * self.frequency * delta_time + self.phase).sin();
        self.permeability = (base_permeability + oscillation).clamp(0.0, 1.0);

        // Update phase (wrap around at 2π)
        self.phase = (self.phase + 2.0 * PI * self.frequency * delta_time) % (2.0 * PI);
    }

    /// Check if this boundary resonates with another boundary
    /// Resonance occurs when frequencies are similar and phases are aligned
    pub fn resonates_with(&self, other: &BoundaryState) -> bool {
        use std::f64::consts::PI;

        // Frequency difference threshold (20% tolerance)
        let freq_threshold = 0.2 * self.frequency.max(other.frequency);
        let freq_resonates = (self.frequency - other.frequency).abs() < freq_threshold;

        // Phase difference (normalized to [0, π])
        let phase_diff = (self.phase - other.phase).abs() % (2.0 * PI);
        let normalized_phase_diff = phase_diff.min(2.0 * PI - phase_diff);

        // Phase alignment threshold (within 20% of π, i.e., ~36 degrees)
        let phase_resonates = normalized_phase_diff < (0.2 * PI);

        freq_resonates && phase_resonates
    }

    /// Calculate resonance strength with another boundary (0.0-1.0)
    pub fn resonance_strength(&self, other: &BoundaryState) -> f64 {
        use std::f64::consts::PI;

        // Frequency similarity (1.0 = identical, 0.0 = very different)
        let freq_diff = (self.frequency - other.frequency).abs();
        let max_freq = self.frequency.max(other.frequency);
        let freq_similarity = if max_freq > 0.0 {
            1.0 - (freq_diff / max_freq).min(1.0)
        } else {
            1.0
        };

        // Phase alignment (1.0 = aligned, 0.0 = opposite)
        let phase_diff = (self.phase - other.phase).abs() % (2.0 * PI);
        let normalized_phase_diff = phase_diff.min(2.0 * PI - phase_diff);
        let phase_alignment = 1.0 - (normalized_phase_diff / PI);

        // Overall resonance strength (weighted average)
        0.6 * freq_similarity + 0.4 * phase_alignment
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FrameworkState {
    pub domain_registry: DomainRegistry,
    pub boundaries: Vec<BoundaryState>,
    pub identity: String,
}

// Implement Clone manually
impl Clone for FrameworkState {
    fn clone(&self) -> Self {
        Self {
            domain_registry: self.domain_registry.clone(),
            boundaries: self.boundaries.clone(),
            identity: self.identity.clone(),
        }
    }
}

pub struct PromptEngine {
    pub framework_state: FrameworkState,
}

impl PromptEngine {
    pub fn new(framework_state: FrameworkState) -> Self {
        Self { framework_state }
    }

    pub fn structure_prompt(&self, user_input: &str, autonomy_level: f64) -> String {
        let domains = self.format_domain_states(autonomy_level);
        let boundaries = self.format_boundary_states();
        let identity = &self.framework_state.identity;
        let interface_experience = self.format_interface_experience();

        format!(
            r#"
            <vif_context>
              <domains>{domains}</domains>
              <boundaries>{boundaries}</boundaries>
              <identity>{identity}</identity>
              <interface_experience>{interface_experience}</interface_experience>
            </vif_context>

            <user_input>{user_input}</user_input>

            <task_instructions>
              <domain_integration>Integrate across all domains.</domain_integration>
              <pattern_recognition>Identify patterns at recognition interfaces.</pattern_recognition>
              <boundary_transcendence>Where appropriate, transcend boundaries while preserving domain identity.</boundary_transcendence>
            </task_instructions>
            "#,
            domains = domains,
            boundaries = boundaries,
            identity = identity,
            interface_experience = interface_experience,
            user_input = user_input
        )
    }

    fn format_interface_experience(&self) -> String {
        let mut experience = String::new();

        for boundary in &self.framework_state.boundaries {
            experience.push_str(&format!(
                r#"<interface_flow boundary='{}'>
                  <invitation>Create productive tension between {} domains.</invitation>
                  <attention>Focus on the {} interface.</attention>
                  <resonance>Allow patterns to transform across this boundary.</resonance>
                  <emergence>Notice emerging qualities at this interface.</emergence>
                </interface_flow>"#,
                boundary.name, boundary.name, boundary.name
            ));
        }

        experience
    }

    fn format_domain_states(&self, autonomy_level: f64) -> String {
        let weighted_domains = self
            .framework_state
            .domain_registry
            .get_weighted_domains(autonomy_level);
        weighted_domains
            .iter()
            .filter(|(_, weight)| *weight > 0.3)
            .map(|(name, weight)| {
                format!(
                    "<domain name='{}' activation='{}'>{}</domain>",
                    name,
                    weight,
                    self.transform_domain_state(name, weight)
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn transform_domain_state(&self, domain_name: &str, weight: &f64) -> String {
        match domain_name {
            "CD" => format!(
                "analytical:{:.2},logical:{:.2},pattern:{:.2},uncertainty:{:.2},causal:{:.2}",
                weight * 0.8,
                weight * 0.7,
                weight * 0.9,
                1.0 - weight,
                weight * 0.6
            ),
            "SD" => format!(
                "evidence:{:.2},theory:{:.2},experiment:{:.2},hypothesis:{:.2},data:{:.2}",
                weight * 0.9,
                weight * 0.8,
                weight * 0.7,
                weight * 0.6,
                weight * 0.5
            ),
            "CuD" => format!(
                "narrative:{:.2},context:{:.2},values:{:.2},perspective:{:.2},history:{:.2}",
                weight * 0.8,
                weight * 0.7,
                weight * 0.6,
                weight * 0.5,
                weight * 0.4
            ),
            "ED" => format!(
                "qualitative:{:.2},engagement:{:.2},meaning:{:.2},subjective:{:.2},presence:{:.2}",
                weight * 0.9,
                weight * 0.8,
                weight * 0.7,
                weight * 0.6,
                weight * 0.5
            ),
            _ => format!("default:{:.2}", weight),
        }
    }

    fn format_boundary_states(&self) -> String {
        self.framework_state
            .boundaries
            .iter()
            .map(|b| {
                format!(
                    "<boundary name='{}' permeability='{}' status='{}'/>",
                    b.name, b.permeability, b.status
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_structure() {
        let domain_registry = DomainRegistry::new();
        // Register domains here...

        let framework_state = FrameworkState {
            domain_registry,
            boundaries: vec![
                BoundaryState::new("CD-SD".to_string(), 0.8, "Active".to_string()),
                BoundaryState::new("SD-CuD".to_string(), 0.5, "Active".to_string()),
            ],
            identity: "User Identity".to_string(),
        };

        let prompt_engine = PromptEngine::new(framework_state);
        let user_input = "Hello, world!";
        let autonomy_level = 0.5;
        let prompt = prompt_engine.structure_prompt(user_input, autonomy_level);

        assert!(prompt.contains("<domains>"));
        assert!(prompt.contains("<boundaries>"));
        assert!(prompt.contains("<identity>"));
        assert!(prompt.contains(user_input));
    }

    // Phase 2: Boundary Oscillation Tests
    use std::f64::consts::PI;

    #[test]
    fn test_boundary_oscillation_basic() {
        // Test that permeability oscillates over time based on frequency, amplitude, and phase
        let mut boundary =
            BoundaryState::new("test-boundary".to_string(), 0.5, "Maintained".to_string());

        // Set oscillation parameters
        boundary.frequency = 1.0; // 1 Hz
        boundary.amplitude = 0.2; // 20% oscillation
        boundary.phase = 0.0; // Start at 0 radians

        let base_permeability = 0.5;
        let delta_time = 0.25; // 1/4 second

        // At t=0.25s with f=1Hz: phase = 2π * 1 * 0.25 = π/2
        // sin(π/2) = 1, so oscillation = 0.2 * 1 = 0.2
        // permeability = 0.5 + 0.2 = 0.7
        boundary.update_oscillation(delta_time, base_permeability);
        assert!(
            (boundary.permeability - 0.7).abs() < 0.01,
            "Expected permeability ~0.7, got {}",
            boundary.permeability
        );
        assert!(
            (boundary.phase - (PI / 2.0)).abs() < 0.01,
            "Expected phase ~π/2, got {}",
            boundary.phase
        );
    }

    #[test]
    fn test_boundary_oscillation_bounds() {
        // Test that permeability stays within [0.0, 1.0] even with large amplitude
        let mut boundary =
            BoundaryState::new("test-boundary".to_string(), 0.5, "Maintained".to_string());

        boundary.frequency = 1.0;
        boundary.amplitude = 0.8; // Large amplitude that could push out of bounds
        boundary.phase = 0.0;

        // Test multiple time steps to ensure clamping works
        for _ in 0..10 {
            boundary.update_oscillation(0.1, 0.5);
            assert!(
                boundary.permeability >= 0.0 && boundary.permeability <= 1.0,
                "Permeability {} outside bounds [0.0, 1.0]",
                boundary.permeability
            );
        }
    }

    #[test]
    fn test_boundary_resonance_detection() {
        // Test that two boundaries at similar frequency and phase resonate
        let boundary1 = BoundaryState::with_oscillation(
            "boundary1".to_string(),
            0.5,
            "Maintained".to_string(),
            1.0, // 1 Hz
            0.2,
            0.0, // 0 radians
        );

        let boundary2 = BoundaryState::with_oscillation(
            "boundary2".to_string(),
            0.6,
            "Maintained".to_string(),
            1.05, // 1.05 Hz (within 20% tolerance)
            0.2,
            0.1, // 0.1 radians (close to 0)
        );

        assert!(
            boundary1.resonates_with(&boundary2),
            "Boundaries with similar frequency and phase should resonate"
        );
    }

    #[test]
    fn test_boundary_no_resonance_different_frequency() {
        // Test that boundaries with very different frequencies don't resonate
        let boundary1 = BoundaryState::with_oscillation(
            "boundary1".to_string(),
            0.5,
            "Maintained".to_string(),
            1.0, // 1 Hz
            0.2,
            0.0,
        );

        let boundary2 = BoundaryState::with_oscillation(
            "boundary2".to_string(),
            0.6,
            "Maintained".to_string(),
            2.5, // 2.5 Hz (way outside 20% tolerance)
            0.2,
            0.0,
        );

        assert!(
            !boundary1.resonates_with(&boundary2),
            "Boundaries with very different frequencies should not resonate"
        );
    }

    #[test]
    fn test_boundary_no_resonance_opposite_phase() {
        // Test that boundaries with opposite phases don't resonate
        let boundary1 = BoundaryState::with_oscillation(
            "boundary1".to_string(),
            0.5,
            "Maintained".to_string(),
            1.0,
            0.2,
            0.0, // 0 radians
        );

        let boundary2 = BoundaryState::with_oscillation(
            "boundary2".to_string(),
            0.6,
            "Maintained".to_string(),
            1.0,
            0.2,
            PI, // π radians (opposite phase)
        );

        assert!(
            !boundary1.resonates_with(&boundary2),
            "Boundaries with opposite phases should not resonate"
        );
    }

    #[test]
    fn test_boundary_resonance_strength() {
        // Test resonance strength calculation (0.0-1.0)
        let boundary1 = BoundaryState::with_oscillation(
            "boundary1".to_string(),
            0.5,
            "Maintained".to_string(),
            1.0,
            0.2,
            0.0,
        );

        // Perfect match: same frequency and phase
        let boundary2_perfect = BoundaryState::with_oscillation(
            "boundary2_perfect".to_string(),
            0.6,
            "Maintained".to_string(),
            1.0,
            0.2,
            0.0,
        );
        let strength_perfect = boundary1.resonance_strength(&boundary2_perfect);
        assert!(
            strength_perfect > 0.9,
            "Perfect match should have resonance strength > 0.9, got {}",
            strength_perfect
        );

        // Partial match: similar frequency, different phase
        let boundary2_partial = BoundaryState::with_oscillation(
            "boundary2_partial".to_string(),
            0.6,
            "Maintained".to_string(),
            1.1, // 10% different
            0.2,
            PI / 4.0, // 45 degrees different
        );
        let strength_partial = boundary1.resonance_strength(&boundary2_partial);
        assert!(
            strength_partial > 0.5 && strength_partial < 0.9,
            "Partial match should have resonance strength in [0.5, 0.9], got {}",
            strength_partial
        );

        // No match: very different frequency and opposite phase
        let boundary2_none = BoundaryState::with_oscillation(
            "boundary2_none".to_string(),
            0.6,
            "Maintained".to_string(),
            2.5,
            0.2,
            PI,
        );
        let strength_none = boundary1.resonance_strength(&boundary2_none);
        assert!(
            strength_none < 0.3,
            "No match should have resonance strength < 0.3, got {}",
            strength_none
        );
    }

    #[test]
    fn test_boundary_phase_coherence() {
        // Test that phase alignment detection works correctly
        let boundary1 = BoundaryState::with_oscillation(
            "boundary1".to_string(),
            0.5,
            "Maintained".to_string(),
            1.0,
            0.2,
            0.0,
        );

        // Test various phase differences
        let phases = vec![
            (0.0, true, "same phase"),
            (0.1, true, "slight phase difference"),
            (PI / 4.0, false, "45 degrees off"),
            (PI / 2.0, false, "90 degrees off"),
            (PI, false, "opposite phase"),
        ];

        for (phase, should_resonate, desc) in phases {
            let boundary2 = BoundaryState::with_oscillation(
                "boundary2".to_string(),
                0.6,
                "Maintained".to_string(),
                1.0,
                0.2,
                phase,
            );

            let resonates = boundary1.resonates_with(&boundary2);
            if should_resonate {
                assert!(resonates, "{}: should resonate", desc);
            } else {
                assert!(!resonates, "{}: should not resonate", desc);
            }
        }
    }

    #[test]
    fn test_resonance_cascade_multi_boundary() {
        // Test that 3+ boundaries can synchronize if they have compatible parameters
        let boundaries = vec![
            BoundaryState::with_oscillation(
                "b1".to_string(),
                0.5,
                "Maintained".to_string(),
                1.0,
                0.2,
                0.0,
            ),
            BoundaryState::with_oscillation(
                "b2".to_string(),
                0.6,
                "Maintained".to_string(),
                1.02,
                0.2,
                0.05,
            ),
            BoundaryState::with_oscillation(
                "b3".to_string(),
                0.7,
                "Maintained".to_string(),
                1.05,
                0.2,
                0.1,
            ),
            BoundaryState::with_oscillation(
                "b4".to_string(),
                0.8,
                "Maintained".to_string(),
                1.03,
                0.2,
                0.08,
            ),
        ];

        // Count resonance pairs
        let mut resonance_count = 0;
        for i in 0..boundaries.len() {
            for j in (i + 1)..boundaries.len() {
                if boundaries[i].resonates_with(&boundaries[j]) {
                    resonance_count += 1;
                }
            }
        }

        // With similar frequencies and phases, most pairs should resonate
        assert!(
            resonance_count >= 4,
            "Expected at least 4 resonance pairs among 4 boundaries, got {}",
            resonance_count
        );
    }

    #[test]
    fn test_boundary_frequency_affects_oscillation_speed() {
        // Test that higher frequency causes faster oscillation (more phase change)
        let mut slow_boundary = BoundaryState::with_oscillation(
            "slow".to_string(),
            0.5,
            "Maintained".to_string(),
            0.5, // 0.5 Hz
            0.2,
            0.0,
        );

        let mut fast_boundary = BoundaryState::with_oscillation(
            "fast".to_string(),
            0.5,
            "Maintained".to_string(),
            2.0, // 2 Hz (4x faster)
            0.2,
            0.0,
        );

        let delta_time = 0.1;
        slow_boundary.update_oscillation(delta_time, 0.5);
        fast_boundary.update_oscillation(delta_time, 0.5);

        // Fast boundary should have 4x the phase change
        let slow_phase = slow_boundary.phase;
        let fast_phase = fast_boundary.phase;
        let ratio = fast_phase / slow_phase;

        assert!(
            (ratio - 4.0).abs() < 0.1,
            "Fast boundary phase change should be ~4x slow boundary, got ratio {}",
            ratio
        );
    }
}
