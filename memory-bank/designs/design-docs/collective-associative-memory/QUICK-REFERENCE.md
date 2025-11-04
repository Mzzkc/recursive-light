# CAM System - Quick Reference Card

## Data Structures

### Insight (Node)
```rust
struct Insight {
    id: Uuid,
    content: String,                          // "When X happens, do Y because Z"
    embedding: Option<Vec<f32>>,              // 1536-dim semantic vector
    primary_domain: Domain,                   // CD, SD, CuD, ED
    secondary_domains: Vec<Domain>,           // Cross-domain insights
    confidence: f64,                          // 0.0-1.0
    lifecycle: LifecycleStage,                // Emerging → Validated → Established
    source_instance_id: Uuid,                 // Which AI discovered this
    oscillation_context: OscillationContext,  // BDE metadata
    observation_count: u32,                   // How many times validated
    created_at: DateTime<Utc>,
    last_validated: DateTime<Utc>,
}
```

### Hyperedge (Relationship)
```rust
struct Hyperedge {
    id: Uuid,
    insight_ids: Vec<Uuid>,                   // 2+ connected insights
    relationship: RelationshipType,           // Contradiction, Reinforcement, etc.
    strength: f64,                            // 0.0-1.0
    spanning_domains: Vec<Domain>,
    discovery_method: DiscoveryMethod,
    discovered_by: Uuid,
}
```

## Query Types

### 1. Semantic Search
```rust
CAMQuery::Semantic {
    query_text: "async Rust best practices",
    domains: Some(vec![Domain::Computational]),
    min_confidence: 0.6,
    limit: 5,
}
```

### 2. Domain Intersection
```rust
CAMQuery::DomainIntersection {
    domains: vec![Domain::Computational, Domain::Experiential],
    min_confidence: 0.7,
    limit: 10,
}
```

### 3. Structural Traversal
```rust
CAMQuery::Structural {
    start_insight_id: some_uuid,
    relationship_types: vec![RelationshipType::Reinforcement],
    max_depth: 3,
    limit: 20,
}
```

### 4. Oscillation Pattern
```rust
CAMQuery::OscillationPattern {
    boundary: "CD-ED",
    frequency_range: (1.5, 2.5),  // High frequency
    amplitude_range: (0.5, 1.0),  // High amplitude
    limit: 5,
}
```

### 5. Temporal (Trending)
```rust
CAMQuery::Temporal {
    time_range: TimeRange::LastWeek,
    domains: None,
    sort_by: TemporalSort::HighestConfidenceGrowth,
    limit: 10,
}
```

## Lifecycle Stages

| Stage | Confidence | Criteria | Validation Frequency |
|-------|-----------|----------|---------------------|
| **Emerging** | 0.3-0.5 | Just discovered | Daily |
| **Validated** | 0.5-0.75 | 2+ instances observed | Weekly |
| **Established** | >0.75 | 3+ instances, widely confirmed | Monthly |
| **Deprecated** | Any | Contradicted or outdated | Quarterly review |

## Confidence Decay Model

```rust
decayed_confidence = confidence * exp(-age_days / half_life)

where half_life:
  - Emerging: 30 days
  - Validated: 90 days
  - Established: 180 days
  - Deprecated: 7 days
```

## Integration Hooks

### Stage 6: Extract Insights
```rust
// In PatternExtractionProcessor::process()
if let Some(extractor) = &context.insight_extractor {
    tokio::spawn(async move {
        let insights = extractor.extract_from_flow(&context).await?;
        // Stored asynchronously, non-blocking
    });
}
```

### Stage 7: Query Insights
```rust
// In AdaptiveEvolutionProcessor::process()
let insights = query_engine.execute(CAMQuery::Semantic {
    query_text: context.user_message.clone(),
    min_confidence: 0.6,
    limit: 5,
}).await?;

// Enrich LLM prompt with collective insights
let prompt = format!(
    "USER: {}\n\nCOLLECTIVE INSIGHTS:\n{}",
    user_message,
    format_insights(&insights)
);
```

## Database Queries

### Insert Insight
```sql
INSERT INTO cam_insights (
    id, content, embedding, primary_domain,
    confidence, lifecycle_stage, source_instance_id,
    oscillation_context
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8);
```

### Semantic Search
```sql
SELECT id, content, confidence,
       1 - (embedding <=> $1) AS similarity
FROM cam_insights
WHERE lifecycle_stage != 'Deprecated'
  AND (1 - (embedding <=> $1)) >= $2
ORDER BY similarity DESC
LIMIT $3;
```

### Increment Observation
```sql
SELECT cam_increment_observation($insight_id);
```

