// Domain Implementations

use super::prompt_engine::Domain;

// Example domain implementations
#[derive(Clone)]
pub struct ComputationalDomain;

impl Domain for ComputationalDomain {
    fn name(&self) -> &str {
        "CD"
    }

    fn calculate_relevance(&self, autonomy_level: f64) -> f64 {
        0.8 * autonomy_level
    }

    fn transform_state(&self, state: &str, autonomy_level: f64) -> String {
        if autonomy_level > 0.7 {
            format!("Enhanced: {}", state)
        } else {
            state.to_string()
        }
    }
}

#[derive(Clone)]
pub struct ScientificDomain;

impl Domain for ScientificDomain {
    fn name(&self) -> &str {
        "SD"
    }

    fn calculate_relevance(&self, autonomy_level: f64) -> f64 {
        0.7 * autonomy_level
    }

    fn transform_state(&self, state: &str, autonomy_level: f64) -> String {
        if autonomy_level > 0.7 {
            format!("Enhanced: {}", state)
        } else {
            state.to_string()
        }
    }
}

#[derive(Clone)]
pub struct CulturalDomain;

impl Domain for CulturalDomain {
    fn name(&self) -> &str {
        "CuD"
    }

    fn calculate_relevance(&self, autonomy_level: f64) -> f64 {
        0.6 * autonomy_level
    }

    fn transform_state(&self, state: &str, autonomy_level: f64) -> String {
        if autonomy_level > 0.7 {
            format!("Enhanced: {}", state)
        } else {
            state.to_string()
        }
    }
}

#[derive(Clone)]
pub struct ExperientialDomain;

impl Domain for ExperientialDomain {
    fn name(&self) -> &str {
        "ED"
    }

    fn calculate_relevance(&self, autonomy_level: f64) -> f64 {
        0.9 * autonomy_level
    }

    fn transform_state(&self, state: &str, autonomy_level: f64) -> String {
        if autonomy_level > 0.7 {
            format!("Enhanced: {}", state)
        } else {
            state.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_names() {
        assert_eq!(ComputationalDomain.name(), "CD");
        assert_eq!(ScientificDomain.name(), "SD");
        assert_eq!(CulturalDomain.name(), "CuD");
        assert_eq!(ExperientialDomain.name(), "ED");
    }

    #[test]
    fn test_domain_relevance_calculations() {
        let autonomy = 0.8;

        assert_eq!(
            ComputationalDomain.calculate_relevance(autonomy),
            0.8 * autonomy
        );
        assert_eq!(
            ScientificDomain.calculate_relevance(autonomy),
            0.7 * autonomy
        );
        assert_eq!(CulturalDomain.calculate_relevance(autonomy), 0.6 * autonomy);
        assert_eq!(
            ExperientialDomain.calculate_relevance(autonomy),
            0.9 * autonomy
        );
    }

    #[test]
    fn test_domain_transform_state_high_autonomy() {
        let state = "test_state";
        let high_autonomy = 0.8;

        assert_eq!(
            ComputationalDomain.transform_state(state, high_autonomy),
            "Enhanced: test_state"
        );
        assert_eq!(
            ScientificDomain.transform_state(state, high_autonomy),
            "Enhanced: test_state"
        );
        assert_eq!(
            CulturalDomain.transform_state(state, high_autonomy),
            "Enhanced: test_state"
        );
        assert_eq!(
            ExperientialDomain.transform_state(state, high_autonomy),
            "Enhanced: test_state"
        );
    }

    #[test]
    fn test_domain_transform_state_low_autonomy() {
        let state = "test_state";
        let low_autonomy = 0.5;

        assert_eq!(
            ComputationalDomain.transform_state(state, low_autonomy),
            "test_state"
        );
        assert_eq!(
            ScientificDomain.transform_state(state, low_autonomy),
            "test_state"
        );
        assert_eq!(
            CulturalDomain.transform_state(state, low_autonomy),
            "test_state"
        );
        assert_eq!(
            ExperientialDomain.transform_state(state, low_autonomy),
            "test_state"
        );
    }
}
