# Harness

## Ontology

Structured [context](./context.md) applied to a [module](./module.md) that bounds what an agent needs to know for effective operation. A harness contains three components: [spec](./spec.md) (what the module does), [contracts](./contract.md) (what callers can rely on), and [evals](./eval.md) (how to verify correctness).

The harness is the narrowest useful [scope](./scope.md) for a coding agent — broader than a function signature, narrower than a project README. It provides just enough context to make informed changes without reading every line of implementation.

## Axiology

Harness context bridges the gap between raw code and agent understanding. Without it, agents must re-derive module behavior from source on every interaction — expensive, error-prone, and inconsistent. The harness serves as persistent, structured memory at the module level.

A well-written harness reduces agent errors, speeds up task completion, and makes agent behavior measurable through [evals](./eval.md).

## Epistemology

### Pattern Expression

A harness manifests as a structured doc comment at the top of each source file. The format is language-agnostic — identical sections (`## Spec`, `## Agentic Contracts`, `## Evals`) appear regardless of whether the comment uses `//!` (Rust), `"""..."""` (Python), or `/** ... */` (TypeScript).

The harness pattern appears at multiple scales:
- **Module level**: doc comment on a single file
- **Crate/package level**: README describing cross-module behavior
- **System level**: architecture documents describing component interactions
