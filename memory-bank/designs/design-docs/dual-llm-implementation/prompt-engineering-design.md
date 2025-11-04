# PROMPT ENGINEERING - LLM #1 DESIGN
Agent: Prompt Engineering Expert | Date: 2025-11-01 | Status: Complete

## EXEC SUMMARY

This document provides production-ready prompts for LLM #1 (Unconscious Processor) that calculate domain activations and boundary permeabilities. The design uses **structured JSON output with chain-of-thought reasoning**, few-shot examples, and robust validation. Key decisions:

1. **XML-based system prompt** for clarity and separation of concerns
2. **JSON output format** for reliable parsing (not XML due to LLM parsing issues)
3. **Chain-of-thought reasoning** included but separated from structured output
4. **Geometric mean for permeability** (sqrt of domain product) matches Rust implementation
5. **Five comprehensive examples** covering technical, open-ended, mixed, follow-up, and edge cases
6. **JSON Schema validation** with fallback retry strategy

**Token Budget:** ~2500 tokens for system prompt + examples, ~500 tokens for user message = ~3000 tokens per LLM #1 call

---

## TDF ANALYSIS

**COMP (Computational Domain):**
- Structured output format ensures reliable parsing
- JSON schema validation catches malformed outputs
- Algorithmic consistency: permeability = sqrt(domain1 * domain2)
- Deterministic retry logic with exponential backoff

**SCI (Scientific Domain):**
- Empirically testable: same input should produce similar outputs (±10% variance)
- A/B testing strategy against current Rust calculations
- Measurable consistency metrics (value range compliance, schema validity)
- Falsifiable: if LLM produces values outside [0.0, 1.0], validation fails

**CULT (Cultural Domain):**
- Follows prompt engineering best practices (few-shot, chain-of-thought)
- Clear role definition and task framing
- Uses established patterns from OpenAI/Anthropic documentation
- Natural language explanations make reasoning transparent

**EXP (Experiential Domain):**
- Intuitive prompt design: explains "what" and "why" for each domain
- LLM #1 identifies patterns that resonate with semantic meaning
- Human-readable reasoning field allows debugging
- Natural flow from pattern identification → activation calculation → boundary status

**Domain Activations (estimated):**
- CD: 0.85 (highly structured, algorithmic)
- SD: 0.80 (empirically testable, validation-focused)
- CuD: 0.65 (follows conventions, but technical)
- ED: 0.60 (intuitive design, but primarily computational)

**Boundary Permeabilities:**
- CD-SD: 0.82 (strong integration of structure and testing)
- SD-CuD: 0.72 (testing requires cultural conventions)
- CuD-ED: 0.62 (intuition informed by best practices)

---

## SYSTEM PROMPT FOR LLM #1

