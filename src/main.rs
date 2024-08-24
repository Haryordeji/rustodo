mod todo_list;
mod task;
mod date_serializer;

use std::io::{self, Write};
use todo_list::{ load_todo_list, save_todo_list};

fn main() -> io::Result<()> {
    let mut todo_list = load_todo_list();
    todo_list.carry_over_tasks();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        match input.split_whitespace().next() {
            Some("today") => todo_list.display_today_tasks(),
            Some("add") => {
                let description = input.splitn(2, ' ').nth(1).unwrap_or("");
                if !description.is_empty() {
                    todo_list.add_task(description.to_string());
                    println!("Task added: {}", description);
                } else {
                    println!("Please provide a task description.");
                }
            }
            Some("done") => {
                if let Some(index) = input.split_whitespace().nth(1) {
                    if let Ok(index) = index.parse::<usize>() {
                        match todo_list.complete_task(index - 1) {
                            Ok(_) => println!("Task {} marked as complete.", index),
                            Err(e) => println!("Error: {}", e),
                        }
                    } else {
                        println!("Invalid task index.");
                    }
                } else {
                    println!("Please provide a task index.");
                }
            }
            Some("quit") | Some("exit") => break,
            _ => println!("Unknown command. Available commands: today, add <task>, done <index>, quit"),
        }

        save_todo_list(&todo_list)?;
    }

    Ok(())
}