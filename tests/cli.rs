use std::path::Path;
use std::process::Command;

fn cargo_bin() -> Command {
    let mut cmd = Command::new(env!("CARGO"));
    cmd.arg("run").arg("--quiet").arg("--");
    cmd.current_dir(env!("CARGO_MANIFEST_DIR"));
    cmd
}

fn fixture_dir() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures"))
}

#[test]
fn parse_rust_file() {
    let output = cargo_bin()
        .arg("parse")
        .arg(fixture_dir().join("sample.rs"))
        .output()
        .expect("failed to run");
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");
    assert_eq!(json["module"], "config");
    assert_eq!(json["spec"].as_array().unwrap().len(), 2);
    assert_eq!(json["contracts"].as_array().unwrap().len(), 1);
    assert_eq!(json["evals"].as_array().unwrap().len(), 2);
}

#[test]
fn parse_python_file() {
    let output = cargo_bin()
        .arg("parse")
        .arg(fixture_dir().join("sample.py"))
        .output()
        .expect("failed to run");
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");
    assert_eq!(json["module"], "utils");
    assert_eq!(json["spec"].as_array().unwrap().len(), 2);
    assert_eq!(json["evals"].as_array().unwrap().len(), 1);
}

#[test]
fn parse_typescript_file() {
    let output = cargo_bin()
        .arg("parse")
        .arg(fixture_dir().join("sample.ts"))
        .output()
        .expect("failed to run");
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");
    assert_eq!(json["module"], "api");
    assert_eq!(json["contracts"].as_array().unwrap().len(), 1);
    assert_eq!(json["evals"].as_array().unwrap().len(), 1);
}

#[test]
fn parse_go_file() {
    let output = cargo_bin()
        .arg("parse")
        .arg(fixture_dir().join("sample.go"))
        .output()
        .expect("failed to run");
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");
    assert_eq!(json["module"], "router");
    assert_eq!(json["spec"].as_array().unwrap().len(), 1);
}

#[test]
fn parse_nonexistent_file_fails() {
    let output = cargo_bin()
        .arg("parse")
        .arg("/nonexistent/file.rs")
        .output()
        .expect("failed to run");
    assert!(!output.status.success());
}

#[test]
fn inventory_fixtures_dir() {
    let output = cargo_bin()
        .arg("inventory")
        .arg(fixture_dir())
        .output()
        .expect("failed to run");
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");
    let evals = json.as_array().unwrap();
    // Should find evals from all fixture files
    assert!(evals.len() >= 4, "expected at least 4 evals, got {}", evals.len());
}

#[test]
fn diff_rust_file() {
    let output = cargo_bin()
        .arg("diff")
        .arg(fixture_dir().join("sample.rs"))
        .output()
        .expect("failed to run");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Module: config"));
    assert!(stdout.contains("Spec entries: 2"));
    assert!(stdout.contains("Evals: 2"));
}

#[test]
fn diff_no_harness() {
    let output = cargo_bin()
        .arg("diff")
        .arg(fixture_dir().join("no_harness.rs"))
        .output()
        .expect("failed to run");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No harness context found"));
}

#[test]
fn coverage_json_output() {
    let output = cargo_bin()
        .arg("coverage")
        .arg(fixture_dir())
        .arg("--json")
        .output()
        .expect("failed to run");
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");
    assert!(json["modules_total"].as_u64().unwrap() > 0);
    assert!(json["evals_total"].as_u64().unwrap() > 0);
    assert!(json.get("coverage_ratio").is_some());
    assert!(json.get("completeness").is_some());
}

#[test]
fn coverage_human_output() {
    let output = cargo_bin()
        .arg("coverage")
        .arg(fixture_dir())
        .output()
        .expect("failed to run");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Module coverage:"));
    assert!(stdout.contains("Eval coverage:"));
}

