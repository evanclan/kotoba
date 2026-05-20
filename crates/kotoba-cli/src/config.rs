//! Locate (and create) Kotoba's data directory.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Resolve the Kotoba data directory, creating it if missing.
///
/// Precedence:
/// 1. The explicit `--home` flag / `KOTOBA_HOME` env (passed in here).
/// 2. `$HOME/.kotoba` on Unix-likes; the OS data dir on other platforms via
///    [`directories::BaseDirs`].
pub fn resolve_home(explicit: Option<&Path>) -> Result<PathBuf> {
    let path = if let Some(p) = explicit {
        p.to_path_buf()
    } else {
        let base =
            directories::BaseDirs::new().context("could not determine user home directory")?;
        base.home_dir().join(".kotoba")
    };
    if !path.exists() {
        std::fs::create_dir_all(&path)
            .with_context(|| format!("creating data directory: {}", path.display()))?;
    }
    Ok(path)
}
