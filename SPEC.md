# module-harness Specification

## Purpose

Provide deterministic tooling for managing module-level harness context — structured documentation that bridges the gap between source code and AI agent understanding.

## Architecture

```
module-harness (binary — deterministic)
├── parse <file>          → JSON of existing harness sections
├── inventory [dir]       → list all evals across modules
├── diff <file>           → show harness summary / drift
├── coverage [dir]        → match evals to test functions (planned)
└── format <file> --lang  → write harness from JSON input (planned)

module-harness (skill — non-deterministic, LLM-powered)
├── /module-harness <file>        → generate/update harness for one module
├── /module-harness batch <dir>   → all modules in directory
└── /module-harness audit <dir>   → report drift and coverage
```

**Binary** handles structure: parsing, validation, inventory, metrics.
**Skill** handles understanding: reading code, writing specs, inferring contracts.

## Harness Sections

### Spec

Behavioral specification. One entry per public function, type, or trait. Describes *what* the module does, not *how*.

- Must cover every public symbol
- Should be verifiable (testable claims, not aspirations)
- Ordered by importance / call frequency

### Agentic Contracts

Invariants and guarantees that callers can rely on without reading implementation.

- Error handling behavior (panics, returns, logs)
- Atomicity and concurrency guarantees
- Idempotency claims
- Side effect boundaries (what the module touches)

### Evals

Named testable scenarios. Each eval has:
- **Name**: snake_case identifier (could be a test function name)
- **Description**: scenario → expected outcome

Evals serve dual purpose:
1. **Documentation** — what should be tested
2. **Metrics** — quantifiable coverage for comparing harness contexts

## Eval Metrics (Planned)

Evals create a quantifiable signal for comparing different harness contexts via backtesting:

- **Coverage ratio**: evals with matching tests / total evals
- **Drift score**: public symbols not mentioned in spec / total public symbols
- **Completeness**: modules with harness / total modules

These metrics enable A/B comparison: given two harness context versions, which one leads to better agent outcomes when working on the codebase?

## Future: Backtesting

The backtesting workflow (not yet implemented):

1. Select a git revision as baseline
2. Present the agent with a target spec (feature to implement, bug to fix)
3. Run with harness context A, measure outcome
4. Run with harness context B, measure outcome
5. Compare: which harness led to fewer errors, less drift, faster completion?

This creates a feedback loop: harness quality improves agent performance, which is measurable.
