// Phase 2: Boundary Oscillation Tests
// Tests for oscillatory parameters (F, A, φ) and resonance detection

use api::prompt_engine::BoundaryState;
use std::f64::consts::PI;

#[test]
fn test_boundary_oscillation_basic() {
    // Test that permeability oscillates over time based on frequency, amplitude, and phase
    let mut boundary = BoundaryState::new(
        "test-boundary".to_string(),
        0.5,
        "Maintained".to_string(),
    );

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
    assert!((boundary.permeability - 0.7).abs() < 0.01, "Expected permeability ~0.7, got {}", boundary.permeability);
    assert!((boundary.phase - (PI / 2.0)).abs() < 0.01, "Expected phase ~π/2, got {}", boundary.phase);
}

#[test]
fn test_boundary_oscillation_bounds() {
    // Test that permeability stays within [0.0, 1.0] even with large amplitude
    let mut boundary = BoundaryState::new(
        "test-boundary".to_string(),
        0.5,
        "Maintained".to_string(),
    );

    boundary.frequency = 1.0;
    boundary.amplitude = 0.8; // Large amplitude that could push out of bounds
    boundary.phase = 0.0;

    // Test multiple time steps to ensure clamping works
    for _ in 0..10 {
        boundary.update_oscillation(0.1, 0.5);
        assert!(boundary.permeability >= 0.0 && boundary.permeability <= 1.0,
            "Permeability {} outside bounds [0.0, 1.0]", boundary.permeability);
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

    assert!(boundary1.resonates_with(&boundary2), "Boundaries with similar frequency and phase should resonate");
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

    assert!(!boundary1.resonates_with(&boundary2), "Boundaries with very different frequencies should not resonate");
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

    assert!(!boundary1.resonates_with(&boundary2), "Boundaries with opposite phases should not resonate");
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
    assert!(strength_perfect > 0.9, "Perfect match should have resonance strength > 0.9, got {}", strength_perfect);

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
    assert!(strength_partial > 0.5 && strength_partial < 0.9,
        "Partial match should have resonance strength in [0.5, 0.9], got {}", strength_partial);

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
    assert!(strength_none < 0.3, "No match should have resonance strength < 0.3, got {}", strength_none);
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
        BoundaryState::with_oscillation("b1".to_string(), 0.5, "Maintained".to_string(), 1.0, 0.2, 0.0),
        BoundaryState::with_oscillation("b2".to_string(), 0.6, "Maintained".to_string(), 1.02, 0.2, 0.05),
        BoundaryState::with_oscillation("b3".to_string(), 0.7, "Maintained".to_string(), 1.05, 0.2, 0.1),
        BoundaryState::with_oscillation("b4".to_string(), 0.8, "Maintained".to_string(), 1.03, 0.2, 0.08),
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
    assert!(resonance_count >= 4, "Expected at least 4 resonance pairs among 4 boundaries, got {}", resonance_count);
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

    assert!((ratio - 4.0).abs() < 0.1, "Fast boundary phase change should be ~4x slow boundary, got ratio {}", ratio);
}
