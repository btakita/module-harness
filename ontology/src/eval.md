# Eval

## Ontology

A named, testable scenario that verifies a [module](./module.md)'s behavior — a repeatable [pattern](./pattern.md) connecting [spec](./spec.md) to evidence. Each eval has a name (identifier), a scenario (input/precondition), and an expected outcome.

Evals serve dual purpose: they document what should be tested (specification) and provide named targets for quantitative measurement (metrics). An eval may map to an existing test function or describe an aspirational scenario not yet implemented.

**Ontology chain mapping:**

| Term | Role in Eval |
|------|-------------|
| **System** | The module under evaluation — viewed from an internalized perspective |
| **Resolution** | The granularity of measurement — boolean (1.0/0.0) is lowest; ordinal and continuous are higher |
| **Pattern** | What evals detect — repeatable, predictable elements that transfer across contexts |
| **Scope** | Bounds what's relevant to each eval — limits the information needed for coherent judgment |
| **Context** | The full attention space (agentic + human) active during evaluation — broader than scope, includes all information held in working memory at the moment of judgment |
| **Focus** | Finite attention applied — each eval targets one measurable aspect to avoid dilution |

## Axiology

Evals make [harness](./harness.md) quality measurable. Without named evals, "how good is this harness?" is subjective. With evals, [coverage](./coverage.md) becomes a ratio, and [backtesting](./backtest.md) can compare harness versions by measuring which leads to better agent outcomes.

The name is critical — it creates an addressable handle that test frameworks, coverage tools, and backtest harnesses can reference programmatically.

## Epistemology

### Pattern Expression

The eval pattern is universal in verification systems:
- **Unit tests**: `#[test] fn load_missing_returns_defaults()`
- **BDD**: `Given no config file, When load() is called, Then defaults are returned`
- **Module harness**: `load_missing: no file → returns defaults`

All three express the same pattern at different resolutions. The module harness format is the most compact — optimized for agent scanning speed.

### Quantification Model

Evals use agents to quantifiably measure phenomena. Every eval produces a score on `[0.0, 1.0]`:

- **Boolean** (`true → 1.0, false → 0.0`): Pass/fail evals — contract violations, structural checks. Lowest resolution.
- **Ordinal** (Likert `1-5` mapped to `0.0–1.0`): Quality/style evals — voice consistency, complexity assessment. Mid resolution.
- **Continuous** (`0.0–1.0`): Coverage ratios, similarity scores. Highest resolution.
- **Composite**: Weighted average of sub-evals → aggregate module health score.

### Range Evals with Directional Deviation

Some properties have an optimal band rather than an extremum. "Is the interface too complex?" and "Is the interface too simple?" are both failure modes — the ideal sits between them.

Range evals return both a **score** and a **signed deviation vector**:

```
eval_result {
  score: 0.0–1.0,                        // how far from optimal
  deviation: f64,                         // signed distance (- = under, + = over)
  direction: Under | Within | Over,       // categorical label
  raw_value: Option<f64>,                 // the measured value
}
```

**Scoring rules:**
- `score = 1.0, deviation = 0.0, direction = Within` when value is inside the band
- `score < 1.0, deviation < 0.0, direction = Under` when value is below the band
- `score < 1.0, deviation > 0.0, direction = Over` when value is above the band

**Why direction matters:**
- **Actionable diagnostics**: "remove 2 hooks" vs "add 2 hooks" — the agent knows *what to do*
- **Trend analysis**: consecutive evals trending toward `+over` signals architectural drift
- **Asymmetric penalties**: over-provisioning might be cheaper to fix than under-provisioning

**Eval type annotations** (bracket syntax in harness comments):
- `[boolean]` — pass/fail: `true → 1.0, false → 0.0`
- `[ordinal: MIN..MAX]` — Likert scale mapped to `[0.0, 1.0]`
- `[continuous]` — raw ratio in `[0.0, 1.0]`
- `[range: LOW..HIGH]` — optimal band with directional deviation

Examples:
- `hook_count [range: 3..8]`: 5 hooks → `{score: 1.0, direction: Within}`, 12 hooks → `{score: 0.27, deviation: +4.0, direction: Over}`
- `interface_complexity [ordinal: 1..5]`: rating 3 → `{score: 0.5}`
- `hook_coverage [boolean]`: all registered → `{score: 1.0}`, missing one → `{score: 0.0}`

### Eval Execution Layers

1. **Static evals** (code-graded, instant): Parse source and config → count hooks, measure API surface, check structural rules
2. **Dynamic evals** (test-backed): Run against a live/test environment → verify behavior, check regressions
3. **Agentic evals** (LLM-graded): Agent reads the module + harness → judges whether contracts match implementation, flags [drift](./drift.md)
