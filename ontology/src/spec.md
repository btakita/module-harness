# Spec

## Ontology

Behavioral [definition](./definition.md) of a [module](./module.md)'s public interface — what it does, not how. Each entry in a spec describes one public function, type, or trait and the observable behavior a caller can expect.

A spec is descriptive (testable claims about current behavior), not aspirational (future goals). It answers: "If I call this function with these inputs, what happens?"

## Axiology

The spec is the primary navigation tool for agents. Before modifying code, an agent reads the spec to understand what the module promises. After modifying code, the agent updates the spec to reflect new behavior — creating a living document that stays synchronized with implementation.

A spec that drifts from reality is worse than no spec — it creates false confidence. [Drift](./drift.md) detection catches this.

## Epistemology

### Pattern Expression

Spec entries follow a consistent pattern across domains:
- **Functions**: "takes X, returns Y, side effects Z"
- **Types**: "represents X, invariants Y"
- **Error handling**: "on failure, does X (never Y)"

The spec pattern is the same one used in API documentation, design-by-contract, and interface definitions — narrowed to module scope and formatted for agent consumption.
