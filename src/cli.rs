use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Reset,
    Add { name: String },
    List { status: Option<String> },
    Status,
    Remove { id: u32 },
    Start { id: u32 },
    Done { id: u32 },
}
