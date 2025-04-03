# Rust CLI Task Manager ✨

A vibey command-line task manager built in Rust with interactive prompts, color coding, and emoji support. Keep track of your tasks with style! 🚀

## Features

### Core Features
- 📝 Add tasks with titles and optional descriptions
- 🎯 Set priority levels (Low, Medium, High, Urgent)
- 📊 Track task status (Todo, In Progress, Done)
- 📅 Add due dates to tasks
- 🎨 Color-coded output for better visibility
- 💾 Persistent storage in JSON format
- ✨ Interactive CLI with emoji indicators

### New Features
- 🏷️ **Task Categories**
  - Predefined categories with colors and emojis
  - Multiple categories per task
  - Work 💼, Personal 🏠, Study 📚, Health 💪, Shopping 🛒
  - Interactive category selection
  - Update categories anytime

- ⏱️ **Time Tracking**
  - Start/stop time tracking for tasks
  - Track multiple time sessions
  - View current running session
  - Detailed time reports with:
    - Individual session details
    - Start and end times
    - Session durations
    - Total time spent

- 🔔 **Smart Notifications**
  - Automatic notifications for tasks due within 24 hours
  - Smart notification spacing (prevents spam)
  - System notifications with task details
  - Due time countdown
  - Notification state persistence

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

### Basic Commands
```bash
# Adding a Task
cargo run -- add

# Listing Tasks
cargo run -- list

# Marking a Task as Complete
cargo run -- complete <task_id>

# Updating Task Status
cargo run -- status <task_id>

# Deleting a Task
cargo run -- delete <task_id>
```

### Category Management
```bash
# Add or Update Categories for a Task
cargo run -- add-categories <task_id>
```

### Time Tracking
```bash
# Start Time Tracking
cargo run -- start-time <task_id>

# Stop Time Tracking
cargo run -- stop-time <task_id>

# View Time Report
cargo run -- time-report <task_id>
```

### Notifications
```bash
# Check for Due Tasks
cargo run -- check-notifications
```

For automatic notifications, you can set up a cron job or scheduled task to run the check-notifications command periodically:

```bash
# Example cron job (runs every hour)
0 * * * * cd /path/to/vibe_tasks && cargo run -- check-notifications
```

## Task Display

Tasks are displayed with:
- Color-coded priorities (blue for Low, yellow for Medium, red for High, bold red for Urgent)
- Color-coded statuses (red for Todo, yellow for In Progress, green for Done)
- Category emojis and labels
- Time tracking information
- Due dates in magenta
- Creation timestamps

## Data Storage

Tasks are automatically saved to `~/.vibe_tasks.json` in your home directory. The data persists between program runs and includes:
- Task details
- Categories
- Time tracking history
- Notification states

## Dependencies

- `clap`: Command line argument parsing
- `colored`: Terminal colors
- `console`: Terminal styling and emoji support
- `dialoguer`: Interactive CLI prompts
- `serde`: JSON serialization
- `chrono`: Date/time handling
- `dirs`: Home directory detection
- `notify-rust`: System notifications

## Contributing

Contributions are welcome! Feel free to submit issues and pull requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.