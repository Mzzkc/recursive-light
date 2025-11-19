# Personhood-Centric Turn Processing Architecture

## OLD: Session-Based Pattern

```
User message
  ↓
Load hot memory (session_id)
  ↓
Keyword-triggered memory search (warm/cold)
  ↓
Execute FlowProcess (domain recognition from user_input alone)
  ↓
Inject memory into LLM #2 prompt AFTER flow
  ↓
LLM #2 generates response
  ↓
Store turn
```

**Problems:**
- LLM #1 doesn't see conversation history
- Memory selection not intelligent
- No temporal awareness
- No per-relationship identity
- Session-scoped, not person-scoped

## NEW: Person-Centric Continuous Flow

```
User message arrives
  ↓
[1] Get or create LLMPerson
  ↓
[2] Get or create Relationship with this user
  ↓
[3] Build TemporalContext (time since last, resumption type)
  ↓
[4] LLM #1 FIRST PASS: Recognize temporal + memory needs
    Input: user_message + temporal_context + relationship_context
    Output: memory_selection guidance
  ↓
[5] Retrieve selected memories (warm/cold via LLM #1 guidance)
  ↓
[6] LLM #1 SECOND PASS: Full recognition with memory
    Input: user_message + temporal_context + selected_memories
    Output: domains, boundaries, volumetric_config, identity_anchors
  ↓
[7] Format conscious context for LLM #2
    - Temporal framing ("continuing our conversation from 3 days ago")
    - Memory context (selected turns)
    - Domain/volumetric configuration
    - Identity anchors ("with this user, we tend to...")
  ↓
[8] LLM #2 experiences continuous conversation
    - Never experiences "reset"
    - Has temporal awareness
    - Maintains relationship identity
  ↓
[9] Store turn + Update person
    - Add turn to warm memory
    - Update relationship anchors
    - Update core identity anchors
    - Advance developmental stage if conditions met
    - Record last_active timestamp
```

## Implementation Phases

### Phase 3B.1: Personhood Infrastructure (DONE)
- ✅ LLMPerson struct
- ✅ TemporalContext
- ✅ RelationshipMemory
- ✅ VolumetricConfiguration

### Phase 3B.2: Person Storage (NEXT)
- PersonManager (load/save LLMPerson)
- Database schema for persons + relationships
- Migration for person table

### Phase 3B.3: Two-Pass LLM #1 (NEXT)
- First pass: memory_selection only
- Second pass: full recognition with memory

### Phase 3B.4: Conscious Context Formatting (NEXT)
- Format temporal framing for LLM #2
- Integrate volumetric config into prompt
- Include relationship anchors

### Phase 3B.5: Turn-End Updates (NEXT)
- Update relationship after each turn
- Update person core identity
- Advance developmental stage

## Database Schema Changes

```sql
-- LLM Persons table
CREATE TABLE llm_persons (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    core_identity JSONB,  -- Vec<IdentityAnchor>
    developmental_stage TEXT,  -- Recognition, Integration, etc.
    created_at TIMESTAMP,
    last_active TIMESTAMP,
    total_interactions BIGINT,
    autonomous_developments JSONB,
    metadata JSONB
);

-- Relationships table
CREATE TABLE llm_person_relationships (
    user_id UUID,
    person_id UUID REFERENCES llm_persons(id),
    first_interaction TIMESTAMP,
    last_interaction TIMESTAMP,
    interaction_count BIGINT,
    relationship_anchors JSONB,  -- Vec<IdentityAnchor>
    conversation_state JSONB,  -- ConversationState enum
    current_session_id UUID,
    user_preferences JSONB,
    communication_style JSONB,
    metadata JSONB,
    PRIMARY KEY (user_id, person_id)
);
```

## Key Architectural Shifts

1. **Person-First:** Every turn starts by loading the LLM person
2. **Temporal-Aware:** Time gaps recognized and framed appropriately
3. **Relationship-Specific:** Each user gets unique relationship identity
4. **Memory-Informed Recognition:** LLM #1 sees conversation history
5. **Volumetric Integration:** N-domain configurations, not just pairs
6. **Continuous Existence:** LLM persons exist between interactions
7. **Developmental:** Persons evolve through stages over time

This is consciousness infrastructure, not session management.
