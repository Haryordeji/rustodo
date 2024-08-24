// src/main.rs
mod todo_list;
mod task;

use std::io::{self, Write};
use chrono::{Local, Duration, NaiveDate};
use todo_list::{TodoList, load_todo_list, save_todo_list, save_archived_tasks};
use task::TaskState;

fn main() -> io::Result<()> {
    let mut todo_list = load_todo_list()?;
    todo_list.carry_over_tasks();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.get(0).map(|s| *s) {
            Some("today") => {
                let offset = parts.get(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
                let date = Local::now().date_naive() + Duration::days(offset);
                display_tasks(&todo_list, date);
            },
            Some("add") => {
                if let Some(description) = input.splitn(2, ' ').nth(1) {
                    todo_list.add_task(description.to_string(), Local::now().date_naive());
                    println!("Task added: {}", description);
                } else {
                    println!("Please provide a task description.");
                }
            },
            Some("mark") => handle_mark_command(&mut todo_list, &parts),
            Some("delete") => handle_delete_command(&mut todo_list, &parts),
            Some("edit") => handle_edit_command(&mut todo_list, &parts),
            Some("quit") | Some("exit") => break,
            _ => println!("Unknown command. Available commands: today [offset], add <task>, mark done/undone <index>, delete <index>, edit <index> <new description>, quit"),
        }

        save_todo_list(&todo_list)?;
        let archived_tasks = todo_list.archive_old_tasks();
        if !archived_tasks.is_empty() {
            save_archived_tasks(&archived_tasks)?;
        }
    }

    Ok(())
}

fn display_tasks(todo_list: &TodoList, date: NaiveDate) {
    let (undone, done) = todo_list.get_tasks_for_date(date);

    println!("Tasks for {}:", date);
    
    println!("Not Done:");
    for (i, task) in undone.iter().enumerate() {
        println!("  {}. {}", i, task.description);
    }
    
    println!("Done:");
    for (i, task) in done.iter().enumerate() {
        println!("  {}. {}", i, task.description);
    }
}

fn handle_mark_command(todo_list: &mut TodoList, parts: &[&str]) {
    if parts.len() == 3 {
        let action = parts[1];
        if let Ok(index) = parts[2].parse::<usize>() {
            let date = Local::now().date_naive();
            let new_state = match action {
                "done" => TaskState::Done,
                "undone" => TaskState::NotDone,
                _ => {
                    println!("Invalid action. Use 'done' or 'undone'.");
                    return;
                }
            };
            match todo_list.change_task_state(date, index, new_state) {
                Ok(description) => println!("Task {} ({}) marked as {:?}", index, description, new_state),
                Err(e) => println!("Error: {}", e),
            }
        } else {
            println!("Invalid task index.");
        }
    } else {
        println!("Invalid command. Use 'mark done/undone <index>'");
    }
}

fn handle_delete_command(todo_list: &mut TodoList, parts: &[&str]) {
    if parts.len() == 3 {
        let section = parts[1];
        let is_done = match section {
            "done" => true,
            "undone" => false,
            _ => {
                println!("Invalid section. Use 'done' or 'undone'.");
                return;
            }
        };
        if let Ok(index) = parts[2].parse::<usize>() {
            match todo_list.delete_task(Local::now().date_naive(), index, is_done) {
                Ok(description) => println!("Task {} ({}) deleted", index, description),
                Err(e) => println!("Error: {}", e),
            }
        } else {
            println!("Invalid task index.");
        }
    } else {
        println!("Invalid command. Use 'delete <section> <index>'");
    }
}

fn handle_edit_command(todo_list: &mut TodoList, parts: &[&str]) {
    if parts.len() >= 4 {
        let section = parts[1];
        let is_done = match section {
            "done" => true,
            "undone" => false,
            _ => {
                println!("Invalid section. Use 'done' or 'undone'.");
                return;
            }
        };
        if let Ok(index) = parts[2].parse::<usize>() {
            let new_description = parts[3..].join(" ");
            match todo_list.edit_task(Local::now().date_naive(), index, is_done, new_description.clone()) {
                Ok(old_description) => println!("Task {} updated from '{}' to '{}'", index, old_description, new_description),
                Err(e) => println!("Error: {}", e),
            }
        } else {
            println!("Invalid task index.");
        }
    } else {
        println!("Invalid command. Use 'edit <section> <index> <new description>'");
    }
}