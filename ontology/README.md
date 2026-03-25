# module-harness Ontology

Domain ontology for the module-harness system, extending the [existence-lang](https://github.com/btakita/existence-lang) kernel vocabulary.

## Terms

| Term | Extends | Definition |
|------|---------|------------|
| **Harness** | [Context](../../existence-lang/ontology/src/context.md) | Structured context applied to a module that bounds what an agent needs to know for effective operation. |
| **Spec** | [Definition](../../existence-lang/ontology/src/definition.md) | Behavioral specification of a module's public interface — what it does, not how. |
| **Contract** | [Integrity](../../existence-lang/ontology/src/integrity.md) | An invariant or guarantee a module upholds for its callers — what they can rely on without reading implementation. |
| **Eval** | [Pattern](../../existence-lang/ontology/src/pattern.md) | A named, testable scenario that verifies a module's behavior — a repeatable pattern connecting spec to evidence. |
| **Coverage** | [Resolution](../../existence-lang/ontology/src/resolution.md) | The degree to which evals account for a module's spec — a resolution metric on harness completeness. |
| **Drift** | [Evolution](../../existence-lang/ontology/src/evolution.md) | Divergence between a module's harness and its actual state — how the system has evolved beyond its documented context. |
| **Backtest** | [Story](../../existence-lang/ontology/src/story.md) | A replayed sequence of agent work against a known codebase state — a story used to measure harness effectiveness. |

## Ontology Chain

```
Existence → Entity → System → Domain → Module → Harness
                                                  ├── Spec (what it does)
                                                  ├── Contract (what callers can assume)
                                                  └── Eval (how to verify)
                                                       ├── Coverage (completeness metric)
                                                       └── Backtest (effectiveness metric)
```

The harness narrows the ontology chain from Existence-scope to Module-scope, providing the precise context an agent needs to work on a specific piece of code.
