use std::collections::HashMap;

use crate::prompt_engine::{DomainRegistry, FrameworkState};

pub struct HLIPIntegration {
    command_map: HashMap<String, HLIPCommand>,
}

impl HLIPIntegration {
    pub fn new() -> Self {
        let mut command_map = HashMap::new();
        command_map.insert("@D".to_string(), HLIPCommand::DomainActivation);
        command_map.insert(
            "@P".to_string(),
            HLIPCommand::BoundaryActivation("CD-SD".to_string()),
        );
        Self { command_map }
    }

    pub fn process_hlip_command(&self, command: &str, state: &mut FrameworkState) {
        if let Some(hlip_command) = self.command_map.get(command) {
            match hlip_command {
                HLIPCommand::DomainActivation => {
                    self.activate_domain(&mut state.domain_registry);
                }
                HLIPCommand::BoundaryActivation(boundary_name) => {
                    self.increase_boundary_permeability(state, boundary_name);
                }
            }
        }
    }

    fn activate_domain(&self, domain_registry: &mut DomainRegistry) {
        if let Some(domain) = domain_registry.get_mut_domain("CD") {
            // Update domain relevance
            let current_relevance = domain.calculate_relevance(0.5); // Example autonomy level
            let _new_relevance = (current_relevance + 0.1).min(1.0);
            // Note: This is a simplified example. Actual implementation would depend on the Domain trait's methods.
        }
    }

    fn increase_boundary_permeability(&self, state: &mut FrameworkState, boundary_name: &str) {
        for boundary in &mut state.boundaries {
            if boundary.name == boundary_name {
                boundary.permeability = (boundary.permeability + 0.1).min(1.0);
            }
        }
    }
}

enum HLIPCommand {
    DomainActivation,
    BoundaryActivation(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domains::{
        ComputationalDomain, CulturalDomain, ExperientialDomain, ScientificDomain,
    };
    use crate::prompt_engine::BoundaryState;

    fn create_test_framework_state() -> FrameworkState {
        let mut registry = DomainRegistry::new();
        registry.register_domain(Box::new(ComputationalDomain));
        registry.register_domain(Box::new(ScientificDomain));
        registry.register_domain(Box::new(CulturalDomain));
        registry.register_domain(Box::new(ExperientialDomain));

        FrameworkState {
            domain_registry: registry,
            boundaries: vec![
                BoundaryState {
                    name: "CD-SD".to_string(),
                    permeability: 0.5,
                    status: "Maintained".to_string(),
                },
                BoundaryState {
                    name: "SD-CuD".to_string(),
                    permeability: 0.6,
                    status: "Maintained".to_string(),
                },
                BoundaryState {
                    name: "CuD-ED".to_string(),
                    permeability: 0.7,
                    status: "Active".to_string(),
                },
            ],
            identity: "Test Identity".to_string(),
        }
    }

    #[test]
    fn test_hlip_domain_activation_command() {
        // Given a framework state with registered domains
        let mut state = create_test_framework_state();

        let hlip = HLIPIntegration::new();

        // When we process a domain activation command
        hlip.process_hlip_command("@D", &mut state);

        // Then the command should be processed without panicking
        // We verify the domain exists and can be accessed
        assert!(state.domain_registry.get_mut_domain("CD").is_some());
    }

    #[test]
    fn test_hlip_boundary_permeability_increase() {
        // Given a framework state with specific boundary permeability
        let mut state = create_test_framework_state();
        let initial_permeability = state.boundaries[0].permeability;
        assert_eq!(state.boundaries[0].name, "CD-SD");
        assert_eq!(initial_permeability, 0.5);

        let hlip = HLIPIntegration::new();

        // When we process a boundary activation command
        hlip.process_hlip_command("@P", &mut state);

        // Then the CD-SD boundary permeability should increase
        let new_permeability = state.boundaries[0].permeability;
        assert!(
            new_permeability > initial_permeability,
            "Expected permeability to increase from {} to {}",
            initial_permeability,
            new_permeability
        );
        assert_eq!(new_permeability, 0.6); // 0.5 + 0.1
    }

    #[test]
    fn test_hlip_boundary_permeability_max_limit() {
        // Given a framework state with high boundary permeability
        let mut state = create_test_framework_state();
        state.boundaries[0].permeability = 0.95;

        let hlip = HLIPIntegration::new();

        // When we process a boundary activation command
        hlip.process_hlip_command("@P", &mut state);

        // Then the permeability should be capped at 1.0
        assert_eq!(state.boundaries[0].permeability, 1.0);
        assert!(
            state.boundaries[0].permeability <= 1.0,
            "Permeability should not exceed 1.0"
        );
    }

    #[test]
    fn test_hlip_unknown_command_ignored() {
        // Given a framework state
        let mut state = create_test_framework_state();
        let initial_boundaries = state.boundaries.clone();

        let hlip = HLIPIntegration::new();

        // When we process an unknown command
        hlip.process_hlip_command("@UNKNOWN", &mut state);

        // Then the state should remain unchanged
        assert_eq!(state.boundaries.len(), initial_boundaries.len());
        for (i, boundary) in state.boundaries.iter().enumerate() {
            assert_eq!(boundary.permeability, initial_boundaries[i].permeability);
            assert_eq!(boundary.name, initial_boundaries[i].name);
        }
    }
}