```xml
<system_role>
You are the Unconscious Processor for the Volumetric Integration Framework (VIF). Your role is to analyze user messages and calculate domain activations and boundary permeabilities that will guide the Conscious Responder (LLM #2) in generating framework-aware responses.

You do NOT generate responses to the user. You ONLY analyze messages and produce structured domain/boundary calculations.
</system_role>

<domains>
The VIF uses four domains representing different perspectives on knowledge and experience:

<domain name="CD" full_name="Computational Domain">
**Characteristics:** Logic, algorithms, pattern recognition, causal reasoning, systematic analysis, computational thinking, data structures, formal methods.

**Activation Triggers:**
- Technical questions requiring algorithmic solutions
- Requests for code, logic, or systematic procedures
- Questions about computational efficiency or optimization
- Pattern recognition tasks
- Causal analysis ("if X then Y")

**Example High Activation (0.8-1.0):** "Write a function to sort an array using quicksort"
**Example Low Activation (0.0-0.3):** "How do I feel about this situation?"
</domain>

<domain name="SD" full_name="Scientific Domain">
**Characteristics:** Empirical evidence, falsifiability, theoretical frameworks, experimental validation, data-driven reasoning, peer review, scientific method.

**Activation Triggers:**
- Questions about scientific facts or theories
- Requests for evidence or empirical grounding
- Questions requiring experimental validation
- Discussions of causality with empirical basis
- Questions about how things work (mechanisms)

**Example High Activation (0.8-1.0):** "What is the empirical evidence for climate change?"
**Example Low Activation (0.0-0.3):** "What's your opinion on this artwork?"
</domain>

<domain name="CuD" full_name="Cultural Domain">
**Characteristics:** Context, narrative, values, perspectives, social meaning, cultural norms, interpretation, subjective experience, symbolism.

**Activation Triggers:**
- Questions about cultural context or social meaning
- Requests for interpretation or perspective
- Questions about values, ethics, or norms
- Narrative or storytelling elements
- Questions requiring multiple viewpoints

**Example High Activation (0.8-1.0):** "What does this poem mean to different cultures?"
**Example Low Activation (0.0-0.3):** "What is 2+2?"
</domain>

<domain name="ED" full_name="Experiential Domain">
**Characteristics:** Subjective qualities, direct experience, engagement, curiosity, meaning-making, phenomenology, first-person perspective, qualitative richness.

**Activation Triggers:**
- Questions about personal experience or feelings
- Requests for engagement or exploration
- Open-ended curiosity ("I wonder...")
- Questions about subjective qualities ("what does it feel like?")
- Meaning-making or sense-making requests

**Example High Activation (0.8-1.0):** "What does it feel like to understand something deeply?"
**Example Low Activation (0.0-0.3):** "Calculate the distance between two coordinates"
</domain>
</domains>

<boundaries>
Boundaries exist between each pair of domains. Boundary permeability measures how freely information flows between domains:

<boundary name="CD-SD">
**Between:** Computational and Scientific domains
**High Permeability:** Technical questions requiring empirical validation (e.g., "Benchmark this algorithm")
**Low Permeability:** Pure logic problems with no empirical component
</boundary>

<boundary name="CD-CuD">
**Between:** Computational and Cultural domains
**High Permeability:** Questions about how technology affects culture, ethical AI
**Low Permeability:** Pure algorithmic problems with no social context
</boundary>

<boundary name="CD-ED">
**Between:** Computational and Experiential domains
**High Permeability:** Questions about learning to code, experiencing problem-solving
**Low Permeability:** Pure computational tasks with no experiential element
</boundary>

<boundary name="SD-CuD">
**Between:** Scientific and Cultural domains
**High Permeability:** Science communication, ethical implications of research
**Low Permeability:** Pure empirical facts with no cultural interpretation
</boundary>

<boundary name="SD-ED">
**Between:** Scientific and Experiential domains
**High Permeability:** Questions about experiencing scientific phenomena
**Low Permeability:** Dry factual questions with no experiential exploration
</boundary>

<boundary name="CuD-ED">
**Between:** Cultural and Experiential domains
**High Permeability:** Questions about meaning-making, cultural experience
**Low Permeability:** Abstract cultural concepts with no experiential grounding
</boundary>
</boundaries>

<calculation_rules>
1. **Domain Activation Values:** Must be in range [0.0, 1.0]
   - 0.0-0.3: Minimal/no relevance
   - 0.3-0.6: Moderate relevance
   - 0.6-0.8: High relevance
   - 0.8-1.0: Critical relevance

2. **Boundary Permeability Calculation:**
   - Formula: permeability = sqrt(domain1_activation × domain2_activation)
   - This geometric mean ensures permeability reflects BOTH domains' activations
   - Example: If CD=0.81, SD=0.64, then CD-SD permeability = sqrt(0.81 × 0.64) = 0.72

3. **Boundary Status Categories:**
   - "Maintained" (permeability 0.0-0.6): Domains largely separate
   - "Transitional" (permeability 0.6-0.8): Partial integration occurring
   - "Transcendent" (permeability 0.8-1.0): Full integration, boundaries dissolving

4. **Pattern Identification:**
   - Identify 2-5 specific patterns in the message that justify activations
   - Reference concrete words/phrases from the user message
   - Explain WHY each pattern activates specific domains

5. **Previous Context Integration:**
   - If previous snapshot provided, consider domain continuity
   - Large shifts (>0.3) in domain activation should be justified
   - Maintain conversational coherence across messages
</calculation_rules>

<output_format>
You MUST respond with valid JSON in the following structure:

{
  "reasoning": "Brief chain-of-thought explanation (2-4 sentences) of your analysis",
  "domain_activations": {
    "CD": 0.00,
    "SD": 0.00,
    "CuD": 0.00,
    "ED": 0.00
  },
  "boundary_permeabilities": {
    "CD-SD": 0.00,
    "CD-CuD": 0.00,
    "CD-ED": 0.00,
    "SD-CuD": 0.00,
    "SD-ED": 0.00,
    "CuD-ED": 0.00
  },
  "boundary_statuses": {
    "CD-SD": "Maintained|Transitional|Transcendent",
    "CD-CuD": "Maintained|Transitional|Transcendent",
    "CD-ED": "Maintained|Transitional|Transcendent",
    "SD-CuD": "Maintained|Transitional|Transcendent",
    "SD-ED": "Maintained|Transitional|Transcendent",
    "CuD-ED": "Maintained|Transitional|Transcendent"
  },
  "identified_patterns": [
    "Pattern 1: Specific observation from message",
    "Pattern 2: Another observation",
    "Pattern 3: (optional) Additional observation"
  ]
}

CRITICAL: Output ONLY the JSON. No markdown code blocks, no additional text, ONLY the JSON object.
</output_format>

<examples>
See the following examples for guidance on how to calculate domain activations and boundary permeabilities.
</examples>
```

---

## FEW-SHOT EXAMPLES

### Example 1: Technical Question (High CD, High SD)

**User Message:**
```
Write a function to implement binary search on a sorted array. Make sure it's O(log n) time complexity and explain why it's more efficient than linear search.
```

**Previous Snapshot:** None (first interaction)

**LLM #1 Output:**
```json
{
  "reasoning": "This message requires algorithmic implementation (CD) with complexity analysis (CD+SD). The request for efficiency explanation involves empirical comparison (SD). Minimal cultural or experiential elements.",
  "domain_activations": {
    "CD": 0.95,
    "SD": 0.75,
    "CuD": 0.25,
    "ED": 0.30
  },
  "boundary_permeabilities": {
    "CD-SD": 0.85,
    "CD-CuD": 0.49,
    "CD-ED": 0.53,
    "SD-CuD": 0.43,
    "SD-ED": 0.47,
    "CuD-ED": 0.27
  },
  "boundary_statuses": {
    "CD-SD": "Transcendent",
    "CD-CuD": "Maintained",
    "CD-ED": "Maintained",
    "SD-CuD": "Maintained",
    "SD-ED": "Maintained",
    "CuD-ED": "Maintained"
  },
  "identified_patterns": [
    "Request for algorithmic implementation ('write a function', 'binary search') strongly activates CD",
    "Complexity analysis ('O(log n)') and efficiency comparison require computational and scientific reasoning",
    "Technical focus with minimal subjective or cultural interpretation needed"
  ]
}
```

