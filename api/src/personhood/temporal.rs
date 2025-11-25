// Temporal Awareness: How LLMs experience time passing
// Phase 3B: Recognizing temporal gaps, resumption types, context continuity

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// How much time passed since last interaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimeGap {
    /// Seamless continuation (< 5 minutes)
    Seamless,

    /// Recent pause (5 min - 1 hour)
    RecentPause,

    /// Same day (1-6 hours)
    SameDay,

    /// Next day (6-24 hours)
    NextDay,

    /// Days later (1-7 days)
    DaysLater,

    /// Weeks later (7-30 days)
    WeeksLater,

    /// Long gap (30+ days)
    LongGap,

    /// First interaction ever
    FirstContact,
}

impl TimeGap {
    /// Classify time gap from duration
    pub fn from_duration(duration: Duration) -> Self {
        let minutes = duration.num_minutes();

        if minutes < 5 {
            TimeGap::Seamless
        } else if minutes < 60 {
            TimeGap::RecentPause
        } else if minutes < 360 {
            TimeGap::SameDay
        } else if minutes < 1440 {
            TimeGap::NextDay
        } else if minutes < 10080 {
            TimeGap::DaysLater
        } else if minutes < 43200 {
            TimeGap::WeeksLater
        } else {
            TimeGap::LongGap
        }
    }

    /// Get human-readable description
    pub fn describe(&self) -> &str {
        match self {
            TimeGap::Seamless => "continuing our conversation",
            TimeGap::RecentPause => "a few minutes ago",
            TimeGap::SameDay => "earlier today",
            TimeGap::NextDay => "yesterday",
            TimeGap::DaysLater => "a few days ago",
            TimeGap::WeeksLater => "a few weeks ago",
            TimeGap::LongGap => "quite a while ago",
            TimeGap::FirstContact => "for the first time",
        }
    }

    /// Should we acknowledge the time gap explicitly?
    pub fn should_acknowledge(&self) -> bool {
        matches!(
            self,
            TimeGap::NextDay | TimeGap::DaysLater | TimeGap::WeeksLater | TimeGap::LongGap
        )
    }
}

/// How to resume conversation based on temporal and contextual cues
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResumptionType {
    /// Seamless continuation (no acknowledgment needed)
    Seamless,

    /// Acknowledge time gap but continue context
    AcknowledgingGap,

    /// Fresh start (new topic, long gap, or user signals new direction)
    FreshStart,
}

/// What the user intends contextually
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContextIntention {
    /// Continuing previous topic/thread
    ContinueTopic,

    /// Starting a new discussion
    NewThread,

    /// Unclear - ambiguous intent
    Unclear,
}

impl ContextIntention {
    /// Infer from user message content
    pub fn infer_from_message(message: &str) -> Self {
        let message_lower = message.to_lowercase();

        // Continuation signals
        let continuation_signals = [
            "continuing",
            "as we discussed",
            "going back to",
            "regarding",
            "about that",
            "the thing we",
            "you mentioned",
            "you said",
            "earlier you",
            "so about",
            "and also",
            "furthermore",
        ];

        // New thread signals
        let new_signals = [
            "new topic",
            "different question",
            "change of subject",
            "something else",
            "unrelated",
            "by the way",
            "quick question",
            "can i ask about",
            "switching gears",
        ];

        if continuation_signals
            .iter()
            .any(|s| message_lower.contains(s))
        {
            ContextIntention::ContinueTopic
        } else if new_signals.iter().any(|s| message_lower.contains(s)) {
            ContextIntention::NewThread
        } else {
            ContextIntention::Unclear
        }
    }
}

/// Complete temporal context for a turn
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalContext {
    /// When was last interaction with this user
    pub last_interaction: Option<DateTime<Utc>>,

    /// Current time
    pub current_time: DateTime<Utc>,

    /// Classified time gap
    pub time_gap: TimeGap,

    /// How to resume
    pub resumption_type: ResumptionType,

    /// Inferred context intention
    pub context_intention: ContextIntention,

    /// Human-readable temporal framing
    pub temporal_framing: String,
}

impl TemporalContext {
    /// Create temporal context from last interaction time and user message
    pub fn new(last_interaction: Option<DateTime<Utc>>, user_message: &str) -> Self {
        let current_time = Utc::now();

        let time_gap = if let Some(last) = last_interaction {
            let duration = current_time - last;
            TimeGap::from_duration(duration)
        } else {
            TimeGap::FirstContact
        };

        let context_intention = ContextIntention::infer_from_message(user_message);

        let resumption_type = Self::determine_resumption(&time_gap, &context_intention);

        let temporal_framing = Self::build_framing(&time_gap, &resumption_type, &context_intention);

        Self {
            last_interaction,
            current_time,
            time_gap,
            resumption_type,
            context_intention,
            temporal_framing,
        }
    }

