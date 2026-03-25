# Backtest

## Ontology

A replayed sequence of agent work against a known codebase state — a [story](./story.md) used to measure [harness](./harness.md) effectiveness. A backtest starts from a git revision, presents the agent with a target task, and measures the outcome.

Backtesting enables A/B comparison of harness contexts: given the same codebase and task, which harness version leads to better agent performance?

## Axiology

Backtesting closes the feedback loop on harness quality. Without it, harness improvement is anecdotal ("I think this helps"). With it, harness improvement is quantitative ("harness v2 reduced agent errors by 40% on the same task set").

This transforms harness authoring from art to engineering — measurable, iteratable, improvable.

## Epistemology

### Pattern Expression

The backtest pattern appears in many domains:
- **Financial backtesting**: replay trading strategies against historical data
- **ML evaluation**: test model versions against held-out datasets
- **Module harness backtesting**: replay agent tasks against historical code states

All share the same structure: fixed input (historical state), variable treatment (strategy/model/harness), measured output (P&L/accuracy/task success).
