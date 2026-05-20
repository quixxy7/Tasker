mod cli;
mod tasks;
use clap::Parser;
use cli::{Cli, Commands};
use std::env;
use std::fs;
use tasks::{Task, TaskStorage};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init => {
            let mut path = env::current_dir().unwrap();
            path.push(".tsk");
            fs::create_dir(&path).unwrap();

            path.push("tasks.json");
            fs::write(path, "{\"next_id\": 1, \"tasks\": []}").unwrap();
        }
        Commands::Add { name } => {
            let mut storage = TaskStorage::load();
            let task = Task::new(storage.next_id, name);
            storage.tasks.push(task);
            storage.next_id += 1;
            storage.save();
        }
        Commands::List => {
            let storage = TaskStorage::load();
            for task in storage.tasks.iter() {
                println!("{:?}", task);
            }
        }
        Commands::Remove { id } => {
            let mut storage = TaskStorage::load();
            for i in 0..storage.tasks.len() {
                if storage.tasks[i].id == id {
                    storage.tasks.remove(i);
                }
            }
            storage.save();
            //TODO Parse from JSON, Remove by ID, Send to JSON
        }
    }
}
