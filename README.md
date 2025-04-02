# Rust CLI Task Manager ✨

A command-line task manager built in Rust with interactive prompts, color coding, and emoji support.

## Features

- 📝 Add tasks with titles and optional descriptions
- 🎯 Set priority levels (Low, Medium, High, Urgent)
- 📊 Track task status (Todo, In Progress, Done)
- 📅 Add due dates to tasks
- 🎨 Color-coded output for better visibility
- 💾 Persistent storage in JSON format
- ✨ Interactive CLI with emoji indicators

## Installation

1. Make sure you have Rust and Cargo installed. If not, install them from [rustup.rs](https://rustup.rs/).

2. Clone the repository:
```bash
git clone https://github.com/sim-lu/rust-cli-task-manager.git
cd rust-cli-task-manager
```

3. Build the project:
```bash
cargo build --release
```

The binary will be available at `target/release/vibe_tasks`.

## Usage

### Adding a Task
```bash
cargo run -- add
```
You'll be prompted to enter:
- Task title
- Description (optional)
- Priority level
- Due date (optional, format: YYYY-MM-DD HH:MM)

### Listing Tasks
```bash
cargo run -- list
```
Displays all tasks with their details, including:
- Task ID and title
- Description (if provided)
- Priority (color-coded)
- Status (color-coded)
- Due date (if set)
- Creation timestamp

### Marking a Task as Complete
```bash
cargo run -- complete <task_id>
```

### Updating Task Status
```bash
cargo run -- status <task_id>
```
Choose between:
- Todo
- In Progress
- Done

### Deleting a Task
```bash
cargo run -- delete <task_id>
```

## Data Storage

Tasks are automatically saved to `~/.vibe_tasks.json` in your home directory. The data persists between program runs.

## Dependencies

- `clap`: Command line argument parsing
- `colored`: Terminal colors
- `console`: Terminal styling and emoji support
- `dialoguer`: Interactive CLI prompts
- `serde`: JSON serialization
- `chrono`: Date/time handling
- `dirs`: Home directory detection

## Contributing

Contributions are welcome! Feel free to submit issues and pull requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