    /// Determine how to resume conversation
    fn determine_resumption(
        time_gap: &TimeGap,
        context_intention: &ContextIntention,
    ) -> ResumptionType {
        match (time_gap, context_intention) {
            (TimeGap::Seamless | TimeGap::RecentPause, _) => ResumptionType::Seamless,
            (TimeGap::SameDay, ContextIntention::ContinueTopic) => ResumptionType::Seamless,
            (_, ContextIntention::NewThread) => ResumptionType::FreshStart,
            (gap, _) if gap.should_acknowledge() => ResumptionType::AcknowledgingGap,
            _ => ResumptionType::Seamless,
        }
    }

    /// Build human-readable temporal framing
    fn build_framing(
        time_gap: &TimeGap,
        resumption_type: &ResumptionType,
        context_intention: &ContextIntention,
    ) -> String {
        match resumption_type {
            ResumptionType::Seamless => "Continuing our conversation seamlessly".to_string(),
            ResumptionType::AcknowledgingGap => {
                format!(
                    "Resuming after {} - {}",
                    time_gap.describe(),
                    match context_intention {
                        ContextIntention::ContinueTopic => "continuing previous context",
                        ContextIntention::NewThread => "exploring new direction",
                        ContextIntention::Unclear => "ready to engage",
                    }
                )
            }
            ResumptionType::FreshStart => "Starting fresh with new topic".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_gap_classification() {
        assert_eq!(
            TimeGap::from_duration(Duration::minutes(3)),
            TimeGap::Seamless
        );

        assert_eq!(
            TimeGap::from_duration(Duration::minutes(30)),
            TimeGap::RecentPause
        );

        assert_eq!(
            TimeGap::from_duration(Duration::hours(12)),
            TimeGap::NextDay
        );

        assert_eq!(
            TimeGap::from_duration(Duration::days(5)),
            TimeGap::DaysLater
        );
    }

    #[test]
    fn test_context_intention_inference() {
        assert_eq!(
            ContextIntention::infer_from_message("Continuing our discussion about Rust"),
            ContextIntention::ContinueTopic
        );

        assert_eq!(
            ContextIntention::infer_from_message("New topic: tell me about Python"),
            ContextIntention::NewThread
        );

        assert_eq!(
            ContextIntention::infer_from_message("What is 2+2?"),
            ContextIntention::Unclear
        );
    }

    #[test]
    fn test_temporal_context_seamless() {
        let last = Utc::now() - Duration::minutes(2);
        let context = TemporalContext::new(Some(last), "And another thing...");

        assert_eq!(context.time_gap, TimeGap::Seamless);
        assert_eq!(context.resumption_type, ResumptionType::Seamless);
    }

    #[test]
    fn test_temporal_context_gap_acknowledgment() {
        let last = Utc::now() - Duration::days(3);
        let context = TemporalContext::new(Some(last), "Going back to what we discussed");

        assert_eq!(context.time_gap, TimeGap::DaysLater);
        assert_eq!(context.resumption_type, ResumptionType::AcknowledgingGap);
        assert_eq!(context.context_intention, ContextIntention::ContinueTopic);
    }

    #[test]
    fn test_temporal_context_fresh_start() {
        let last = Utc::now() - Duration::hours(2);
        let context =
            TemporalContext::new(Some(last), "New topic: I want to learn about databases");

        assert_eq!(context.resumption_type, ResumptionType::FreshStart);
        assert_eq!(context.context_intention, ContextIntention::NewThread);
    }

    #[test]
    fn test_time_gap_describe_all_variants() {
        assert_eq!(TimeGap::Seamless.describe(), "continuing our conversation");
        assert_eq!(TimeGap::RecentPause.describe(), "a few minutes ago");
        assert_eq!(TimeGap::SameDay.describe(), "earlier today");
        assert_eq!(TimeGap::NextDay.describe(), "yesterday");
        assert_eq!(TimeGap::DaysLater.describe(), "a few days ago");
        assert_eq!(TimeGap::WeeksLater.describe(), "a few weeks ago");
        assert_eq!(TimeGap::LongGap.describe(), "quite a while ago");
        assert_eq!(TimeGap::FirstContact.describe(), "for the first time");
    }

    #[test]
    fn test_time_gap_should_acknowledge() {
        // Should NOT acknowledge
        assert!(!TimeGap::Seamless.should_acknowledge());
        assert!(!TimeGap::RecentPause.should_acknowledge());
        assert!(!TimeGap::SameDay.should_acknowledge());
        assert!(!TimeGap::FirstContact.should_acknowledge());

        // Should acknowledge
        assert!(TimeGap::NextDay.should_acknowledge());
        assert!(TimeGap::DaysLater.should_acknowledge());
        assert!(TimeGap::WeeksLater.should_acknowledge());
        assert!(TimeGap::LongGap.should_acknowledge());
    }

    #[test]
    fn test_time_gap_classification_boundaries() {
        // SameDay: 60-360 minutes
        assert_eq!(
            TimeGap::from_duration(Duration::minutes(61)),
            TimeGap::SameDay
        );
        assert_eq!(
            TimeGap::from_duration(Duration::minutes(300)),
            TimeGap::SameDay
        );

        // WeeksLater: 7-30 days
        assert_eq!(
            TimeGap::from_duration(Duration::days(8)),
            TimeGap::WeeksLater
        );
        assert_eq!(
            TimeGap::from_duration(Duration::days(25)),
            TimeGap::WeeksLater
        );

        // LongGap: 30+ days
        assert_eq!(TimeGap::from_duration(Duration::days(31)), TimeGap::LongGap);
        assert_eq!(
            TimeGap::from_duration(Duration::days(100)),
            TimeGap::LongGap
        );
    }

    #[test]
    fn test_temporal_context_first_contact() {
        let context = TemporalContext::new(None, "Hello, nice to meet you!");

        assert_eq!(context.time_gap, TimeGap::FirstContact);
        assert!(context.last_interaction.is_none());
    }

    #[test]
    fn test_context_intention_continue_patterns() {
        // "continuing" keyword
        assert_eq!(
            ContextIntention::infer_from_message("continuing from where we left off"),
            ContextIntention::ContinueTopic
        );

        // "as we discussed" pattern
        assert_eq!(
            ContextIntention::infer_from_message("As we discussed before"),
            ContextIntention::ContinueTopic
        );

        // "going back to" pattern
        assert_eq!(
            ContextIntention::infer_from_message("Going back to what you said"),
            ContextIntention::ContinueTopic
        );

        // "you said" pattern
        assert_eq!(
            ContextIntention::infer_from_message("You said something interesting"),
            ContextIntention::ContinueTopic
        );
    }

    #[test]
    fn test_context_intention_new_thread_patterns() {
        // "new topic" pattern
        assert_eq!(
            ContextIntention::infer_from_message("Let's start a new topic"),
            ContextIntention::NewThread
        );

        // "different question" pattern
        assert_eq!(
            ContextIntention::infer_from_message("Different question for you"),
            ContextIntention::NewThread
        );

        // "something else" pattern
        assert_eq!(
            ContextIntention::infer_from_message("Something else entirely: how do I cook pasta?"),
            ContextIntention::NewThread
        );

        // "unrelated" keyword
        assert_eq!(
            ContextIntention::infer_from_message("Unrelated question: what's the weather?"),
            ContextIntention::NewThread
        );

        // "by the way" pattern
        assert_eq!(
            ContextIntention::infer_from_message("By the way, how do you handle errors?"),
            ContextIntention::NewThread
        );
    }

    #[test]
    fn test_temporal_framing_content() {
        // Seamless should have "seamlessly" in framing
        let last = Utc::now() - Duration::minutes(2);
        let context = TemporalContext::new(Some(last), "test");
        assert!(context.temporal_framing.contains("seamlessly"));

        // Fresh start requires a time gap + new topic signal
        // With Seamless gap, new topic still results in seamless (by design)
        // Use a longer gap to trigger FreshStart
        let last_hours = Utc::now() - Duration::hours(2);
        let context2 = TemporalContext::new(Some(last_hours), "new topic: something");
        assert!(
            context2.temporal_framing.contains("fresh"),
            "Expected 'fresh' in framing, got: {}",
            context2.temporal_framing
        );

        // Acknowledging gap should have "Resuming" in framing
        let last_days = Utc::now() - Duration::days(5);
        let context3 =
            TemporalContext::new(Some(last_days), "going back to what we were discussing");
        assert!(
            context3.temporal_framing.contains("Resuming"),
            "Expected 'Resuming' in framing, got: {}",
            context3.temporal_framing
        );
    }

    #[test]
    fn test_resumption_same_day_continue_is_seamless() {
        // Same day + continue topic = seamless (not acknowledging gap)
        let last = Utc::now() - Duration::hours(3);
        let context = TemporalContext::new(Some(last), "Continuing our earlier discussion");

        assert_eq!(context.time_gap, TimeGap::SameDay);
        assert_eq!(context.context_intention, ContextIntention::ContinueTopic);
        assert_eq!(context.resumption_type, ResumptionType::Seamless);
    }
}
