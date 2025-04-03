use chrono::{DateTime, Duration, Local, NaiveDateTime};  // For date/time handling
use clap::{Parser, Subcommand};                // For CLI argument parsing
use colored::*;                                // For terminal colors
use console::Emoji;                           // For emoji support
use dialoguer::{Input, MultiSelect, Select};  // For interactive CLI prompts
use notify_rust::Notification;                // For system notifications
use serde::{Deserialize, Serialize};          // For JSON serialization
use std::{fs, path::PathBuf, thread, time};   // For file system operations and threading

// Define emoji constants for consistent usage throughout the app
static SPARKLES: Emoji<'_, '_> = Emoji("✨ ", "");
static ROCKET: Emoji<'_, '_> = Emoji("🚀 ", "");
static CHECKMARK: Emoji<'_, '_> = Emoji("✅ ", "");
static CALENDAR: Emoji<'_, '_> = Emoji("📅 ", "");
static FIRE: Emoji<'_, '_> = Emoji("🔥 ", "");
static CLOCK: Emoji<'_, '_> = Emoji("⏰ ", "");
static TAG: Emoji<'_, '_> = Emoji("🏷️ ", "");

// Category represents a task category with associated color and emoji
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Category {
    name: String,
    color: String,
    emoji: String,
}

// TimeEntry represents a single time tracking session
#[derive(Debug, Serialize, Deserialize, Clone)]
struct TimeEntry {
    start_time: DateTime<Local>,
    end_time: Option<DateTime<Local>>,
    duration: Option<Duration>,
}

// Task struct represents a single task in the system
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: usize,                                // Unique identifier for the task
    title: String,                            // Task title
    description: Option<String>,              // Optional task description
    priority: Priority,                       // Task priority level
    status: Status,                           // Current task status
    due_date: Option<DateTime<Local>>,        // Optional due date
    created_at: DateTime<Local>,              // Creation timestamp
    categories: Vec<Category>,                // Task categories/tags
    time_entries: Vec<TimeEntry>,             // Time tracking entries
    current_time_entry: Option<TimeEntry>,    // Currently running time entry
    last_notification: Option<DateTime<Local>>, // Last notification sent
}

// Priority enum defines possible priority levels for tasks
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

// Status enum defines possible states for a task
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
enum Status {
    Todo,
    InProgress,
    Done,
}

// CLI struct for parsing command line arguments
#[derive(Parser)]
#[command(
    name = "vibe_tasks",
    about = "A vibey task manager for good vibes only ✨",
    version = "1.0"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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
    #[command(about = "Add categories to a task")]
    AddCategories { id: usize },
    #[command(about = "Start time tracking for a task")]
    StartTime { id: usize },
    #[command(about = "Stop time tracking for a task")]
    StopTime { id: usize },
    #[command(about = "Show time tracking summary for a task")]
    TimeReport { id: usize },
    #[command(about = "Check for due tasks and send notifications")]
    CheckNotifications,
}

// TaskManager handles all task-related operations and storage
struct TaskManager {
    tasks: Vec<Task>,
    file_path: PathBuf,
}

impl TaskManager {
    // Creates a new TaskManager instance
    fn new() -> Self {
        let home_dir = dirs::home_dir().expect("Could not find home directory");
        let file_path = home_dir.join(".vibe_tasks.json");
        
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

    // Adds categories to a task
    fn add_categories(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            // Predefined categories with colors and emojis
            let available_categories = vec![
                Category {
                    name: "Work".to_string(),
                    color: "blue".to_string(),
                    emoji: "💼".to_string(),
                },
                Category {
                    name: "Personal".to_string(),
                    color: "green".to_string(),
                    emoji: "🏠".to_string(),
                },
                Category {
                    name: "Study".to_string(),
                    color: "yellow".to_string(),
                    emoji: "📚".to_string(),
                },
                Category {
                    name: "Health".to_string(),
                    color: "red".to_string(),
                    emoji: "💪".to_string(),
                },
                Category {
                    name: "Shopping".to_string(),
                    color: "cyan".to_string(),
                    emoji: "🛒".to_string(),
                },
            ];

            let category_names: Vec<String> = available_categories
                .iter()
                .map(|c| format!("{} {}", c.emoji, c.name))
                .collect();

            let selections = MultiSelect::new()
                .with_prompt(format!("{} Select categories", TAG))
                .items(&category_names)
                .interact()
                .unwrap();

            task.categories = selections
                .iter()
                .map(|&i| available_categories[i].clone())
                .collect();

            self.save();
            println!("{} Categories updated!", CHECKMARK);
        } else {
            println!("Task not found!");
        }
    }

