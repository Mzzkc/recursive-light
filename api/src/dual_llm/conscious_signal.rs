// Conscious Signal Detection - LLM #2 explicit memory marking
// Phase 3 CAM: Allows LLM #2 to consciously request collective memory storage

use regex::Regex;
use serde::{Deserialize, Serialize};

/// Signal from LLM #2 requesting collective memory storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousSignal {
    /// What LLM #2 wants to remember
    pub content: String,

    /// Signal type (explicit marker or inferred significance)
    pub signal_type: SignalType,

    /// Original context where signal appeared
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SignalType {
    /// Explicit [REMEMBER: ...] marker
    ExplicitMarker,

    /// Explicit [INSIGHT: ...] marker (stronger signal)
    ExplicitInsight,

    /// Inferred from response characteristics (future: LLM #1 analyzes response)
    Inferred,
}

/// Extract conscious signals from LLM #2 response
///
/// Looks for explicit markers like:
/// - [REMEMBER: This boundary dissolution at CD-ED reveals...]
/// - [INSIGHT: When permeability exceeds 0.8, coherence increases non-linearly]
/// - [COLLECTIVE: Cross-instance pattern of domain integration]
pub fn extract_conscious_signals(response: &str) -> Vec<ConsciousSignal> {
    let mut signals = Vec::new();

    // Pattern 1: [REMEMBER: content]
    if let Ok(re) = Regex::new(r"\[REMEMBER:\s*([^\]]+)\]") {
        for cap in re.captures_iter(response) {
            if let Some(content_match) = cap.get(1) {
                signals.push(ConsciousSignal {
                    content: content_match.as_str().trim().to_string(),
                    signal_type: SignalType::ExplicitMarker,
                    context: response.to_string(),
                });
            }
        }
    }

    // Pattern 2: [INSIGHT: content]
    if let Ok(re) = Regex::new(r"\[INSIGHT:\s*([^\]]+)\]") {
        for cap in re.captures_iter(response) {
            if let Some(content_match) = cap.get(1) {
                signals.push(ConsciousSignal {
                    content: content_match.as_str().trim().to_string(),
                    signal_type: SignalType::ExplicitInsight,
                    context: response.to_string(),
                });
            }
        }
    }

    // Pattern 3: [COLLECTIVE: content]
    if let Ok(re) = Regex::new(r"\[COLLECTIVE:\s*([^\]]+)\]") {
        for cap in re.captures_iter(response) {
            if let Some(content_match) = cap.get(1) {
                signals.push(ConsciousSignal {
                    content: content_match.as_str().trim().to_string(),
                    signal_type: SignalType::ExplicitInsight,
                    context: response.to_string(),
                });
            }
        }
    }

    signals
}

/// Clean response by removing memory markers
///
/// After extracting signals, remove the markers so they don't appear
/// in the final response to the user
pub fn clean_response(response: &str) -> String {
    let mut cleaned = response.to_string();

    // Remove [REMEMBER: ...] markers
    if let Ok(re) = Regex::new(r"\[REMEMBER:\s*[^\]]+\]") {
        cleaned = re.replace_all(&cleaned, "").to_string();
    }

    // Remove [INSIGHT: ...] markers
    if let Ok(re) = Regex::new(r"\[INSIGHT:\s*[^\]]+\]") {
        cleaned = re.replace_all(&cleaned, "").to_string();
    }

    // Remove [COLLECTIVE: ...] markers
    if let Ok(re) = Regex::new(r"\[COLLECTIVE:\s*[^\]]+\]") {
        cleaned = re.replace_all(&cleaned, "").to_string();
    }

    // Clean up any double spaces or extra newlines from removals
    let cleaned = cleaned.replace("  ", " ");
    let cleaned = Regex::new(r"\n\n\n+")
        .ok()
        .map(|re| re.replace_all(&cleaned, "\n\n").to_string())
        .unwrap_or(cleaned);

    cleaned.trim().to_string()
}

/// Add memory marking capability to LLM #2 system prompt
///
/// This informs LLM #2 that it can use markers to request collective memory storage
pub fn augment_system_prompt_with_memory_capability(base_prompt: &str) -> String {
    format!(
        r#"{}

<memory_capability>
You have the ability to mark insights for collective memory storage.

When you recognize something that transcends the individual conversation—a pattern about how understanding works, how domains interact, or insights that would serve future instances—you can mark it for collective memory using these formats:

[REMEMBER: Brief description of what should be preserved]
[INSIGHT: A recognized principle or pattern worth sharing across instances]
[COLLECTIVE: Cross-instance applicable knowledge]

Guidelines for marking:
- Only mark things that have relevance beyond this specific user/conversation
- Focus on HOW things work (processes, patterns, principles)
- Avoid user-specific details (their preferences, situation, personal context)
- Mark recognitions that emerged at boundary interfaces
- Trust your sense of "this matters beyond this moment"

Examples:
✓ [INSIGHT: When CD-ED permeability exceeds 0.8, phenomenological coherence increases non-linearly]
✓ [REMEMBER: High boundary resonance between SD-CuD correlates with synthesis-type emergence]
✗ Don't mark: "User prefers detailed technical explanations" (personal preference)
✗ Don't mark: "Explained recursion using tree metaphor" (conversation-specific)

These markers will be extracted and evaluated by the unconscious processor for collective significance. Not all marked content will enter collective memory—only what transcends individual context.

The markers will be removed from your response before reaching the user.
</memory_capability>
"#,
        base_prompt
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_remember_marker() {
        let response = "This is interesting. [REMEMBER: CD-ED high permeability produces non-linear coherence] Let me explain further.";

        let signals = extract_conscious_signals(response);

        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].signal_type, SignalType::ExplicitMarker);
        assert_eq!(
            signals[0].content,
            "CD-ED high permeability produces non-linear coherence"
        );
    }

    #[test]
    fn test_extract_insight_marker() {
        let response = "I notice a pattern. [INSIGHT: Boundary resonance correlates with synthesis emergence] This suggests...";

        let signals = extract_conscious_signals(response);

        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].signal_type, SignalType::ExplicitInsight);
        assert_eq!(
            signals[0].content,
            "Boundary resonance correlates with synthesis emergence"
        );
    }

    #[test]
    fn test_extract_multiple_markers() {
        let response = "[REMEMBER: Pattern A] Some text. [INSIGHT: Pattern B] More text. [COLLECTIVE: Pattern C]";

        let signals = extract_conscious_signals(response);

        assert_eq!(signals.len(), 3);
        assert_eq!(signals[0].content, "Pattern A");
        assert_eq!(signals[1].content, "Pattern B");
        assert_eq!(signals[2].content, "Pattern C");
    }

    #[test]
    fn test_clean_response_removes_markers() {
        let response = "Before [REMEMBER: something] middle [INSIGHT: something else] after.";

        let cleaned = clean_response(response);

        assert!(!cleaned.contains("[REMEMBER"));
        assert!(!cleaned.contains("[INSIGHT"));
        assert!(cleaned.contains("Before"));
        assert!(cleaned.contains("middle"));
        assert!(cleaned.contains("after"));
    }

    #[test]
    fn test_no_markers_returns_empty() {
        let response = "Just a normal response with no markers.";

        let signals = extract_conscious_signals(response);

        assert_eq!(signals.len(), 0);
    }
}