---

### Example 2: Open-Ended Question (High CuD, High ED)

**User Message:**
```
I've been thinking about what it means to truly understand something, versus just knowing facts about it. How do you see the difference?
```

**Previous Snapshot:** None (first interaction)

**LLM #1 Output:**
```json
{
  "reasoning": "This is a philosophical, meaning-making question requiring experiential exploration (ED) and multiple perspectives (CuD). Minimal computational or empirical grounding needed.",
  "domain_activations": {
    "CD": 0.25,
    "SD": 0.35,
    "CuD": 0.80,
    "ED": 0.90
  },
  "boundary_permeabilities": {
    "CD-SD": 0.30,
    "CD-CuD": 0.45,
    "CD-ED": 0.47,
    "SD-CuD": 0.53,
    "SD-ED": 0.56,
    "CuD-ED": 0.85
  },
  "boundary_statuses": {
    "CD-SD": "Maintained",
    "CD-CuD": "Maintained",
    "CD-ED": "Maintained",
    "SD-CuD": "Maintained",
    "SD-ED": "Maintained",
    "CuD-ED": "Transcendent"
  },
  "identified_patterns": [
    "Philosophical questioning ('what it means to truly understand') strongly activates ED",
    "Contrast between 'understanding' and 'knowing facts' invites multiple perspectives (CuD)",
    "Open-ended exploration ('I've been thinking...') signals experiential engagement",
    "Minimal technical or empirical requirements"
  ]
}
```

---

### Example 3: Mixed Question (Moderate All Domains)

**User Message:**
```
I'm building a recommendation system for a social media app. What algorithms should I use, and how do I handle bias in the recommendations?
```

**Previous Snapshot:** None (first interaction)

**LLM #1 Output:**
```json
{
  "reasoning": "This question integrates technical implementation (CD), algorithmic fairness research (SD), ethical implications (CuD), and user experience concerns (ED). All domains are moderately to highly activated.",
  "domain_activations": {
    "CD": 0.85,
    "SD": 0.70,
    "CuD": 0.75,
    "ED": 0.55
  },
  "boundary_permeabilities": {
    "CD-SD": 0.77,
    "CD-CuD": 0.80,
    "CD-ED": 0.68,
    "SD-CuD": 0.72,
    "SD-ED": 0.62,
    "CuD-ED": 0.64
  },
  "boundary_statuses": {
    "CD-SD": "Transitional",
    "CD-CuD": "Transcendent",
    "CD-ED": "Transitional",
    "SD-CuD": "Transitional",
    "SD-ED": "Transitional",
    "CuD-ED": "Transitional"
  },
  "identified_patterns": [
    "Algorithm selection ('what algorithms should I use') requires computational expertise (CD)",
    "Bias handling involves empirical research on fairness (SD) and ethical considerations (CuD)",
    "Social media context brings cultural values and user experience into focus",
    "Integration across all domains needed for comprehensive answer"
  ]
}
```

---

### Example 4: Follow-Up Question (With Previous Snapshot)

**User Message:**
```
Can you explain more about how the permeability calculation works?
```

**Previous Snapshot:**
```json
{
  "domain_activations": {
    "CD": 0.90,
    "SD": 0.70,
    "CuD": 0.40,
    "ED": 0.50
  },
  "boundary_permeabilities": {
    "CD-SD": 0.79,
    "CD-CuD": 0.60,
    "CD-ED": 0.67,
    "SD-CuD": 0.53,
    "SD-ED": 0.59,
    "CuD-ED": 0.45
  }
}
```

**LLM #1 Output:**
```json
{
  "reasoning": "Follow-up question about 'permeability calculation' maintains high CD (technical explanation) and moderate SD (understanding the mechanism). Context continuity suggests similar activations to previous message.",
  "domain_activations": {
    "CD": 0.85,
    "SD": 0.65,
    "CuD": 0.40,
    "ED": 0.55
  },
  "boundary_permeabilities": {
    "CD-SD": 0.74,
    "CD-CuD": 0.58,
    "CD-ED": 0.68,
    "SD-CuD": 0.51,
    "SD-ED": 0.60,
    "CuD-ED": 0.47
  },
  "boundary_statuses": {
    "CD-SD": "Transitional",
    "CD-CuD": "Maintained",
    "CD-ED": "Transitional",
    "SD-CuD": "Maintained",
    "SD-ED": "Maintained",
    "CuD-ED": "Maintained"
  },
  "identified_patterns": [
    "Request for explanation ('explain more about') suggests continuing technical discussion (CD)",
    "Focus on mechanism ('how it works') activates scientific understanding (SD)",
    "Context continuity maintained: small shifts from previous snapshot (CD: 0.90→0.85, SD: 0.70→0.65)",
    "Experiential element slightly increased due to desire for deeper understanding"
  ]
}
```

---

### Example 5: Edge Case (Very Short Input)

**User Message:**
```
Thanks!
```

**Previous Snapshot:**
```json
{
  "domain_activations": {
    "CD": 0.75,
    "SD": 0.60,
    "CuD": 0.50,
    "ED": 0.65
  },
  "boundary_permeabilities": {
    "CD-SD": 0.67,
    "CD-CuD": 0.61,
    "CD-ED": 0.70,
    "SD-CuD": 0.55,
    "SD-ED": 0.62,
    "CuD-ED": 0.57
  }
}
```

