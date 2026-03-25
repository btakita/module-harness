# module-harness

Language-agnostic module-level harness context management. Parse, validate, and inventory structured doc comments containing **Spec**, **Agentic Contracts**, and **Evals** across source modules.

## What is a Module Harness?

A module harness is a structured doc comment at the top of each source file that provides:

- **Spec** — behavioral specification for each public function/type
- **Agentic Contracts** — invariants and guarantees for both AI agents and API consumers
- **Evals** — named testable scenarios with expected outcomes

This gives AI coding agents (and human developers) immediate context about what a module does, what callers can rely on, and how to verify correctness.

## Installation

```bash
cargo install module-harness
```

## Usage

### Parse harness from a file

```bash
module-harness parse src/config.rs
```

Outputs JSON with module name, spec entries, contracts, and evals.

### Inventory all evals in a project

```bash
module-harness inventory src/ --ext rs
```

Lists every named eval across all modules — useful for tracking test coverage.

### Show harness summary

```bash
module-harness diff src/config.rs
```

## Supported Languages

| Language | Comment Style | Extension |
|----------|--------------|-----------|
| Rust | `//!` doc comments | `.rs` |
| Python | Module docstring (`"""..."""`) | `.py` |
| TypeScript | JSDoc (`/** ... */`) | `.ts`, `.tsx` |
| JavaScript | JSDoc (`/** ... */`) | `.js`, `.jsx` |
| Go | Package comment (`//`) | `.go` |
| Kotlin | KDoc (`/** ... */`) | `.kt`, `.kts` |
| Java | Javadoc (`/** ... */`) | `.java` |

## Harness Format

The format is identical across languages — only the comment syntax changes:

```
# Module: <name>

## Spec
- <behavioral specification for each public function/type>

## Agentic Contracts
- <invariants and guarantees callers can rely on>

## Evals
- <eval_name>: <scenario> → <expected outcome>
```

## License

MIT OR Apache-2.0
