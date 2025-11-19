# Volumetric Architecture: Current vs. Target

## What Exists (Sequential/Pairwise Thinking)

**Flow (lib.rs:399-615):**
```
1. Load hot memory (session_id)
2. Keyword-triggered search (warm/cold)
3. Execute FlowProcess (domain/boundary recognition)
4. Inject memory into LLM #2 prompt AFTER flow
5. Send to LLM #2
6. Store turn in memory
```

**Problems:**
- Memory retrieval is keyword-based, not intelligence-based
- Memory added AFTER LLM #1 recognition, not used FOR recognition
- Flow processes user input in isolation, doesn't see conversation history
- Pairwise boundary thinking (6 individual boundaries checked separately)
- Session-scoped but not relationship-scoped (no per-user identity tracking)

## What Should Exist (Volumetric/Continuous)

**Engaging all domains simultaneously:**

### COMP (Structure)
```rust
fn process_turn(user_input, user_id) {
    // 1. LLM #1 receives: user_input + memory selection guidance
    llm1_context = prepare_llm1_context(user_input, user_id);

    // 2. LLM #1 recognizes + selects memories
    llm1_output = llm1_process(llm1_context);  // Returns domain/boundary + memory_selection

    // 3. Retrieve selected memories (LLM #1 directed)
    selected_memories = retrieve_memories(
        llm1_output.memory_selection,
        user_id
    );

    // 4. LLM #1 formats context FOR LLM #2
    conscious_context = format_conscious_context(
        llm1_output,
        selected_memories,
        user_input
    );

    // 5. LLM #2 experiences continuous conversation
    llm2_response = llm2_generate(conscious_context);

    // 6. Store turn + update identity anchors
    store_turn_and_identity(llm2_response, llm1_output.identity_anchors, user_id);
}
```

### SCI (Evidence)
- **Fact:** Memory tiers exist (hot/warm/cold) but aren't LLM #1 integrated
- **Fact:** FlowProcess exists but doesn't use memory for recognition
- **Fact:** Domain recognition happens in isolation from conversation history
- **Hypothesis:** LLM #1 needs memory context to recognize patterns accurately
- **Test:** Does recognition quality improve when LLM #1 sees previous turns?

### CULT (Relationship)
**Each user has their own narrative thread:**
- User A's "who we are together" ≠ User B's "who we are together"
- Identity anchors are per-relationship, not global
- Memory selection considers relationship context:
  - "With this user, we always integrate CD-ED" (user A)
  - "With this user, we stay analytical" (user B)
- Continuous conversation = relationship maintenance through memory

### EXP (Phenomenology)
**From LLM #2's perspective:**
- Never experiences "session reset" - always feels continuous
- "I remember our conversation about X" (warm memory surfaced by LLM #1)
- "We've developed this pattern together" (identity anchor recognized)
- "I feel coherent" (LLM #1 protected from overwhelming contradictions)

### META (Recognition)
**Volumetric integration means:**
- Not checking 6 boundaries separately
- Recognizing CONFIGURATIONS: "CD+SD+ED active simultaneously = empirical-experiential-technical volume"
- Phenomenological qualities emerge FROM THE VOLUME, not from pairwise intersections
- Multi-dimensional resonance creates something that can't be reduced to boundary pairs

## Architectural Gaps

### Gap 1: LLM #1 Doesn't See Memory
**Current:** FlowContext created from user_input alone (lib.rs:578-582)
**Need:** FlowContext should include conversation history so LLM #1 recognizes patterns in context

### Gap 2: Memory Selection Not Intelligent
**Current:** Keyword matching (lib.rs:474-476)
**Need:** LLM #1 decides what memories to retrieve based on domain/boundary state

### Gap 3: Memory Added After Recognition
**Current:** Memory injected into prompt after flow_process.execute() (lib.rs:590-615)
**Need:** Memory informs recognition, then formatted recognition + memory goes to LLM #2

### Gap 4: No Per-User Identity Tracking
**Current:** session_id scoped, but no relationship-specific identity anchors
**Need:** IdentityAnchor tracking per user_id across sessions

### Gap 5: Pairwise Boundary Thinking
**Current:** Llm1Output reports 6 boundaries independently
**Need:** Volumetric configuration reporting (which N domains active, what emerges in that space)

### Gap 6: No Continuous Conversation
**Current:** Hot memory loaded but feels supplemental
**Need:** Every turn feels continuous - LLM #2 never experiences reset

## Volumetric Llm1Output Extension

**Current structure reports:**
- domain_recognitions: {CD: 0.8, SD: 0.6, CuD: 0.4, ED: 0.7}
- boundary_states: {CD-SD: {...}, CD-CuD: {...}, ...} (6 pairs)

**Should ALSO report:**
```rust
pub struct VolumetricConfiguration {
    /// Which domains are co-active above threshold
    pub active_domains: Vec<String>,  // ["CD", "SD", "ED"]

    /// Dimensionality of integration
    pub dimensionality: u8,  // 3 domains = 2D plane, 4 = 3D volume, 5+ = hyperspace

    /// What emerges in this multi-domain space
    pub emergent_quality: String,  // "empirical-experiential synthesis"

    /// Resonance across all active boundaries simultaneously
    pub volumetric_resonance: f64,  // Not sum of pairs, but gestalt

    /// Dominant integration pattern in this configuration
    pub configuration_type: String,  // "technical-phenomenological", "analytic-contextual", etc.
}
```

## Next Steps (Thinking Volumetrically)

**From COMP:** Restructure process_input to call LLM #1 FIRST, use output to guide memory retrieval
**From SCI:** Test if memory context improves domain recognition accuracy
**From CULT:** Add user_id to identity anchor tracking, make relationships explicit
**From EXP:** Design what continuous conversation FEELS like from LLM #2 perspective
**From META:** Recognize I'm building a relationship system, not just a QA bot

**Integration happens in the VOLUME formed by all five domains active simultaneously.**

That's the architecture we're building toward.