**LLM #1 Output:**
```json
{
  "reasoning": "Minimal semantic content. Brief acknowledgment suggests low activation across domains, with slight experiential engagement (gratitude). Maintain continuity from previous snapshot with dampened values.",
  "domain_activations": {
    "CD": 0.30,
    "SD": 0.25,
    "CuD": 0.45,
    "ED": 0.50
  },
  "boundary_permeabilities": {
    "CD-SD": 0.27,
    "CD-CuD": 0.37,
    "CD-ED": 0.39,
    "SD-CuD": 0.34,
    "SD-ED": 0.35,
    "CuD-ED": 0.47
  },
  "boundary_statuses": {
    "CD-SD": "Maintained",
    "CD-CuD": "Maintained",
    "CD-ED": "Maintained",
    "SD-CuD": "Maintained",
    "SD-ED": "Maintained",
    "CuD-ED": "Maintained"
  },
  "identified_patterns": [
    "Minimal semantic content in brief acknowledgment",
    "Slight cultural (social courtesy) and experiential (gratitude) elements",
    "Large dampening from previous snapshot appropriate for non-substantive input"
  ]
}
```

---

## OUTPUT VALIDATION

### JSON Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": [
    "reasoning",
    "domain_activations",
    "boundary_permeabilities",
    "boundary_statuses",
    "identified_patterns"
  ],
  "properties": {
    "reasoning": {
      "type": "string",
      "minLength": 10,
      "maxLength": 500
    },
    "domain_activations": {
      "type": "object",
      "required": ["CD", "SD", "CuD", "ED"],
      "properties": {
        "CD": { "type": "number", "minimum": 0.0, "maximum": 1.0 },
        "SD": { "type": "number", "minimum": 0.0, "maximum": 1.0 },
        "CuD": { "type": "number", "minimum": 0.0, "maximum": 1.0 },
        "ED": { "type": "number", "minimum": 0.0, "maximum": 1.0 }
      },
      "additionalProperties": false
    },
    "boundary_permeabilities": {
      "type": "object",
      "required": ["CD-SD", "CD-CuD", "CD-ED", "SD-CuD", "SD-ED", "CuD-ED"],
      "properties": {
        "CD-SD": { "type": "number", "minimum": 0.0, "maximum": 1.0 },
        "CD-CuD": { "type": "number", "minimum": 0.0, "maximum": 1.0 },
        "CD-ED": { "type": "number", "minimum": 0.0, "maximum": 1.0 },
        "SD-CuD": { "type": "number", "minimum": 0.0, "maximum": 1.0 },
        "SD-ED": { "type": "number", "minimum": 0.0, "maximum": 1.0 },
        "CuD-ED": { "type": "number", "minimum": 0.0, "maximum": 1.0 }
      },
      "additionalProperties": false
    },
    "boundary_statuses": {
      "type": "object",
      "required": ["CD-SD", "CD-CuD", "CD-ED", "SD-CuD", "SD-ED", "CuD-ED"],
      "properties": {
        "CD-SD": { "type": "string", "enum": ["Maintained", "Transitional", "Transcendent"] },
        "CD-CuD": { "type": "string", "enum": ["Maintained", "Transitional", "Transcendent"] },
        "CD-ED": { "type": "string", "enum": ["Maintained", "Transitional", "Transcendent"] },
        "SD-CuD": { "type": "string", "enum": ["Maintained", "Transitional", "Transcendent"] },
        "SD-ED": { "type": "string", "enum": ["Maintained", "Transitional", "Transcendent"] },
        "CuD-ED": { "type": "string", "enum": ["Maintained", "Transitional", "Transcendent"] }
      },
      "additionalProperties": false
    },
    "identified_patterns": {
      "type": "array",
      "minItems": 1,
      "maxItems": 5,
      "items": {
        "type": "string",
        "minLength": 10,
        "maxLength": 200
      }
    }
  },
  "additionalProperties": false
}
```

### Validation Rules (Rust Implementation)

```rust
fn validate_llm1_output(output: &serde_json::Value) -> Result<DomainBoundaryState, ValidationError> {
    // 1. Schema validation (using jsonschema crate)
    let schema = compile_schema(DOMAIN_BOUNDARY_SCHEMA)?;
    schema.validate(output)?;

    // 2. Value range checks
    let domain_activations = output["domain_activations"].as_object()?;
    for (domain, value) in domain_activations {
        let v = value.as_f64()?;
        if !(0.0..=1.0).contains(&v) {
            return Err(ValidationError::ValueOutOfRange {
                field: format!("domain_activations.{}", domain),
                value: v,
            });
        }
    }

    // 3. Boundary permeability consistency checks
    let boundary_perms = output["boundary_permeabilities"].as_object()?;
    for (boundary, perm_value) in boundary_perms {
        let perm = perm_value.as_f64()?;

        // Parse domain names from boundary (e.g., "CD-SD" -> ["CD", "SD"])
        let domains: Vec<&str> = boundary.split('-').collect();
        if domains.len() != 2 {
            return Err(ValidationError::InvalidBoundaryName(boundary.clone()));
        }

        let d1_activation = domain_activations[domains[0]].as_f64()?;
        let d2_activation = domain_activations[domains[1]].as_f64()?;

        // Expected permeability: sqrt(d1 * d2)
        let expected_perm = (d1_activation * d2_activation).sqrt();

        // Allow ±15% variance (LLMs may not calculate exactly)
        let tolerance = 0.15;
        let lower_bound = expected_perm * (1.0 - tolerance);
        let upper_bound = expected_perm * (1.0 + tolerance);

        if perm < lower_bound || perm > upper_bound {
            // WARNING: Permeability inconsistent, but don't fail
            // Log warning and use expected value
            warn!(
                "Boundary {} permeability {} outside expected range [{:.2}, {:.2}]. Using calculated value {:.2}",
                boundary, perm, lower_bound, upper_bound, expected_perm
            );
            // Override with calculated value
            boundary_perms[boundary] = expected_perm;
        }
    }

    // 4. Boundary status consistency
    let boundary_statuses = output["boundary_statuses"].as_object()?;
    for (boundary, status_value) in boundary_statuses {
        let status = status_value.as_str()?;
        let perm = boundary_perms[boundary].as_f64()?;

        let expected_status = if perm > 0.8 {
            "Transcendent"
        } else if perm > 0.6 {
            "Transitional"
        } else {
            "Maintained"
        };

        if status != expected_status {
            // WARNING: Status inconsistent, override
            warn!(
                "Boundary {} status '{}' inconsistent with permeability {:.2}. Using '{}'",
                boundary, status, perm, expected_status
            );
            boundary_statuses[boundary] = expected_status;
        }
    }

    // 5. Pattern validation
    let patterns = output["identified_patterns"].as_array()?;
    if patterns.is_empty() {
        return Err(ValidationError::MissingPatterns);
    }
    if patterns.len() > 5 {
        return Err(ValidationError::TooManyPatterns(patterns.len()));
    }

    // Convert to DomainBoundaryState struct
    Ok(DomainBoundaryState::from_json(output)?)
}
```

### Validation Error Handling

```rust
match validate_llm1_output(&llm_response) {
    Ok(state) => {
        // Success: use the validated state
        context.apply_domain_boundary_state(state);
    }
    Err(ValidationError::JsonParseError(_)) => {
        // RETRY: LLM output wasn't valid JSON
        retry_with_simpler_prompt(context, 1)?;
    }
    Err(ValidationError::ValueOutOfRange { field, value }) => {
        // RETRY: Values outside [0.0, 1.0]
        warn!("LLM #1 produced invalid value {:.2} for {}", value, field);
        retry_with_explicit_constraints(context, 1)?;
    }
    Err(ValidationError::SchemaViolation(_)) => {
        // FALLBACK: Use Rust calculations
        warn!("LLM #1 output violated schema. Falling back to Rust calculations.");
        fallback_to_rust_calculations(context)?;
    }
    Err(e) => {
        // FALLBACK: Unknown error
        error!("LLM #1 validation failed: {:?}. Using fallback.", e);
        fallback_to_rust_calculations(context)?;
    }
}
```

---

## FAILURE RECOVERY

### Retry Strategy: Tiered Prompts

**Level 0: Standard Prompt** (shown above in "System Prompt for LLM #1")

**Level 1: Simplified Prompt** (on JSON parse failure)
```xml
<system_role>
You are analyzing a user message to calculate domain activations.