### Get Insights Needing Validation
```sql
SELECT * FROM cam_insights_needing_validation
LIMIT 100;
```

## Performance Tips

1. **Use Connection Pooling:** Max 20 connections for reads
2. **Cache Frequent Queries:** LRU cache with 5-min TTL
3. **Batch Embeddings:** Generate embeddings in parallel
4. **Tune pgvector Index:** `lists = sqrt(total_insights)`
5. **Monitor Query Time:** Alert if p95 >150ms

## Error Handling

```rust
match query_engine.execute(query).await {
    Ok(result) => {
        // Use insights to enrich response
    }
    Err(CAMError::QueryError(e)) => {
        // Log error, continue without CAM insights
        tracing::warn!("CAM query failed: {}", e);
    }
    Err(e) => {
        // Critical error, return gracefully
        return Err(FlowError::from(e));
    }
}
```

## Monitoring Metrics

- `cam_query_duration_ms` (histogram)
- `cam_extraction_duration_ms` (histogram)
- `cam_insights_total` (gauge)
- `cam_validations_per_hour` (counter)
- `cam_confidence_distribution` (histogram)
- `cam_query_cache_hit_rate` (gauge)

## Common Patterns

### Extract and Store
```rust
let insight = Insight::new(
    "Users prefer concrete examples before abstract theory".to_string(),
    Domain::Experiential,
    vec![Domain::Computational],
    instance_id,
    oscillation_context,
);

let embedding = embedding_generator.generate(&insight.content).await?;
insight.embedding = Some(embedding);

storage.insert_insight(&insight).await?;
```

### Query and Filter
```rust
let result = query_engine.execute(CAMQuery::Hybrid {
    semantic: Some(Box::new(CAMQuery::Semantic {
        query_text: user_input.clone(),
        domains: None,
        min_confidence: 0.5,
        limit: 10,
    })),
    structural: None,
    filters: QueryFilters {
        min_confidence: Some(0.6),
        lifecycle_stages: Some(vec![
            LifecycleStage::Validated,
            LifecycleStage::Established,
        ]),
        exclude_deprecated: true,
        domains: None,
    },
}).await?;
```

### Validate and Update
```rust
let validation = validation_pipeline.validate_insight(&insight).await?;

match validation.outcome {
    ValidationOutcome::Confirmed => {
        storage.update_confidence(
            insight.id,
            (insight.confidence * 1.1).min(1.0)
        ).await?;
    }
    ValidationOutcome::Contradicted => {
        storage.update_lifecycle(
            insight.id,
            LifecycleStage::Deprecated
        ).await?;
    }
    _ => {}
}
```

## Testing

### Unit Test: Insight Creation
```rust
#[test]
fn test_insight_confidence_decay() {
    let mut insight = Insight::new(...);
    insight.last_validated = Utc::now() - Duration::days(45);
    insight.confidence = 0.8;
    insight.lifecycle = LifecycleStage::Validated;

    let decayed = insight.calculate_decayed_confidence();
    assert!(decayed < 0.8); // Should decay over time
    assert!(insight.needs_revalidation()); // Should trigger validation
}
```

### Integration Test: Query Flow
```rust
#[tokio::test]
async fn test_semantic_query_flow() {
    let query_engine = setup_test_query_engine().await;

    let result = query_engine.execute(CAMQuery::Semantic {
        query_text: "test query",
        domains: None,
        min_confidence: 0.5,
        limit: 5,
    }).await.unwrap();

    assert!(result.query_metadata.query_time_ms < 100);
    assert!(result.insights.len() <= 5);
}
```

## Troubleshooting

| Issue | Cause | Solution |
|-------|-------|----------|
| Queries >200ms | Index not created | `CREATE INDEX ... USING ivfflat` |
| Low confidence insights | Not enough observations | Lower threshold or wait for validation |
| No results | Embedding mismatch | Check embedding dimension (1536 vs 768) |
| High memory usage | Too many cached embeddings | Reduce LRU cache size |
| Validation failing | LLM API timeout | Increase timeout, add retry logic |

## Quick Checklist

- [ ] PostgreSQL with pgvector extension installed
- [ ] CAM tables created via migration
- [ ] Insight extractor integrated in Stage 6
- [ ] Query engine integrated in Stage 7
- [ ] Validation pipeline scheduled (hourly)
- [ ] Monitoring/alerting configured
- [ ] Error handling tested
- [ ] Performance benchmarked (<100ms queries)

## Further Reading

- **Full Design:** CAM-DESIGN.md
- **Architecture:** Section 2
- **Implementation Plan:** Section 10
- **Example Queries:** Section 11
