//! # Module: config
//!
//! ## Spec
//! - Load configuration from TOML file
//! - Merge defaults with user overrides
//!
//! ## Agentic Contracts
//! - Returns defaults when config file is missing
//!
//! ## Evals
//! - load_missing: no file on disk → returns default config
//! - roundtrip: save then load → values match

use std::path::Path;

pub fn load(_path: &Path) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_missing() {
        let result = load(Path::new("/nonexistent"));
        assert!(result.is_empty());
    }
}
