// CAM (Collective Associative Memory) - Public API
// Phase 3: Hypergraph-based knowledge system for Recursive Light Framework

pub mod storage;
pub mod types;

// Re-export core types for convenience
pub use types::{
    CAMError, CAMQuery, CAMQueryResult, DiscoveryMethod, Domain, Hyperedge, Insight,
    LifecycleStage, OscillationContext, PhenomenologicalQualities, QueryFilters, QueryMetadata,
    RelationshipType, TemporalSort, TimeRange,
};

// Re-export storage
pub use storage::CAMStorage;