    // Starts time tracking for a task
    fn start_time_tracking(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            if task.current_time_entry.is_some() {
                println!("Time tracking is already running for this task!");
                return;
            }

            let time_entry = TimeEntry {
                start_time: Local::now(),
                end_time: None,
                duration: None,
            };

            task.current_time_entry = Some(time_entry);
            self.save();
            println!("{} Time tracking started!", CLOCK);
        } else {
            println!("Task not found!");
        }
    }

    // Stops time tracking for a task
    fn stop_time_tracking(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            if let Some(mut current_entry) = task.current_time_entry.take() {
                let end_time = Local::now();
                current_entry.end_time = Some(end_time);
                current_entry.duration = Some(end_time - current_entry.start_time);
                task.time_entries.push(current_entry);
                self.save();
                println!("{} Time tracking stopped!", CLOCK);
            } else {
                println!("No active time tracking for this task!");
            }
        } else {
            println!("Task not found!");
        }
    }

    // Generates a time report for a task
    fn generate_time_report(&self, id: usize) {
        if let Some(task) = self.tasks.iter().find(|t| t.id == id) {
            println!("\n{}", "=".repeat(50).cyan());
            println!("Time Report for Task #{}: {}", task.id, task.title.bold());
            
            if task.time_entries.is_empty() {
                println!("No time entries recorded for this task.");
                return;
            }

            let mut total_duration = Duration::zero();
            for (i, entry) in task.time_entries.iter().enumerate() {
                if let Some(duration) = entry.duration {
                    total_duration = total_duration + duration;
                    println!("\nSession {}:", i + 1);
                    println!("Start: {}", entry.start_time.format("%Y-%m-%d %H:%M:%S"));
                    if let Some(end) = entry.end_time {
                        println!("End: {}", end.format("%Y-%m-%d %H:%M:%S"));
                    }
                    println!("Duration: {:.2} hours", duration.num_minutes() as f64 / 60.0);
                }
            }

            if let Some(current) = &task.current_time_entry {
                println!("\nCurrent session:");
                println!("Started: {}", current.start_time.format("%Y-%m-%d %H:%M:%S"));
                println!("Running for: {:.2} hours", 
                    (Local::now() - current.start_time).num_minutes() as f64 / 60.0);
            }

            println!("\nTotal time spent: {:.2} hours", total_duration.num_minutes() as f64 / 60.0);
            println!("{}", "=".repeat(50).cyan());
        } else {
            println!("Task not found!");
        }
    }

    // Checks for tasks that need notifications
    fn check_notifications(&mut self) {
        // First, collect all tasks that need notifications
        let notifications: Vec<(String, String)> = self.tasks.iter()
            .filter_map(|task| {
                if let Some(due_date) = task.due_date {
                    let now = Local::now();
                    let time_until_due = due_date - now;

                    // Check if task is due within 24 hours
                    if time_until_due.num_hours() <= 24 && time_until_due.num_hours() >= 0 {
                        let should_notify = match task.last_notification {
                            Some(last) => (now - last).num_hours() >= 6,
                            None => true,
                        };

                        if should_notify {
                            let notification_text = format!(
                                "Task '{}' is due {}!", 
                                task.title,
                                if time_until_due.num_hours() == 0 {
                                    "now".to_string()
                                } else {
                                    format!("in {} hours", time_until_due.num_hours())
                                }
                            );
                            return Some((task.id.to_string(), notification_text));
                        }
                    }
                }
                None
            })
            .collect();

        // Then, send notifications and update last_notification times
        for (task_id, notification_text) in notifications {
            match Notification::new()
                .summary("Task Due Soon!")
                .body(&notification_text)
                .icon("calendar")
                .show() 
            {
                Ok(_) => {
                    if let Some(task) = self.tasks.iter_mut().find(|t| t.id.to_string() == task_id) {
                        task.last_notification = Some(Local::now());
                    }
                },
                Err(e) => println!("Failed to send notification: {}", e),
            }
        }
        
        // Save any updates to notification times
        self.save();
    }

    // Modified add_task method to handle categories after task creation
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

        // Create the task
        let task_id = self.tasks.len() + 1;
        let task = Task {
            id: task_id,
            title,
            description: if description.is_empty() { None } else { Some(description) },
            priority,
            status: Status::Todo,
            due_date,
            created_at: Local::now(),
            categories: Vec::new(),
            time_entries: Vec::new(),
            current_time_entry: None,
            last_notification: None,
        };

        self.tasks.push(task);
        self.save();
        println!("{} Task added successfully!", CHECKMARK);
        
        // Add categories as a separate step
        self.add_categories(task_id);
    }

    // Modified list_tasks method to show categories and time tracking
    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found. Add some tasks to get started! ✨");
            return;
        }

        for task in &self.tasks {
            let status_str = match task.status {
                Status::Todo => "TODO".red(),
                Status::InProgress => "IN PROGRESS".yellow(),
                Status::Done => "DONE".green(),
            };

            let priority_str = match task.priority {
                Priority::Low => "LOW".blue(),
                Priority::Medium => "MEDIUM".yellow(),
                Priority::High => "HIGH".red(),
                Priority::Urgent => "URGENT".red().bold(),
            };

            println!("\n{}", "=".repeat(50).cyan());
            println!("Task #{}: {}", task.id, task.title.bold());
            if let Some(desc) = &task.description {
                println!("Description: {}", desc);
            }
            println!("Priority: {}", priority_str);
            println!("Status: {}", status_str);
            
            // Display categories
            if !task.categories.is_empty() {
                print!("Categories: ");
                for (i, category) in task.categories.iter().enumerate() {
                    if i > 0 { print!(", "); }
                    print!("{} {}", category.emoji, category.name);
                }
                println!();
            }

            // Display time tracking status
            if let Some(current) = &task.current_time_entry {
                println!("🔄 Currently tracking time (started: {})", 
                    current.start_time.format("%H:%M:%S"));
            }
            if !task.time_entries.is_empty() {
                let total_duration: Duration = task.time_entries
                    .iter()
                    .filter_map(|e| e.duration)
                    .sum();
                println!("⏱️ Total time: {:.2} hours", total_duration.num_minutes() as f64 / 60.0);
            }

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

fn main() {
    let cli = Cli::parse();
    let mut task_manager = TaskManager::new();

    match cli.command {
        Commands::Add => task_manager.add_task(),
        Commands::List => task_manager.list_tasks(),
        Commands::Complete { id } => task_manager.complete_task(id),
        Commands::Status { id } => task_manager.update_status(id),
        Commands::Delete { id } => task_manager.delete_task(id),
        Commands::AddCategories { id } => task_manager.add_categories(id),
        Commands::StartTime { id } => task_manager.start_time_tracking(id),
        Commands::StopTime { id } => task_manager.stop_time_tracking(id),
        Commands::TimeReport { id } => task_manager.generate_time_report(id),
        Commands::CheckNotifications => task_manager.check_notifications(),
    }
}