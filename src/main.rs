use chrono::{DateTime, Local, NaiveDateTime};  // For date/time handling
use clap::{Parser, Subcommand};                // For CLI argument parsing
use colored::*;                                // For terminal colors
use console::Emoji;                           // For emoji support
use dialoguer::{Input, Select};               // For interactive CLI prompts
use serde::{Deserialize, Serialize};          // For JSON serialization
use std::{fs, path::PathBuf};                 // For file system operations

// Define emoji constants for consistent usage throughout the app
static SPARKLES: Emoji<'_, '_> = Emoji("✨ ", "");
static ROCKET: Emoji<'_, '_> = Emoji("🚀 ", "");
static CHECKMARK: Emoji<'_, '_> = Emoji("✅ ", "");
static CALENDAR: Emoji<'_, '_> = Emoji("📅 ", "");
static FIRE: Emoji<'_, '_> = Emoji("🔥 ", "");

// Task struct represents a single task in the system
// Derives necessary traits for debugging, serialization, and cloning
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: usize,                                // Unique identifier for the task
    title: String,                            // Task title
    description: Option<String>,              // Optional task description
    priority: Priority,                       // Task priority level
    status: Status,                           // Current task status
    due_date: Option<DateTime<Local>>,        // Optional due date
    created_at: DateTime<Local>,              // Creation timestamp
}

// Priority enum defines possible priority levels for tasks
// Derives traits for comparison, serialization, and cloning
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

// Status enum defines possible states for a task
// Derives traits for comparison, serialization, and cloning
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
enum Status {
    Todo,
    InProgress,
    Done,
}

// CLI struct for parsing command line arguments using clap
// The derive macro generates the argument parsing code
#[derive(Parser)]
#[command(
    name = "vibe_tasks",
    about = "A vibey task manager for good vibes only ✨",
    version = "1.0"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,  // Subcommands for different operations
}

// Enum defining all available CLI commands
#[derive(Subcommand)]
enum Commands {
    #[command(about = "Add a new task")]
    Add,
    #[command(about = "List all tasks")]
    List,
    #[command(about = "Mark a task as complete")]
    Complete { id: usize },
    #[command(about = "Update task status")]
    Status { id: usize },
    #[command(about = "Delete a task")]
    Delete { id: usize },
}

// TaskManager handles all task-related operations and storage
struct TaskManager {
    tasks: Vec<Task>,          // Vector storing all tasks
    file_path: PathBuf,        // Path to the storage file
}

impl TaskManager {
    // Creates a new TaskManager instance
    // Initializes storage and loads existing tasks if any
    fn new() -> Self {
        let home_dir = dirs::home_dir().expect("Could not find home directory");
        let file_path = home_dir.join(".vibe_tasks.json");
        
        // Load existing tasks or start with empty vector
        let tasks = if file_path.exists() {
            let data = fs::read_to_string(&file_path).expect("Failed to read tasks file");
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Vec::new()
        };

        TaskManager { tasks, file_path }
    }

    // Saves current tasks to the JSON file
    fn save(&self) {
        let data = serde_json::to_string_pretty(&self.tasks).expect("Failed to serialize tasks");
        fs::write(&self.file_path, data).expect("Failed to save tasks");
    }

