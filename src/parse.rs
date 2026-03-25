use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Rust,
    Python,
    TypeScript,
    JavaScript,
    Go,
    Kotlin,
    Java,
    Unknown,
}

#[derive(Debug, Default, Serialize)]
pub struct Harness {
    pub module: String,
    pub spec: Vec<String>,
    pub contracts: Vec<String>,
    pub evals: Vec<Eval>,
}

#[derive(Debug, Serialize)]
pub struct Eval {
    pub name: String,
    pub description: String,
}

pub fn detect_language(path: &Path) -> Language {
    match path.extension().and_then(|e| e.to_str()) {
        Some("rs") => Language::Rust,
        Some("py") => Language::Python,
        Some("ts" | "tsx") => Language::TypeScript,
        Some("js" | "jsx") => Language::JavaScript,
        Some("go") => Language::Go,
        Some("kt" | "kts") => Language::Kotlin,
        Some("java") => Language::Java,
        _ => Language::Unknown,
    }
}

pub fn extract_harness(content: &str, lang: Language) -> Harness {
    let doc_comment = extract_doc_comment(content, lang);
    parse_harness_sections(&doc_comment)
}

/// Extract the module-level doc comment from source code.
fn extract_doc_comment(content: &str, lang: Language) -> String {
    match lang {
        Language::Rust => extract_rust_doc_comment(content),
        Language::Python => extract_python_docstring(content),
        Language::TypeScript | Language::JavaScript | Language::Kotlin | Language::Java => {
            extract_jsdoc_comment(content)
        }
        Language::Go => extract_go_comment(content),
        Language::Unknown => String::new(),
    }
}

/// Extract `//!` doc comments from Rust source.
fn extract_rust_doc_comment(content: &str) -> String {
    let mut lines = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("//!") {
            // Strip `//! ` or `//!` prefix
            let text = trimmed.strip_prefix("//! ").unwrap_or(
                trimmed.strip_prefix("//!").unwrap_or(""),
            );
            lines.push(text.to_string());
        } else if trimmed.is_empty() && lines.is_empty() {
            continue; // skip leading blank lines
        } else if !trimmed.starts_with("//!") && !lines.is_empty() {
            break; // end of doc comment block
        }
    }
    lines.join("\n")
}

/// Extract module docstring from Python source (triple-quoted string at top).
fn extract_python_docstring(content: &str) -> String {
    let trimmed = content.trim_start();
    // Look for """ or '''
    for delim in ["\"\"\"", "'''"] {
        if trimmed.starts_with(delim) {
            if let Some(end) = trimmed[delim.len()..].find(delim) {
                return trimmed[delim.len()..delim.len() + end].to_string();
            }
        }
    }
    String::new()
}

/// Extract JSDoc/KDoc `/** ... */` comment at top of file.
fn extract_jsdoc_comment(content: &str) -> String {
    let trimmed = content.trim_start();
    if trimmed.starts_with("/**") {
        if let Some(end) = trimmed.find("*/") {
            let block = &trimmed[3..end];
            let lines: Vec<&str> = block
                .lines()
                .map(|l| {
                    let t = l.trim();
                    t.strip_prefix("* ").unwrap_or(t.strip_prefix("*").unwrap_or(t))
                })
                .collect();
            return lines.join("\n").trim().to_string();
        }
    }
    String::new()
}

/// Extract Go package comment (// lines before `package` keyword).
fn extract_go_comment(content: &str) -> String {
    let mut lines = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") {
            let text = trimmed.strip_prefix("// ").unwrap_or(
                trimmed.strip_prefix("//").unwrap_or(""),
            );
            lines.push(text.to_string());
        } else if trimmed.starts_with("package ") {
            break;
        } else if trimmed.is_empty() && lines.is_empty() {
            continue;
        } else if !trimmed.starts_with("//") && !lines.is_empty() {
            break;
        }
    }
    lines.join("\n")
}