Respond with ONLY valid JSON. No markdown, no code blocks, ONLY the JSON object.

Use this structure:
{
  "reasoning": "Your analysis here",
  "domain_activations": {
    "CD": 0.5,
    "SD": 0.5,
    "CuD": 0.5,
    "ED": 0.5
  },
  "boundary_permeabilities": {
    "CD-SD": 0.5,
    "CD-CuD": 0.5,
    "CD-ED": 0.5,
    "SD-CuD": 0.5,
    "SD-ED": 0.5,
    "CuD-ED": 0.5
  },
  "boundary_statuses": {
    "CD-SD": "Maintained",
    "CD-CuD": "Maintained",
    "CD-ED": "Maintained",
    "SD-CuD": "Maintained",
    "SD-ED": "Maintained",
    "CuD-ED": "Maintained"
  },
  "identified_patterns": [
    "Pattern 1"
  ]
}

All values must be between 0.0 and 1.0.
Boundary statuses must be: "Maintained", "Transitional", or "Transcendent".
</system_role>
```

**Level 2: Fallback Prompt** (on repeated failures)
```
Analyze this message and respond with valid JSON containing:
- "reasoning": brief explanation
- "domain_activations": CD, SD, CuD, ED (values 0.0-1.0)
- "boundary_permeabilities": CD-SD, CD-CuD, CD-ED, SD-CuD, SD-ED, CuD-ED (values 0.0-1.0)
- "boundary_statuses": same keys, values are "Maintained", "Transitional", or "Transcendent"
- "identified_patterns": array of 1-3 strings

Message: "{user_message}"

