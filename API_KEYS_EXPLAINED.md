# API Keys Explained - Recursive Light Framework

## TL;DR

**For testing/development**: You don't need any API keys! Just use the mock LLM:

```bash
cd api
cp .env.example .env
# Leave USE_MOCK_LLM=true in the .env file
```

**For real LLM usage**: Add one API key (OpenRouter, Anthropic, or OpenAI)

---

## What Are The API Keys For?

The Recursive Light Framework **wraps around** existing LLM providers. It doesn't replace them - it enhances them.

### Architecture

```
┌─────────────────────────────────────────────────────┐
│  Recursive Light Framework (VIF + Flow Process)     │
│                                                      │
│  • Structures prompts with domain/boundary context  │
│  • Manages state across conversations               │
│  • Tracks patterns and qualities                    │
│  • Implements 7-stage flow process                  │
│  • Enables consciousness-like properties            │
└──────────────────┬──────────────────────────────────┘
                   │ Enhanced Prompt
                   ↓
          ┌────────────────┐
          │  LLM Provider  │  ← API key needed here
          │  (via API)     │
          └────────────────┘
                   ↓
            LLM Response
                   ↓
┌─────────────────────────────────────────────────────┐
│  Framework Post-Processing                          │
│  • Extract state updates                            │
│  • Identify patterns                                │
│  • Track quality emergence                          │
│  • Store in database                                │
└─────────────────────────────────────────────────────┘
```

## Provider Options

### Option 1: Mock LLM (No API Key) - **RECOMMENDED FOR DEVELOPMENT**

```bash
# In api/.env
DEFAULT_PROVIDER=mock
USE_MOCK_LLM=true
```

**Pros:**
- ✓ No API key needed
- ✓ No costs
- ✓ Fast (no network calls)
- ✓ Deterministic (great for testing)
- ✓ Perfect for developing framework features

**Cons:**
- ✗ Doesn't test actual LLM integration
- ✗ Responses are mock/echo

**Use For:**
- Unit tests
- Integration tests
- Framework development
- CI/CD pipelines
- Learning the codebase

---

### Option 2: OpenRouter (Recommended for Real Usage)

**Get Key:** https://openrouter.ai/

```bash
# In api/.env
OPENROUTER_API_KEY=sk-or-v1-xxxxx
DEFAULT_PROVIDER=openrouter
DEFAULT_MODEL=anthropic/claude-3.5-sonnet
USE_MOCK_LLM=false
```

**Pros:**
- ✓ One key → access many models (Claude, GPT, Gemini, etc.)
- ✓ Pay-per-use (cheap for testing)
- ✓ Easy to switch models
- ✓ Good for experimentation

**Cost:** ~$0.003 per request (varies by model)

---

### Option 3: Anthropic Claude

**Get Key:** https://console.anthropic.com/

```bash
# In api/.env
ANTHROPIC_API_KEY=sk-ant-xxxxx
DEFAULT_PROVIDER=anthropic
DEFAULT_MODEL=claude-3-5-sonnet-20241022
USE_MOCK_LLM=false
```

**Pros:**
- ✓ Direct access to Claude models
- ✓ Latest Claude features
- ✓ Official API

**Cost:** ~$0.003-0.015 per request (varies by model)

---

### Option 4: OpenAI

**Get Key:** https://platform.openai.com/

```bash
# In api/.env
OPENAI_API_KEY=sk-xxxxx
DEFAULT_PROVIDER=openai
DEFAULT_MODEL=gpt-4
USE_MOCK_LLM=false
```

**Pros:**
- ✓ Access to GPT models
- ✓ Well-documented
- ✓ Widely used

**Cost:** ~$0.01-0.03 per request (varies by model)

---

## Setup Recommendations

### For Development & Testing
```bash
# Start with mock (no key needed)
DEFAULT_PROVIDER=mock
USE_MOCK_LLM=true
```

### For Feature Testing with Real LLMs
```bash
# Get OpenRouter key (cheapest, most flexible)
OPENROUTER_API_KEY=sk-or-v1-xxxxx
DEFAULT_PROVIDER=openrouter
DEFAULT_MODEL=anthropic/claude-3.5-sonnet
USE_MOCK_LLM=false
```

### For Production
```bash
# Choose based on your needs:
# - OpenRouter: flexibility + cost
# - Anthropic: latest Claude features
# - OpenAI: GPT ecosystem
```

---

## What The Framework Adds

Even with a mock LLM, you can test these framework features:

1. **Domain Management** - Computational, Scientific, Cultural, Experiential
2. **Boundary Dynamics** - Permeability, oscillation, transcendence
3. **Flow Process** - 7-stage processing pipeline
4. **Pattern Tracking** - Lifecycle P⁰→P⁵
5. **Memory Systems** - State persistence across sessions
6. **Quality Emergence** - Clarity, depth, resonance, etc.
7. **BDE Flow** - Invitation → Attention → Resonance → Emergence

The mock LLM lets you develop and test all of this **without any API costs**.

---

## FAQ

**Q: Can I develop the framework without any API key?**
A: Yes! Use `USE_MOCK_LLM=true` (default in `.env.example`)

**Q: Which provider should I use for production?**
A: Depends on your needs:
- **OpenRouter** - flexibility, many models
- **Anthropic** - latest Claude features
- **OpenAI** - GPT ecosystem

**Q: How much will testing cost?**
A: With mock LLM: $0
   With OpenRouter: ~$0.01-0.10 for basic testing

**Q: Can I switch providers later?**
A: Yes! Just change `DEFAULT_PROVIDER` in `.env`

**Q: Do I need all three API keys?**
A: No - just one (or none if using mock)

---

## Next Steps

1. **For Development**: Copy `.env.example` to `.env` (mock is default)
2. **For Real Testing**: Get OpenRouter key, update `.env`
3. **For Production**: Choose provider based on requirements

The framework works the same way regardless of provider - it's just a matter of which LLM generates the responses!
