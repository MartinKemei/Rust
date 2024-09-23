use clap::Parser;
use todo_list::task::{Task, TaskManager};

#[derive(Parser, Debug)]
#[command(author = "Your Name", version = "0.1.0", about = "A simple To-Do list application")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    Add(AddCommand),
    List(ListCommand),
    Complete(CompleteCommand),
    Delete(DeleteCommand),
}

#[derive(Parser, Debug)]
struct AddCommand {
    #[arg(short, long)]
    description: String,
}

#[derive(Parser, Debug)]
struct ListCommand {
    #[arg(short, long)]
    all: bool,
}

#[derive(Parser, Debug)]
struct CompleteCommand {
    #[arg(short, long)]
    id: u32,
}

#[derive(Parser, Debug)]
struct DeleteCommand {
    #[arg(short, long)]
    id: u32,
}