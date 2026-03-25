# Audit

## Ontology

The systematic process of evaluating a [module](./module.md)'s [harness](./harness.md) against its actual state — applying [evals](./eval.md) to detect [drift](./drift.md) and measure [coverage](./coverage.md). An audit produces two outputs: a **quantifiable result report** (scores, coverage ratios, drift metrics) and **actionable items** that agents can directly perform. The report measures; the actions remediate.

An audit is a [focus](./focus.md)-bounded operation: it selects a [scope](./scope.md) (single module, directory, or project), runs evals within that scope, and returns results at a chosen [resolution](./resolution.md).

**Relationship to other terms:**
- **Eval** defines *what* to measure; **Audit** is the act of *running* those measurements
- **Coverage** is a *metric* the audit computes; **Drift** is a *condition* the audit detects
- **Backtest** compares audit results *across time*; a single audit is a point-in-time snapshot

## Axiology

Audits close the feedback loop between harness authoring and harness quality. Without audits, a harness is a static document — with audits, it becomes a living system that signals when attention is needed.

The value of an audit is proportional to its actionability:
- A good audit tells you *what* is wrong (drift type), *where* (module + symbol), *how far* (eval score + deviation), and *which direction* (over/under provisioned) — and the agent can act on each item immediately
- A poor audit only reports pass/fail without context for remediation

In practice, audit results feed directly into agent work queues. An agentic audit that detects missing spec entries doesn't just report the gap — it generates the spec entries. This closes the loop: audit → detect → remediate → re-audit.

## Epistemology

### Audit Layers

Audits execute across the three eval execution layers:

1. **Static audit**: Parse source → check harness exists, verify symbol coverage, validate structural rules. Fast, deterministic, no side effects.
2. **Dynamic audit**: Run tests → map test results to named evals, compute coverage ratios. Requires build + test infrastructure.
3. **Agentic audit**: Agent reads module + harness → judges contract validity, identifies behavioral drift that static analysis can't detect. Highest fidelity, highest cost.

### Audit Results

Each eval in an audit produces an `eval_result`:

```
eval_result {
  score: 0.0–1.0,
  deviation: f64,
  direction: Under | Within | Over,
}
```

The audit aggregates these into:
- **Module score**: composite of all evals for one module
- **Directory score**: composite across modules (weighted by module size or criticality)
- **Drift report**: list of detected drift items with type, location, and severity

### Pattern Expression

Auditing is universal in systems that maintain invariants: financial audits verify books against transactions, security audits verify configurations against policy, code audits verify implementation against specification. The module-harness audit localizes this to the smallest useful unit — a single source module — and quantifies the result using the eval framework.
