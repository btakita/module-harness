# Drift

## Ontology

Divergence between a [module](./module.md)'s [harness](./harness.md) and its actual state — how the [system](./system.md) has [evolved](./evolution.md) beyond its documented [context](./context.md). Drift occurs when code changes but the harness does not update to match.

Types of drift:
- **Symbol drift**: public functions exist that aren't mentioned in the [spec](./spec.md)
- **Behavioral drift**: spec claims don't match actual behavior
- **Eval drift**: named evals reference tests that no longer exist

## Axiology

Drift is the primary failure mode of documentation systems. A harness that drifts becomes a liability — agents act on stale information, introducing bugs that the harness was supposed to prevent.

Drift detection is therefore a core operation: `module-harness diff` compares harness claims against code reality, surfacing gaps before they cause harm.

## Epistemology

### Pattern Expression

Drift is universal in any system where description and implementation are separate artifacts — API docs vs. API behavior, database schemas vs. application models, architecture diagrams vs. deployed topology. The harness localizes this problem to module scope, making drift detectable and fixable at the smallest useful unit.
