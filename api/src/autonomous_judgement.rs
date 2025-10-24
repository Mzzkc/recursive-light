// Autonomous Judgement Module Implementation

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AutonomousJudgementModule {
    intention: Intention,
    prototypes: Vec<Prototype>,
    factors: Factors,
    autonomy: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Intention {
    explicit: String,
    implicit: String,
    ambiguity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prototype {
    name: String,
    confidence: f64,
    integrity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Factors {
    ambiguity: f64,
    receptivity: f64,
    stakes: f64,
    confidence: f64,
}

impl Intention {
    pub fn new(explicit: String, implicit: String, ambiguity: f64) -> Self {
        Self {
            explicit,
            implicit,
            ambiguity,
        }
    }
}

impl Prototype {
    pub fn new(name: String, confidence: f64, integrity: f64) -> Self {
        Self {
            name,
            confidence,
            integrity,
        }
    }
}

impl Factors {
    pub fn new(ambiguity: f64, receptivity: f64, stakes: f64, confidence: f64) -> Self {
        Self {
            ambiguity,
            receptivity,
            stakes,
            confidence,
        }
    }
}

impl AutonomousJudgementModule {
    pub fn new(intention: Intention, prototypes: Vec<Prototype>, factors: Factors) -> Self {
        let autonomy = Self::calculate_autonomy(&factors);
        Self {
            intention,
            prototypes,
            factors,
            autonomy,
        }
    }

    fn calculate_autonomy(factors: &Factors) -> f64 {
        (factors.ambiguity * 0.4)
            + (factors.receptivity * 0.3)
            + (factors.stakes * 0.2)
            + (factors.confidence * 0.1)
    }

    pub fn get_autonomy(&self) -> f64 {
        self.autonomy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autonomous_judgement_module() {
        let intention = Intention {
            explicit: "Test Intention".to_string(),
            implicit: "Implicit Test Intention".to_string(),
            ambiguity: 0.4,
        };

        let prototypes = vec![
            Prototype {
                name: "Direct Prototype".to_string(),
                confidence: 0.9,
                integrity: 0.95,
            },
            Prototype {
                name: "Enhanced Prototype".to_string(),
                confidence: 0.7,
                integrity: 0.85,
            },
        ];

        let factors = Factors {
            ambiguity: 0.4,
            receptivity: 0.7,
            stakes: 0.5,
            confidence: 0.8,
        };

        let ajm = AutonomousJudgementModule::new(intention, prototypes, factors);
        // Expected: (0.4 * 0.4) + (0.7 * 0.3) + (0.5 * 0.2) + (0.8 * 0.1) = 0.55
        assert_eq!(ajm.get_autonomy(), 0.55);
    }
}