    // Handles the interactive process of adding a new task
    fn add_task(&mut self) {
        // Get task title with interactive prompt
        let title: String = Input::new()
            .with_prompt(format!("{} Task title", SPARKLES))
            .interact()
            .unwrap();

        // Get optional task description
        let description: String = Input::new()
            .with_prompt(format!("{} Description (optional)", ROCKET))
            .allow_empty(true)
            .interact()
            .unwrap();

        // Priority selection using interactive menu
        let priorities = vec!["Low", "Medium", "High", "Urgent"];
        let priority_idx = Select::new()
            .with_prompt(format!("{} Select priority", FIRE))
            .items(&priorities)
            .default(0)
            .interact()
            .unwrap();

        // Convert selection index to Priority enum
        let priority = match priority_idx {
            0 => Priority::Low,
            1 => Priority::Medium,
            2 => Priority::High,
            3 => Priority::Urgent,
            _ => Priority::Medium,
        };

        // Get optional due date with specific format
        let due_date: String = Input::new()
            .with_prompt(format!("{} Due date (YYYY-MM-DD HH:MM, optional)", CALENDAR))
            .allow_empty(true)
            .interact()
            .unwrap();

        // Parse and validate due date if provided
        let due_date = if !due_date.is_empty() {
            match NaiveDateTime::parse_from_str(&due_date, "%Y-%m-%d %H:%M") {
                Ok(dt) => Some(DateTime::from_naive_utc_and_offset(dt, Local::now().offset().clone())),
                Err(_) => None,
            }
        } else {
            None
        };

        // Create and add the new task
        let task = Task {
            id: self.tasks.len() + 1,
            title,
            description: if description.is_empty() { None } else { Some(description) },
            priority,
            status: Status::Todo,
            due_date,
            created_at: Local::now(),
        };

        self.tasks.push(task);
        self.save();
        println!("{} Task added successfully!", CHECKMARK);
    }

    // Displays all tasks with formatting and color coding
    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found. Add some tasks to get started! ✨");
            return;
        }

        for task in &self.tasks {
            // Color-coded status display
            let status_str = match task.status {
                Status::Todo => "TODO".red(),
                Status::InProgress => "IN PROGRESS".yellow(),
                Status::Done => "DONE".green(),
            };

            // Color-coded priority display
            let priority_str = match task.priority {
                Priority::Low => "LOW".blue(),
                Priority::Medium => "MEDIUM".yellow(),
                Priority::High => "HIGH".red(),
                Priority::Urgent => "URGENT".red().bold(),
            };

            // Format and display task details
            println!("\n{}", "=".repeat(50).cyan());
            println!("Task #{}: {}", task.id, task.title.bold());
            if let Some(desc) = &task.description {
                println!("Description: {}", desc);
            }
            println!("Priority: {}", priority_str);
            println!("Status: {}", status_str);
            if let Some(due) = task.due_date {
                println!("Due: {}", due.format("%Y-%m-%d %H:%M").to_string().magenta());
            }
            println!("Created: {}", task.created_at.format("%Y-%m-%d %H:%M"));
        }
        println!("{}", "=".repeat(50).cyan());
    }

    // Marks a specific task as complete
    fn complete_task(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.status = Status::Done;
            self.save();
            println!("{} Task {} marked as complete!", CHECKMARK, id);
        } else {
            println!("Task not found!");
        }
    }

    // Updates the status of a specific task using interactive menu
    fn update_status(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            let statuses = vec!["Todo", "In Progress", "Done"];
            let status_idx = Select::new()
                .with_prompt(format!("{} Select new status", ROCKET))
                .items(&statuses)
                .default(0)
                .interact()
                .unwrap();

            // Convert selection index to Status enum
            task.status = match status_idx {
                0 => Status::Todo,
                1 => Status::InProgress,
                2 => Status::Done,
                _ => Status::Todo,
            };
            self.save();
            println!("{} Task status updated!", CHECKMARK);
        } else {
            println!("Task not found!");
        }
    }

    // Removes a task from the list
    fn delete_task(&mut self, id: usize) {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            self.save();
            println!("{} Task {} deleted!", CHECKMARK, id);
        } else {
            println!("Task not found!");
        }
    }
}

// Main function: entry point of the program
fn main() {
    // Parse command line arguments
    let cli = Cli::parse();
    let mut task_manager = TaskManager::new();

    // Route to appropriate function based on command
    match cli.command {
        Commands::Add => task_manager.add_task(),
        Commands::List => task_manager.list_tasks(),
        Commands::Complete { id } => task_manager.complete_task(id),
        Commands::Status { id } => task_manager.update_status(id),
        Commands::Delete { id } => task_manager.delete_task(id),
    }
}