#[test]
fn parse_plugin_harness_with_typed_evals() {
    let output = cargo_bin()
        .arg("parse")
        .arg(fixture_dir().join("plugin.kt"))
        .output()
        .expect("failed to run");
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");
    assert_eq!(json["module"], "PythonIntentionsPlugin");
    let evals = json["evals"].as_array().unwrap();
    assert_eq!(evals.len(), 7);

    // Check typed evals
    let hook_coverage = &evals[0];
    assert_eq!(hook_coverage["name"], "hook_coverage");
    assert_eq!(hook_coverage["eval_type"]["type"], "boolean");

    let hook_count = &evals[2];
    assert_eq!(hook_count["name"], "hook_count");
    assert_eq!(hook_count["eval_type"]["type"], "range");
    assert_eq!(hook_count["eval_type"]["low"], 3.0);
    assert_eq!(hook_count["eval_type"]["high"], 8.0);

    let complexity = &evals[3];
    assert_eq!(complexity["name"], "interface_complexity");
    assert_eq!(complexity["eval_type"]["type"], "ordinal");
    assert_eq!(complexity["eval_type"]["min"], 1);
    assert_eq!(complexity["eval_type"]["max"], 5);

    let hot_reload = &evals[6];
    assert_eq!(hot_reload["name"], "hot_reload_compat");
    assert_eq!(hot_reload["eval_type"]["type"], "continuous");
}

#[test]
fn score_boolean_pass() {
    let output = cargo_bin()
        .args(["score", "-t", "boolean", "1.0", "--name", "test_eval"])
        .output()
        .expect("failed to run");
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");
    assert_eq!(json["score"], 1.0);
    assert_eq!(json["direction"], "within");
    assert_eq!(json["name"], "test_eval");
}

#[test]
fn score_boolean_fail() {
    let output = cargo_bin()
        .args(["score", "-t", "boolean", "0.0"])
        .output()
        .expect("failed to run");
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");
    assert_eq!(json["score"], 0.0);
    assert_eq!(json["direction"], "under");
}

#[test]
fn score_range_within() {
    let output = cargo_bin()
        .args(["score", "-t", "range: 5..10", "7.0"])
        .output()
        .expect("failed to run");
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");
    assert_eq!(json["score"], 1.0);
    assert_eq!(json["deviation"], 0.0);
    assert_eq!(json["direction"], "within");
}

#[test]
fn score_range_over() {
    let output = cargo_bin()
        .args(["score", "-t", "range: 5..10", "12.0"])
        .output()
        .expect("failed to run");
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");
    assert!(json["score"].as_f64().unwrap() < 1.0);
    assert!(json["deviation"].as_f64().unwrap() > 0.0);
    assert_eq!(json["direction"], "over");
}

#[test]
fn score_range_under() {
    let output = cargo_bin()
        .args(["score", "-t", "range: 5..10", "3.0"])
        .output()
        .expect("failed to run");
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");
    assert!(json["score"].as_f64().unwrap() < 1.0);
    assert!(json["deviation"].as_f64().unwrap() < 0.0);
    assert_eq!(json["direction"], "under");
}

#[test]
fn score_ordinal() {
    let output = cargo_bin()
        .args(["score", "-t", "ordinal: 1..5", "3.0"])
        .output()
        .expect("failed to run");
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");
    assert_eq!(json["score"], 0.5);
}

#[test]
fn score_continuous() {
    let output = cargo_bin()
        .args(["score", "-t", "continuous", "0.85"])
        .output()
        .expect("failed to run");
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");
    assert_eq!(json["score"], 0.85);
}

#[test]
fn score_file_plugin() {
    let output = cargo_bin()
        .arg("score-file")
        .arg(fixture_dir().join("plugin.kt"))
        .arg(r#"{"hook_coverage": 1.0, "hook_count": 12.0, "interface_complexity": 3.0, "hot_reload_compat": 0.85}"#)
        .output()
        .expect("failed to run");
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");
    let results = json.as_array().unwrap();
    assert_eq!(results.len(), 4);

    // hook_coverage: boolean, value=1.0 → score=1.0
    assert_eq!(results[0]["name"], "hook_coverage");
    assert_eq!(results[0]["score"], 1.0);

    // hook_count: range 3..8, value=12.0 → over
    assert_eq!(results[1]["name"], "hook_count");
    assert_eq!(results[1]["direction"], "over");

    // interface_complexity: ordinal 1..5, value=3.0 → score=0.5
    assert_eq!(results[2]["name"], "interface_complexity");
    assert_eq!(results[2]["score"], 0.5);

    // hot_reload_compat: continuous, value=0.85 → score=0.85
    assert_eq!(results[3]["name"], "hot_reload_compat");
    assert_eq!(results[3]["score"], 0.85);
}

#[test]
fn inventory_with_ext_filter() {
    let output = cargo_bin()
        .arg("inventory")
        .arg(fixture_dir())
        .arg("--ext")
        .arg("rs")
        .output()
        .expect("failed to run");
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");
    let evals = json.as_array().unwrap();
    // Only Rust files should be scanned
    for eval in evals {
        assert!(eval["file"].as_str().unwrap().ends_with(".rs"));
    }
}
