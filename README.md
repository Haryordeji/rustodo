# rustodo
A terminal-based to-do list application written in Rust

## Commands

- **today [offset]**: Display tasks for today or a specific day (offset in days, e.g., `-1` for yesterday).
- **add <task>**: Add a new task.
- **mark done/undone <index>**: Mark a task as done or undone.
- **delete <section> <index>**: Delete a task (`section`: 'done' or 'undone').
- **edit <section> <index> <new description>**: Edit an existing task.
- **reset**: Delete all tasks and start fresh.
- **quit** or **exit**: Exit the application.

## Usage

1. Run the application: `cargo run`
2. Enter commands at the prompt.
3. Tasks are automatically saved between sessions.

## Features

- Persistent storage using JSON.
- Automatic task carryover.
- Task archiving after one week.
- Historical view of tasks.