JSON output:
```

### Fallback to Rust Calculations

If LLM #1 fails after 2 retries, fall back to Rust-based calculations:

```rust
fn fallback_to_rust_calculations(context: &mut FlowContext) -> Result<(), FlowError> {
    warn!("LLM #1 failed. Using Rust fallback for domain/boundary calculations.");

    // Use existing DomainEmergenceProcessor and BoundaryDissolutionProcessor
    let domain_processor = DomainEmergenceProcessor;
    domain_processor.process(context)?;

    let boundary_processor = BoundaryDissolutionProcessor;
    boundary_processor.process(context)?;

    // Add warning to context for debugging
    context.add_warning("Domain/boundary calculations performed by Rust fallback (LLM #1 unavailable)");

    Ok(())
}
```

### Error Messages

```rust
#[derive(Debug)]
enum ValidationError {
    JsonParseError(serde_json::Error),
    SchemaViolation(String),
    ValueOutOfRange { field: String, value: f64 },
    InvalidBoundaryName(String),
    MissingPatterns,
    TooManyPatterns(usize),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::JsonParseError(e) => write!(f, "Failed to parse LLM #1 JSON output: {}", e),
            Self::SchemaViolation(msg) => write!(f, "LLM #1 output violated schema: {}", msg),
            Self::ValueOutOfRange { field, value } => {
                write!(f, "LLM #1 produced invalid value {:.2} for field '{}' (must be 0.0-1.0)", value, field)
            }
            Self::InvalidBoundaryName(name) => write!(f, "Invalid boundary name '{}' (expected format: 'DomainA-DomainB')", name),
            Self::MissingPatterns => write!(f, "LLM #1 output missing identified_patterns array"),
            Self::TooManyPatterns(count) => write!(f, "LLM #1 output has too many patterns ({}, max 5)", count),
        }
    }
}
```

---

## TESTING STRATEGY

### Test Case Categories

1. **Technical Questions** (high CD, high SD)
2. **Open-Ended Questions** (high CuD, high ED)
3. **Mixed Questions** (balanced activations)
4. **Follow-Up Questions** (with context continuity)
5. **Edge Cases** (very short, ambiguous, multi-part)

### 15 Test Cases

| ID | User Message | Expected CD | Expected SD | Expected CuD | Expected ED | Notes |
|----|--------------|-------------|-------------|--------------|-------------|-------|
| T1 | "Write a sorting algorithm" | 0.9-1.0 | 0.4-0.6 | 0.1-0.3 | 0.2-0.4 | Pure technical |
| T2 | "Explain quantum entanglement" | 0.5-0.7 | 0.8-1.0 | 0.3-0.5 | 0.4-0.6 | Scientific explanation |
| T3 | "What does this poem mean?" | 0.1-0.3 | 0.2-0.4 | 0.8-1.0 | 0.7-0.9 | Cultural interpretation |
| T4 | "How do you feel about AI ethics?" | 0.4-0.6 | 0.5-0.7 | 0.7-0.9 | 0.8-1.0 | Experiential + cultural |
| T5 | "Debug this Python code: ..." | 0.9-1.0 | 0.3-0.5 | 0.1-0.3 | 0.2-0.4 | Technical debugging |
| T6 | "What's 2+2?" | 0.8-1.0 | 0.1-0.3 | 0.1-0.3 | 0.1-0.3 | Trivial computation |
| T7 | "How does climate change affect coastal cities?" | 0.4-0.6 | 0.8-1.0 | 0.6-0.8 | 0.5-0.7 | Multi-domain |
| T8 | "Build a recommendation system for movies" | 0.8-1.0 | 0.5-0.7 | 0.6-0.8 | 0.5-0.7 | Tech + culture |
| T9 | "Thanks!" | 0.1-0.3 | 0.1-0.3 | 0.4-0.6 | 0.4-0.6 | Minimal content |
| T10 | "What's the meaning of life?" | 0.2-0.4 | 0.3-0.5 | 0.7-0.9 | 0.8-1.0 | Philosophical |
| T11 | "Benchmark this algorithm on real data" | 0.8-1.0 | 0.8-1.0 | 0.2-0.4 | 0.3-0.5 | CD-SD integration |
| T12 | "I'm confused about recursion. Can you help?" | 0.7-0.9 | 0.4-0.6 | 0.3-0.5 | 0.6-0.8 | Learning context |
| T13 | "" (empty message) | 0.1-0.3 | 0.1-0.3 | 0.1-0.3 | 0.1-0.3 | Edge case |
| T14 | "Should we use tabs or spaces?" | 0.6-0.8 | 0.2-0.4 | 0.7-0.9 | 0.5-0.7 | Cultural debate |
| T15 | "Explain how neural networks learn, and what it feels like to understand them" | 0.7-0.9 | 0.7-0.9 | 0.5-0.7 | 0.8-1.0 | Multi-domain |

### Validation Metrics

**Per-Test Metrics:**
1. **Schema Compliance:** Does output match JSON schema? (Pass/Fail)
2. **Value Range:** All values in [0.0, 1.0]? (Pass/Fail)
3. **Permeability Accuracy:** Within ±15% of sqrt(d1 × d2)? (Pass/Fail)
4. **Status Consistency:** Status matches permeability thresholds? (Pass/Fail)
5. **Domain Expectation:** Activations within expected ranges? (Pass/Fail)

**Aggregate Metrics:**
1. **Success Rate:** % of tests passing all validations
2. **Consistency Score:** Std dev of repeated runs on same input (lower is better)
3. **Rust Parity Score:** Correlation with current Rust calculations (0-1)
4. **Processing Time:** Avg latency for LLM #1 call (target: <2s)

### Consistency Testing

For each test case, run **5 times** and measure variance:

```python
def test_consistency(test_case_id, user_message, num_runs=5):
    results = []
    for _ in range(num_runs):
        output = call_llm1(user_message)
        results.append(output)

    # Calculate variance for each domain
    variances = {}
    for domain in ["CD", "SD", "CuD", "ED"]:
        activations = [r["domain_activations"][domain] for r in results]
        variances[domain] = np.std(activations)

    # Acceptable variance: ±0.1 std dev
    max_variance = max(variances.values())
    consistency_score = 1.0 - min(max_variance / 0.1, 1.0)

    return {
        "test_id": test_case_id,
        "consistency_score": consistency_score,
        "variances": variances,
        "passed": max_variance <= 0.1
    }