/// Parse the harness sections from extracted doc comment text.
fn parse_harness_sections(text: &str) -> Harness {
    let mut harness = Harness::default();
    let mut current_section = Section::None;

    for line in text.lines() {
        let trimmed = line.trim();

        // Check for section headers
        if trimmed.starts_with("# Module:") || trimmed.starts_with("# Module ") {
            harness.module = trimmed
                .trim_start_matches("# Module:")
                .trim_start_matches("# Module ")
                .trim()
                .to_string();
            continue;
        }
        if trimmed == "## Spec" {
            current_section = Section::Spec;
            continue;
        }
        if trimmed == "## Agentic Contracts" {
            current_section = Section::Contracts;
            continue;
        }
        if trimmed == "## Evals" {
            current_section = Section::Evals;
            continue;
        }
        // Stop at any other ## heading
        if trimmed.starts_with("## ") {
            current_section = Section::None;
            continue;
        }

        // Parse list items in current section
        if let Some(item) = trimmed.strip_prefix("- ") {
            match current_section {
                Section::Spec => harness.spec.push(item.to_string()),
                Section::Contracts => harness.contracts.push(item.to_string()),
                Section::Evals => {
                    if let Some(eval) = parse_eval_entry(item) {
                        harness.evals.push(eval);
                    }
                }
                Section::None => {}
            }
        }
    }

    harness
}

#[derive(Clone, Copy)]
enum Section {
    None,
    Spec,
    Contracts,
    Evals,
}

/// Parse an eval entry like "name: description → expected"
fn parse_eval_entry(text: &str) -> Option<Eval> {
    // Format: "name: description" or "name — description"
    let (name, description) = if let Some(pos) = text.find(": ") {
        (&text[..pos], &text[pos + 2..])
    } else if let Some(pos) = text.find(" — ") {
        (&text[..pos], &text[pos + 4..])
    } else {
        (text, "")
    };

    let name = name
        .trim_start_matches('`')
        .trim_end_matches('`')
        .trim()
        .to_string();

    if name.is_empty() {
        return None;
    }

    Some(Eval {
        name,
        description: description.trim().to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_doc_comment_extraction() {
        let src = r#"//! # Module: config
//!
//! ## Spec
//! - Load config from disk
//! - Save config to disk
//!
//! ## Agentic Contracts
//! - Never panics on missing file
//!
//! ## Evals
//! - load_missing: no file → returns defaults
//! - save_roundtrip: save then load → identical

use anyhow::Result;
"#;
        let harness = extract_harness(src, Language::Rust);
        assert_eq!(harness.module, "config");
        assert_eq!(harness.spec.len(), 2);
        assert_eq!(harness.contracts.len(), 1);
        assert_eq!(harness.evals.len(), 2);
        assert_eq!(harness.evals[0].name, "load_missing");
        assert_eq!(harness.evals[1].name, "save_roundtrip");
    }

    #[test]
    fn python_docstring_extraction() {
        let src = r#""""
# Module: utils

## Spec
- Parse input data
- Validate schema

## Evals
- parse_valid: valid JSON → parsed object
"""

import json
"#;
        let harness = extract_harness(src, Language::Python);
        assert_eq!(harness.module, "utils");
        assert_eq!(harness.spec.len(), 2);
        assert_eq!(harness.evals.len(), 1);
    }

    #[test]
    fn jsdoc_comment_extraction() {
        let src = r#"/**
 * # Module: api
 *
 * ## Spec
 * - Handle GET requests
 *
 * ## Agentic Contracts
 * - Always returns JSON
 *
 * ## Evals
 * - get_success: valid endpoint → 200 response
 */

export function handler() {}
"#;
        let harness = extract_harness(src, Language::TypeScript);
        assert_eq!(harness.module, "api");
        assert_eq!(harness.spec.len(), 1);
        assert_eq!(harness.contracts.len(), 1);
        assert_eq!(harness.evals.len(), 1);
    }

    #[test]
    fn no_harness_returns_empty() {
        let src = "use std::io;\nfn main() {}";
        let harness = extract_harness(src, Language::Rust);
        assert!(harness.module.is_empty());
        assert!(harness.spec.is_empty());
    }

    #[test]
    fn eval_entry_parsing() {
        let eval = parse_eval_entry("load_missing: no file → returns defaults").unwrap();
        assert_eq!(eval.name, "load_missing");
        assert!(eval.description.contains("no file"));

        let eval = parse_eval_entry("`save_roundtrip`: save then load → identical").unwrap();
        assert_eq!(eval.name, "save_roundtrip");
    }

    #[test]
    fn language_detection() {
        assert_eq!(detect_language(Path::new("foo.rs")), Language::Rust);
        assert_eq!(detect_language(Path::new("bar.py")), Language::Python);
        assert_eq!(detect_language(Path::new("baz.ts")), Language::TypeScript);
        assert_eq!(detect_language(Path::new("qux.go")), Language::Go);
        assert_eq!(detect_language(Path::new("x.kt")), Language::Kotlin);
        assert_eq!(detect_language(Path::new("y.java")), Language::Java);
        assert_eq!(detect_language(Path::new("z.txt")), Language::Unknown);
    }
}
