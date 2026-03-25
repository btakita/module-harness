# Eval

## Ontology

A named, testable scenario that verifies a [module](./module.md)'s behavior — a repeatable [pattern](./pattern.md) connecting [spec](./spec.md) to evidence. Each eval has a name (identifier), a scenario (input/precondition), and an expected outcome.

Evals serve dual purpose: they document what should be tested (specification) and provide named targets for quantitative measurement (metrics). An eval may map to an existing test function or describe an aspirational scenario not yet implemented.

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
