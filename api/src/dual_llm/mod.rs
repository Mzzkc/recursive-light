// Dual-LLM Architecture Module
// LLM #1 (Unconscious): Domain/boundary calculations
// LLM #2 (Conscious): Response generation (existing LLM system)

pub mod config;
pub mod memory_tiering;
pub mod processors;
pub mod prompts;
pub mod types;

// Re-exports for convenience
pub use config::DualLlmConfig;
pub use memory_tiering::{ColdMemory, ConversationTurn, HotMemory, MemoryTierManager, WarmMemory};
pub use processors::UnconscciousLlmProcessor;
pub use prompts::{
    build_llm1_minimal_prompt, build_llm1_simplified_prompt, build_llm1_system_prompt,
    build_llm1_user_prompt,
};
pub use types::{DomainBoundarySnapshot, Llm1Output, ValidationError};
