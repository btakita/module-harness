use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

mod parse;

#[derive(Parser)]
#[command(name = "module-harness", about = "Module-level harness context management")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Parse harness context from a source file
    Parse {
        /// Source file to parse
        file: PathBuf,
    },
    /// List all evals across modules in a directory
    Inventory {
        /// Directory to scan (default: current directory)
        #[arg(default_value = ".")]
        dir: PathBuf,
        /// File extensions to scan (comma-separated, e.g. "rs,py,ts")
        #[arg(short, long)]
        ext: Option<String>,
    },
    /// Show harness summary for a file
    Diff {
        /// Source file to check
        file: PathBuf,
    },
    /// Report eval coverage: which evals have matching tests
    Coverage {
        /// Directory to scan (default: current directory)
        #[arg(default_value = ".")]
        dir: PathBuf,
        /// File extensions to scan (comma-separated, e.g. "rs,py,ts")
        #[arg(short, long)]
        ext: Option<String>,
        /// Output as JSON instead of human-readable
        #[arg(long)]
        json: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Parse { file } => cmd_parse(&file),
        Command::Inventory { dir, ext } => cmd_inventory(&dir, ext.as_deref()),
        Command::Diff { file } => cmd_diff(&file),
        Command::Coverage { dir, ext, json } => cmd_coverage(&dir, ext.as_deref(), json),
    }
}

fn cmd_parse(file: &Path) -> Result<()> {
    let content = std::fs::read_to_string(file)
        .with_context(|| format!("failed to read {}", file.display()))?;

    let lang = parse::detect_language(file);
    let harness = parse::extract_harness(&content, lang);

    let json = serde_json::to_string_pretty(&harness)?;
    println!("{}", json);
    Ok(())
}

fn cmd_inventory(dir: &Path, ext_filter: Option<&str>) -> Result<()> {
    let extensions: Vec<&str> = ext_filter
        .map(|e| e.split(',').collect())
        .unwrap_or_else(|| vec!["rs", "py", "ts", "js", "go", "kt", "java"]);

    let mut all_evals = Vec::new();

    walk_files(dir, &extensions, &mut |path| {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return,
        };
        let lang = parse::detect_language(path);
        let harness = parse::extract_harness(&content, lang);
        for eval in &harness.evals {
            all_evals.push(serde_json::json!({
                "file": path.display().to_string(),
                "module": harness.module.clone(),
                "eval": eval.name,
                "description": eval.description,
            }));
        }
    })?;

    let json = serde_json::to_string_pretty(&all_evals)?;
    println!("{}", json);
    Ok(())
}

fn cmd_diff(file: &Path) -> Result<()> {
    let content = std::fs::read_to_string(file)
        .with_context(|| format!("failed to read {}", file.display()))?;

    let lang = parse::detect_language(file);
    let harness = parse::extract_harness(&content, lang);

    if harness.module.is_empty() && harness.spec.is_empty() {
        println!("No harness context found in {}", file.display());
    } else {
        println!("Module: {}", harness.module);
        println!("Spec entries: {}", harness.spec.len());
        println!("Contracts: {}", harness.contracts.len());
        println!("Evals: {}", harness.evals.len());
    }
    Ok(())
}

fn cmd_coverage(dir: &Path, ext_filter: Option<&str>, json_output: bool) -> Result<()> {
    let extensions: Vec<&str> = ext_filter
        .map(|e| e.split(',').collect())
        .unwrap_or_else(|| vec!["rs", "py", "ts", "js", "go", "kt", "java"]);

    let mut modules_total = 0usize;
    let mut modules_with_harness = 0usize;
    let mut evals_total = 0usize;
    let mut evals_covered = 0usize;
    let mut module_reports = Vec::new();

    walk_files(dir, &extensions, &mut |path| {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return,
        };
        let lang = parse::detect_language(path);
        let harness = parse::extract_harness(&content, lang);

        modules_total += 1;
        if !harness.module.is_empty() || !harness.spec.is_empty() {
            modules_with_harness += 1;
        }

        // Find test functions in the file
        let test_names = parse::find_test_names(&content, lang);

        let mut eval_results = Vec::new();
        for eval in &harness.evals {
            evals_total += 1;
            let covered = test_names.iter().any(|t| {
                t == &eval.name || t.contains(&eval.name) || eval.name.contains(t.as_str())
            });
            if covered {
                evals_covered += 1;
            }
            eval_results.push(serde_json::json!({
                "name": eval.name,
                "covered": covered,
                "description": eval.description,
            }));
        }

        if !harness.evals.is_empty() {
            module_reports.push(serde_json::json!({
                "file": path.display().to_string(),
                "module": harness.module,
                "evals": eval_results,
            }));
        }
    })?;

    if json_output {
        let report = serde_json::json!({
            "modules_total": modules_total,
            "modules_with_harness": modules_with_harness,
            "evals_total": evals_total,
            "evals_covered": evals_covered,
            "coverage_ratio": if evals_total > 0 { evals_covered as f64 / evals_total as f64 } else { 0.0 },
            "completeness": if modules_total > 0 { modules_with_harness as f64 / modules_total as f64 } else { 0.0 },
            "modules": module_reports,
        });
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("Module coverage: {}/{} ({:.0}%)",
            modules_with_harness, modules_total,
            if modules_total > 0 { modules_with_harness as f64 / modules_total as f64 * 100.0 } else { 0.0 });
        println!("Eval coverage:   {}/{} ({:.0}%)",
            evals_covered, evals_total,
            if evals_total > 0 { evals_covered as f64 / evals_total as f64 * 100.0 } else { 0.0 });
    }
    Ok(())
}

fn walk_files(
    dir: &Path,
    extensions: &[&str],
    callback: &mut dyn FnMut(&Path),
) -> Result<()> {
    if !dir.is_dir() {
        anyhow::bail!("{} is not a directory", dir.display());
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            if name.starts_with('.') || name == "target" || name == "node_modules" {
                continue;
            }
            walk_files(&path, extensions, callback)?;
        } else if let Some(ext) = path.extension() {
            if extensions.iter().any(|e| ext == *e) {
                callback(&path);
            }
        }
    }
    Ok(())
}
