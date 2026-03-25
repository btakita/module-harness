# Contract

## Ontology

An invariant or guarantee a [module](./module.md) upholds for its callers — what they can rely on without reading implementation. Contracts describe [integrity](./integrity.md): the module's ability to adhere to its promises under all conditions.

Contracts differ from [specs](./spec.md) in scope: a spec describes what each function does; a contract describes cross-cutting guarantees that span multiple functions (atomicity, error handling, concurrency safety, idempotency).

## Axiology

Contracts are the most valuable section for agent callers. An agent deciding whether to call a function doesn't need to know every behavior detail — it needs to know: "Will this panic? Is it safe to call concurrently? Does it modify shared state?"

Contracts reduce agent hesitation and defensive coding. When a contract says "never panics on missing file," the agent can skip null-checking boilerplate.

## Epistemology

### Pattern Expression

Contract patterns appear at every system boundary:
- **API contracts**: "POST is idempotent with the same request ID"
- **Database contracts**: "reads are eventually consistent within 5s"
- **Module contracts**: "save_project() creates parent directories if needed"

The agentic qualifier distinguishes these from general API contracts: agentic contracts specifically surface guarantees that AI agents need to reason about — error boundaries, side effect scopes, and safety properties.
