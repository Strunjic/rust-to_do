use crate::models::ToDo;
use anyhow::{Context, Result};
use dirs::config_dir;
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};

pub fn default_store_path() -> Result<PathBuf> {
    if let Some(mut cfg) = config_dir() {
        cfg.push("rust_todo");
        fs::create_dir_all(&cfg)?;
        cfg.push("todos.json");
        Ok(cfg)
    } else {
        // fallback to current dir
        Ok(PathBuf::from("todos.json"))
    }
}

pub fn load_data(path: &Path) -> Result<Vec<ToDo>> {
    if !path.exists() {
        return Ok(vec![]);
    }
    let data = fs::read_to_string(path).with_context(|| format!("Failed reading todo file at {}", path.display()))?;
    if data.trim().is_empty() {
        return Ok(vec![]);
    }
    let todos = serde_json::from_str(&data).context("Failed parsing todos JSON")?;
    Ok(todos)
}

pub fn save_data(path: &Path, todos: &[ToDo]) -> Result<()> {
    let data = serde_json::to_string_pretty(todos).context("Failed serializing todos")?;
    fs::write(path, data).with_context(|| format!("Failed writing todos to {}", path.display()))?;
    Ok(())
}
