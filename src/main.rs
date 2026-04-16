use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand};
use chrono::Local;
use colored::*;

#[derive(Parser)]
#[command(name = "Todo App")]
#[command(about = "A simple command-line todo application in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo item
    Add {
        #[arg(help = "Description of the todo item")]
        description: String,
    },
    /// List all todo items
    List,
    /// Mark a todo item as complete
    Complete {
        #[arg(help = "Index of the todo item to complete")]
        index: usize,
    },
    /// Remove a todo item
    Remove {
        #[arg(help = "Index of the todo item to remove")]
        index: usize,
    },
    /// Clear all todo items
    Clear,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Todo {
    id: u32,
    description: String,
    completed: bool,
    created_at: String,
    completed_at: Option<String>,
}

struct TodoApp {
    todos: Vec<Todo>,
    file_path: String,
}

impl TodoApp {
    fn new() -> Self {
        let file_path = "todos.json".to_string();
        let todos = TodoApp::load_todos(&file_path);
        TodoApp { todos, file_path }
    }

    fn load_todos(file_path: &str) -> Vec<Todo> {
        if Path::new(file_path).exists() {
            match fs::read_to_string(file_path) {
                Ok(content) => match serde_json::from_str(&content) {
                    Ok(todos) => todos,
                    Err(_) => {
                        println!("{}", "Error parsing todos.json, starting fresh.".yellow());
                        Vec::new()
                    }
                },
                Err(_) => Vec::new(),
            }
        } else {
            Vec::new()
        }
    }

    fn save_todos(&self) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self.todos)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }

    fn add_todo(&mut self, description: String) {
        let id = self.todos.len() as u32 + 1;
        let todo = Todo {
            id,
            description: description.clone(),
            completed: false,
            created_at: Local::now().to_rfc3339(),
            completed_at: None,
        };
        self.todos.push(todo);
        self.save_todos().expect("Failed to save todos");
        println!(
            "{}",
            format!("✓ Added: {}", description).green()
        );
    }

    fn list_todos(&self) {
        if self.todos.is_empty() {
            println!("{}", "No todos found. Add one with 'todo add <description>'".yellow());
            return;
        }

        println!("\n{}", "═══════════════════════════════════════════════════".cyan());
        println!("{}", "  TODO LIST".cyan().bold());
        println!("{}\n", "═══════════════════════════════════════════════════".cyan());

        let mut completed_count = 0;
        let mut pending_count = 0;

        for (index, todo) in self.todos.iter().enumerate() {
            let status = if todo.completed {
                "[✓]".green()
            } else {
                "[ ]".yellow()
            };

            let desc = if todo.completed {
                todo.description.strikethrough().bright_black()
            } else {
                todo.description.white()
            };

            println!("  {} {}. {}", status, index + 1, desc);

            if todo.completed {
                completed_count += 1;
            } else {
                pending_count += 1;
            }
        }

        println!(
            "\n{}  {} pending, {} completed",
            "═══════════════════════════════════════════════════".cyan(),
            pending_count.to_string().yellow(),
            completed_count.to_string().green()
        );
        println!("{}\n", "═══════════════════════════════════════════════════".cyan());
    }

    fn complete_todo(&mut self, index: usize) {
        if index == 0 || index > self.todos.len() {
            println!(
                "{}",
                format!("Error: Invalid index {}. Use 'todo list' to see available items.", index).red()
            );
            return;
        }

        self.todos[index - 1].completed = true;
        self.todos[index - 1].completed_at = Some(Local::now().to_rfc3339());
        self.save_todos().expect("Failed to save todos");
        println!(
            "{}",
            format!("✓ Completed: {}", self.todos[index - 1].description).green()
        );
    }

    fn remove_todo(&mut self, index: usize) {
        if index == 0 || index > self.todos.len() {
            println!(
                "{}",
                format!("Error: Invalid index {}. Use 'todo list' to see available items.", index).red()
            );
            return;
        }

        let removed = self.todos.remove(index - 1);
        self.save_todos().expect("Failed to save todos");
        println!(
            "{}",
            format!("✓ Removed: {}", removed.description).green()
        );
    }

    fn clear_todos(&mut self) {
        self.todos.clear();
        self.save_todos().expect("Failed to save todos");
        println!("{}", "✓ All todos cleared!".green());
    }
}

fn main() {
    let cli = Cli::parse();
    let mut app = TodoApp::new();

    match cli.command {
        Commands::Add { description } => {
            app.add_todo(description);
        }
        Commands::List => {
            app.list_todos();
        }
        Commands::Complete { index } => {
            app.complete_todo(index);
        }
        Commands::Remove { index } => {
            app.remove_todo(index);
        }
        Commands::Clear => {
            app.clear_todos();
        }
    }
}
