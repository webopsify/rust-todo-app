# Rust Todo Application

A simple yet powerful command-line todo application built with Rust. Manage your tasks efficiently with a clean CLI interface.

## 🚀 Features

- ✅ **Add todos** — Create new todo items with descriptions
- 📋 **List todos** — View all your todos with completion status
- ✓ **Complete todos** — Mark tasks as complete
- 🗑️ **Remove todos** — Delete individual todo items
- 🧹 **Clear all** — Remove all todos at once
- 💾 **Persistent storage** — Todos are saved to `todos.json`
- 🎨 **Colored output** — Beautiful terminal UI with colors and formatting
- ⏰ **Timestamps** — Track when todos were created and completed

## 📋 Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

## 🔧 Installation

Clone the repository:
```bash
git clone https://github.com/webopsify/rust-todo-app.git
cd rust-todo-app
```

Build the application:
```bash
cargo build --release
```

The binary will be available at `target/release/todo`

## 📖 Usage

### Add a new todo
```bash
cargo run -- add "Buy groceries"
```

### List all todos
```bash
cargo run -- list
```

### Mark a todo as complete
```bash
cargo run -- complete 1
```

### Remove a todo
```bash
cargo run -- remove 1
```

### Clear all todos
```bash
cargo run -- clear
```

## 📝 Examples

```bash
# Add multiple todos
$ cargo run -- add "Finish project report"
✓ Added: Finish project report

$ cargo run -- add "Review pull requests"
✓ Added: Review pull requests

$ cargo run -- add "Schedule team meeting"
✓ Added: Schedule team meeting

# List all todos
$ cargo run -- list

═══════════════════════════════════════════════════
  TODO LIST
═══════════════════════════════════════════════════

  [ ] 1. Finish project report
  [ ] 2. Review pull requests
  [ ] 3. Schedule team meeting

═══════════════════════════════════════════════════  3 pending, 0 completed
═══════════════════════════════════════════════════

# Complete a todo
$ cargo run -- complete 1
✓ Completed: Finish project report

# List updated todos
$ cargo run -- list

═══════════════════════════════════════════════════
  TODO LIST
═══════════════════════════════════════════════════

  [✓] 1. Finish project report
  [ ] 2. Review pull requests
  [ ] 3. Schedule team meeting

═══════════════════════════════════════════════════  2 pending, 1 completed
═══════════════════════════════════════════════════
```

## 🏗️ Project Structure

```
rust-todo-app/
├── Cargo.toml           # Project manifest and dependencies
├── src/
│   └── main.rs          # Main application code
├── README.md            # This file
└── todos.json           # Persisted todo data (created on first run)
```

## 🔍 Data Storage

Todos are stored in `todos.json` in the current directory. Each todo item contains:
- `id` — Unique identifier
- `description` — Task description
- `completed` — Completion status (true/false)
- `created_at` — ISO 8601 timestamp of creation
- `completed_at` — ISO 8601 timestamp of completion (if completed)

### Example `todos.json`:
```json
[
  {
    "id": 1,
    "description": "Finish project report",
    "completed": true,
    "created_at": "2024-04-16T13:14:00+05:30",
    "completed_at": "2024-04-16T13:15:00+05:30"
  },
  {
    "id": 2,
    "description": "Review pull requests",
    "completed": false,
    "created_at": "2024-04-16T13:14:05+05:30",
    "completed_at": null
  }
]
```

## 🛠️ Dependencies

- **serde** — Serialization/deserialization framework
- **serde_json** — JSON support for Serde
- **clap** — Command-line argument parsing
- **chrono** — Date and time handling
- **colored** — Terminal color support

## 🚀 Development

Run with debug output:
```bash
cargo run -- list
```

Run tests (if added):
```bash
cargo test
```

Format code:
```bash
cargo fmt
```

Check code:
```bash
cargo clippy
```

## 📦 Building for Release

Create an optimized release binary:
```bash
cargo build --release
```

Run the release binary:
```bash
./target/release/todo list
```

## 🎯 Future Enhancements

- [ ] Add priority levels (high, medium, low)
- [ ] Add due date support
- [ ] Add categories/tags for todos
- [ ] Add search/filter functionality
- [ ] Add configuration file for customization
- [ ] Add unit tests
- [ ] Add database backend (SQLite)
- [ ] Add REST API with web interface
- [ ] Add recurring todos
- [ ] Add reminders and notifications

## 📄 License

MIT License — See LICENSE file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📞 Support

For issues or questions, please open an issue on GitHub.

---

**Happy task management! 🎉**
