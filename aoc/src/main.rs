use anyhow::{Result, bail};
use std::path::PathBuf;

fn find_project_root() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;

    let mut current = current_dir.as_path();

    loop {
        if current.join("aoc.toml").exists() {
            return Ok(current.to_path_buf());
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => {
                bail!(
                    "Could not find 'aoc.toml' root marker.\n\
                     Are you inside your AoC repository?\n\
                     (Run 'touch aoc.toml' at your repo root to mark it)"
                );
            }
        }
    }
}

fn main() -> Result<()> {
    find_project_root()?;
    Ok(())
}
