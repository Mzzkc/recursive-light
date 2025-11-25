// Dual-LLM Architecture Module
// LLM #1 (Unconscious): Domain/boundary calculations
// LLM #2 (Conscious): Response generation (existing LLM system)

pub mod config;
pub mod conscious_signal;
pub mod insight_extraction;
pub mod insight_processor;
pub mod memory_tiering;
pub mod processors;
pub mod prompts;
pub mod types;
pub mod unified_system_prompt;
pub mod unified_system_v2;

// Re-exports for convenience
pub use config::DualLlmConfig;
pub use conscious_signal::{
    augment_system_prompt_with_memory_capability, clean_response, extract_conscious_signals,
    ConsciousSignal, SignalType,
};
pub use insight_extraction::{
    build_conscious_signal_prompt, build_significance_prompt, SignificanceEvaluation,
};
pub use insight_processor::{InsightExtractionError, InsightExtractionProcessor};
pub use memory_tiering::{ColdMemory, ConversationTurn, HotMemory, MemoryTierManager, WarmMemory};
pub use processors::UnconscciousLlmProcessor;
pub use prompts::{
    build_llm1_minimal_prompt, build_llm1_simplified_prompt, build_llm1_system_prompt,
    build_llm1_user_prompt,
};
pub use types::{
    DomainBoundarySnapshot, IdentityAnchor, Llm1Output, MemorySelection, MemorySelectionGuidance,
    RetrievedMemories, ValidationError, VolumetricConfiguration,
};
pub use unified_system_prompt::build_unified_llm1_system;
pub use unified_system_v2::build_unified_llm1_system_v2;
