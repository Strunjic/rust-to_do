use crate::models::ToDo;
use crate::storage;
use anyhow::{Context, Result};
use std::path::Path;
use uuid::Uuid;

pub fn add(path: &Path, title: String, desc: Option<String>) -> Result<()> {
    let mut data = storage::load_data(path)?;
    let mut todo = ToDo::new(title);
    todo.description = desc;
    data.push(todo);
    storage::save_data(path, &data)?;
    println!("Added todo.");
    Ok(())
}

pub fn list(path: &Path, all: bool) -> Result<()> {
    let data = storage::load_data(path)?;
    if data.is_empty() {
        println!("No todos found.");
        return Ok(());
    }
    for d in data.iter().filter(|d| all || !d.completed) {
        println!(
            "{} [{}] - {}{}",
            d.id,
            if d.completed { "x" } else { " " },
            d.title,
            match &d.description {
                Some(d) => format!(" — {}", d),
                None => "".into(),
            }
        );
    }
    Ok(())
}

fn find_index_by_id(todos: &[ToDo], id_str: &str) -> Option<usize> {
    if let Ok(id) = Uuid::parse_str(id_str) {
        todos.iter().position(|t| t.id == id)
    } else {
        None
    }
}

pub fn done(path: &Path, id_str: &str) -> Result<()> {
    let mut data = storage::load_data(path)?;
    let idx = find_index_by_id(&data, id_str).context("Todo not found")?;
    data[idx].completed = true;
    storage::save_data(path, &data)?;
    println!("Marked as done.");
    Ok(())
}

pub fn remove(path: &Path, id_str: &str) -> Result<()> {
    let mut data = storage::load_data(path)?;
    let idx = find_index_by_id(&data, id_str).context("Todo not found")?;
    data.remove(idx);
    storage::save_data(path, &data)?;
    println!("Removed.");
    Ok(())
}

pub fn change(path: &Path, id_str: &str, title: String, desc: Option<String>) -> Result<()> {
    let mut data = storage::load_data(path)?;
    let mut todo = ToDo::new(title);
    todo.description = desc;
    remove(path, id_str)?;
    data.push(todo);
    storage::save_data(path, &data)?;
    println!("Changd todo");
    Ok(())
}

pub fn clear(path: &Path) -> Result<()> {
    storage::save_data(path, &Vec::new())?;
    println!("Cleared all todos.");
    Ok(())
}