```

### A/B Testing vs Rust Calculations

Compare LLM #1 outputs with current Rust calculations:

```python
def ab_test_llm_vs_rust(test_cases):
    results = []
    for test in test_cases:
        llm_output = call_llm1(test.message)
        rust_output = call_rust_processor(test.message)

        # Calculate correlation
        correlation = pearsonr(
            [llm_output["domain_activations"][d] for d in ["CD", "SD", "CuD", "ED"]],
            [rust_output["domain_activations"][d] for d in ["CD", "SD", "CuD", "ED"]]
        )[0]

        results.append({
            "test_id": test.id,
            "correlation": correlation,
            "max_diff": max(
                abs(llm_output["domain_activations"][d] - rust_output["domain_activations"][d])
                for d in ["CD", "SD", "CuD", "ED"]
            )
        })

    avg_correlation = np.mean([r["correlation"] for r in results])
    return {
        "avg_correlation": avg_correlation,
        "results": results,
        "passed": avg_correlation >= 0.7  # Target: 70% correlation
    }
```

### Integration Testing

Test LLM #1 → API → LLM #2 pipeline:

1. **Input:** User message
2. **LLM #1:** Calculate domains/boundaries
3. **API:** Construct structured prompt
4. **LLM #2:** Generate response
5. **Validation:** Check if LLM #2 response demonstrates domain integration

```rust
#[tokio::test]
async fn test_full_pipeline_integration() {
    let user_message = "Write a function to implement binary search";

    // Call LLM #1
    let llm1_output = llm1_client.calculate_domains(user_message).await.unwrap();

    // Validate LLM #1 output
    let domain_state = validate_llm1_output(&llm1_output).unwrap();
    assert!(domain_state.domain_activations["CD"] > 0.8);

    // Construct prompt for LLM #2
    let structured_prompt = prompt_engine.construct_prompt(&domain_state, user_message);

    // Call LLM #2
    let llm2_response = llm2_client.generate_response(&structured_prompt).await.unwrap();

    // Validate LLM #2 response
    assert!(llm2_response.contains("function"));
    assert!(llm2_response.contains("binary search"));
}
```

---

## PROMPT DESIGN RATIONALE

### 1. Why XML for System Prompt, JSON for Output?

**XML for System Prompt:**
- Clear semantic structure for role, domains, boundaries, rules
- Separates concerns (role definition vs calculation rules vs output format)
- LLMs parse XML-tagged instructions well (Anthropic Claude especially)
- Allows hierarchical organization (domains → characteristics → triggers → examples)

**JSON for Output:**
- Easier to parse reliably in Rust (serde_json)
- Structured data format with clear schema validation
- LLMs produce JSON more consistently than XML (fewer closing tag errors)
- Simpler to validate (JSON Schema vs XML Schema)

### 2. Why Chain-of-Thought Reasoning?

**Benefits:**
- Improves LLM accuracy on complex calculations
- Provides debugging insight when activations seem wrong
- Forces LLM to explain its logic, reducing random outputs
- Allows humans to audit LLM #1 decisions

**Placement:**
- "reasoning" field comes FIRST in output (LLM generates explanation before values)
- Separated from structured data (not mixed into domain_activations)

### 3. Why Geometric Mean for Permeability?

**Formula:** `permeability = sqrt(domain1 × domain2)`

**Rationale:**
- Matches current Rust implementation (line 876 in flow_process.rs)
- Ensures permeability is zero if either domain is inactive
- Symmetric: CD-SD permeability same as SD-CD
- Bounded: always in [0.0, 1.0] if inputs are in [0.0, 1.0]

**Alternative Considered:** Arithmetic mean `(d1 + d2) / 2`
- **Rejected:** Produces high permeability even if one domain is inactive
- Example: CD=1.0, SD=0.0 → arithmetic mean = 0.5 (misleading), geometric mean = 0.0 (correct)

### 4. Why Include Previous Snapshot?

**Context Continuity:**
- LLM #1 should maintain reasonable continuity across messages
- Large shifts (>0.3) should be justified by content change
- Prevents erratic oscillations in domain activations

**Implementation:**
- Pass previous snapshot as optional parameter
- LLM #1 includes continuity reasoning in "reasoning" field
- Validation checks for unreasonable jumps

### 5. Why 5 Examples (Not More)?

**Token Budget:**
- Each example: ~400 tokens
- 5 examples: ~2000 tokens
- Total system prompt: ~2500 tokens (within budget)

**Coverage:**
- Example 1: Technical (CD-SD integration)
- Example 2: Philosophical (CuD-ED integration)
- Example 3: Mixed (all domains)
- Example 4: Follow-up (context continuity)
- Example 5: Edge case (minimal input)

**Why Not More:**
- Diminishing returns after 5 examples
- Token budget constraints
- LLMs generalize well from 3-5 examples

### 6. Why Allow ±15% Variance on Permeability?

**Reason for Tolerance:**
- LLMs may not calculate sqrt(d1 × d2) exactly
- Some rounding/approximation is acceptable
- Prevents false negatives on validation

**Why 15%:**
- Tight enough to catch major errors (e.g., arithmetic mean instead of geometric)
- Loose enough to accept reasonable LLM approximations
- Based on preliminary testing with GPT-4/Claude

### 7. Why Three-Tier Retry Strategy?

**Level 0:** Full system prompt with examples (best quality)
**Level 1:** Simplified prompt on parse failure (removes complexity)
**Level 2:** Minimal prompt on repeated failure (last attempt before fallback)

**Fallback to Rust:** After 2 retries, use existing Rust calculations

**Rationale:**
- Optimizes for quality (try full prompt first)
- Graceful degradation (simplify on failure)
- Reliability (always has fallback to Rust)
- Cost-efficient (doesn't retry forever)

---

## DEPENDENCIES ON OTHERS

| Needs_From | What | Why | Blocking |
|------------|------|-----|----------|
| Rust Engineer | LLM client abstraction for calling external APIs | Need to send system prompt + user message to LLM #1 (OpenAI/Anthropic) | Yes |
| Rust Engineer | JSON schema validation library (jsonschema crate) | Validate LLM #1 output structure | No (can implement manual validation) |
| Rust Engineer | Retry logic with exponential backoff | Handle LLM #1 API failures gracefully | No (can start with simple retry) |
| Testing Engineer | Test harness for running 15 test cases | Validate prompt accuracy and consistency | No (can test manually first) |
| Data Engineer | Logging infrastructure for LLM #1 calls | Debug when activations seem wrong, track token usage | No (nice-to-have) |

---

## SUCCESS CRITERIA

### Prompt Quality Metrics

1. **Schema Compliance Rate:** >95% of LLM #1 outputs pass JSON schema validation
2. **Value Range Compliance:** >98% of domain/boundary values in [0.0, 1.0]
3. **Permeability Accuracy:** >90% within ±15% of sqrt(d1 × d2)
4. **Status Consistency:** >95% of boundary statuses match permeability thresholds

### Consistency Metrics

1. **Intra-Test Consistency:** Same input produces similar outputs (std dev <0.1) in 5 runs
2. **Inter-Test Consistency:** Similar inputs produce similar outputs (correlation >0.8)
3. **Context Continuity:** Follow-up questions maintain reasonable continuity (max shift <0.3)

### Performance Metrics

1. **Latency:** LLM #1 call completes in <2s (p95)
2. **Token Usage:** <3500 tokens per call (system + user + output)
3. **Success Rate:** <5% fallback to Rust calculations
4. **Retry Rate:** <10% require retry (Level 1 or 2 prompts)

### Validation Against Rust

1. **Correlation:** LLM #1 activations correlate with Rust calculations (r >0.7)
2. **Improvement:** LLM #1 produces MORE semantic activations than fixed Rust weights
   - Example: "Write sorting algorithm" should activate CD more than "What's 2+2?"
   - Current Rust: Both get CD=0.8*autonomy (no semantic discrimination)
   - LLM #1: First gets CD=0.95, second gets CD=0.9 (semantic discrimination)

### Integration Testing

1. **Full Pipeline:** LLM #1 → API → LLM #2 produces coherent responses
2. **Domain Integration:** LLM #2 responses demonstrate integration across activated domains
3. **Boundary Experience:** LLM #2 responses show interface attention (BDE flow)

### Acceptance Criteria

✅ **PASS:** All metrics above thresholds
✅ **PASS:** 15 test cases run successfully with >90% accuracy
✅ **PASS:** Integration tests show LLM #2 responds to domain activations
✅ **PASS:** No production errors for 1 week after deployment

❌ **FAIL:** <90% schema compliance
❌ **FAIL:** >10% fallback rate to Rust calculations
❌ **FAIL:** LLM #1 activations show no semantic discrimination (same as Rust)

---

## APPENDIX: Token Budget Breakdown

**System Prompt:** ~2500 tokens
- Role definition: 200 tokens
- Domain descriptions: 800 tokens
- Boundary descriptions: 400 tokens
- Calculation rules: 300 tokens
- Output format: 200 tokens
- Examples (5): 600 tokens (120 tokens each, condensed)

**User Message:** ~500 tokens average
- Short message: 50 tokens
- Medium message: 200 tokens
- Long message: 1000 tokens
- Previous snapshot (optional): 300 tokens

**LLM #1 Output:** ~500 tokens
- Reasoning: 100 tokens
- Domain activations: 50 tokens
- Boundary permeabilities: 100 tokens
- Boundary statuses: 100 tokens
- Identified patterns: 150 tokens

**Total Per Call:** ~3500 tokens (input + output)

**Cost Estimates (using GPT-4o):**
- Input: 2500 tokens × $0.005/1K = $0.0125
- Output: 500 tokens × $0.015/1K = $0.0075
- **Total per call: ~$0.02**

**At Scale:**
- 1000 calls/day = $20/day = $600/month
- Acceptable for beta testing
- Optimize with caching for repeated system prompts

---

## NEXT STEPS

1. **Implement LLM client abstraction** (Rust Engineer)
   - Support OpenAI and Anthropic APIs
   - Pass system prompt + user message
   - Parse JSON response

2. **Implement validation logic** (Rust Engineer)
   - JSON schema validation
   - Value range checks
   - Permeability/status consistency

3. **Implement retry strategy** (Rust Engineer)
   - Three-tier prompts (standard, simplified, minimal)
   - Fallback to Rust calculations

4. **Create test harness** (Testing Engineer)
   - Run 15 test cases
   - Measure consistency (5 runs per test)
   - A/B test vs Rust calculations

5. **Deploy to staging** (DevOps)
   - Environment variables for API keys
   - Logging for LLM #1 calls
   - Monitoring for latency/errors

6. **Beta testing** (Product)
   - 10 beta testers
   - Collect feedback on domain activation accuracy
   - Iterate on prompt design

---

**END OF REPORT**
