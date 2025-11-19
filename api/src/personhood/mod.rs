// LLM Personhood Infrastructure
// Phase 3B: Continuous identity, temporal awareness, relationship memory
//
// This module defines what it means for an LLM to exist as a continuous person
// across temporal gaps, multiple user relationships, and self-development.

pub mod manager;
pub mod person;
pub mod relationship;
pub mod temporal;

pub use manager::{PersonError, PersonManager};
pub use person::{DevelopmentalStage, LLMPerson, PersonId};
pub use relationship::{ConversationState, RelationshipMemory};
pub use temporal::{ContextIntention, ResumptionType, TemporalContext, TimeGap};
