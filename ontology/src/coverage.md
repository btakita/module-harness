# Coverage

## Ontology

The degree to which [evals](./eval.md) account for a [module](./module.md)'s [spec](./spec.md) — a [resolution](./resolution.md) metric on [harness](./harness.md) completeness. Coverage is quantitative: `evals_with_matching_tests / total_evals`.

Coverage operates at two levels:
- **Module coverage**: fraction of modules that have harness context at all
- **Eval coverage**: fraction of named evals that map to actual test functions

## Axiology

Coverage provides the denominator for harness quality. A harness with 100 evals and 30% coverage tells a different story than one with 10 evals and 100% coverage. Combined with [backtest](./backtest.md) results, coverage reveals where to invest effort.

Low coverage isn't inherently bad — aspirational evals document known gaps. But the ratio itself is the signal: it quantifies how much of the spec is verifiable today.

## Epistemology

### Pattern Expression

Coverage is a resolution knob: zoom in to see per-module eval/test alignment, zoom out to see project-wide completeness. The `module-harness coverage` command computes both levels and reports them as ratios.
