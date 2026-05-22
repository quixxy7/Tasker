mod cli;
mod tasks;
use clap::Parser;
use cli::{Cli, Commands};
use std::env;
use std::fs;
use tasks::{Task, TaskStatus, TaskStorage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init => {
            let mut path = env::current_dir()?;
            path.push(".tsk");
            if !path.exists() {
                fs::create_dir(&path)?;
            } else {
                println!("Already initialized");
                return Ok(());
            }

            path.push("tasks.json");
            if !path.is_file() {
                fs::write(path, "{\"next_id\": 1, \"tasks\": []}")?;
            }
        }
        Commands::Add { name } => {
            let mut storage = TaskStorage::load()?;
            let task = Task::new(storage.next_id, name);
            storage.tasks.push(task);
            storage.next_id += 1;
            storage.save()?;
        }
        Commands::List => {
            let storage = TaskStorage::load()?;
            for task in storage.tasks.iter() {
                println!("{}", task);
            }
        }
        Commands::Remove { id } => {
            let mut storage = TaskStorage::load()?;
            let mut found: bool = false;
            for i in 0..storage.tasks.len() {
                if storage.tasks[i].id == id {
                    found = true;
                    storage.tasks.remove(i);
                    break;
                }
            }
            if !found {
                println!("Id has been not found. Task was not removed");
            }
            storage.save()?;
        }
        Commands::Start { id } => {
            let mut storage = TaskStorage::load()?;
            let mut found: bool = false;
            for task in storage.tasks.iter_mut() {
                if task.id == id {
                    found = true;
                    task.status = TaskStatus::InProgress;
                    break;
                }
            }
            if !found {
                println!("Id has been not found. Task was not changed");
            }
            storage.save()?
        }
        Commands::Done { id } => {
            let mut storage = TaskStorage::load()?;
            let mut found: bool = false;
            for task in storage.tasks.iter_mut() {
                if task.id == id {
                    found = true;
                    task.status = TaskStatus::Done;
                    break;
                }
            }
            if !found {
                println!("Id has been not found. Task was not changed");
            }
            storage.save()?
        }
    }
    Ok(())
}
