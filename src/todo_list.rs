use std::fs;
use std::io;
use chrono::Local;
use serde::{Deserialize, Serialize};
use crate::task::Task;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, description: String) {
        let task = Task {
            description,
            completed: false,
            date: Local::now().date_naive(),
        };
        self.tasks.push(task);
    }

    pub fn complete_task(&mut self, index: usize) -> Result<(), String> {
        if index < self.tasks.len() {
            self.tasks[index].completed = true;
            Ok(())
        } else {
            Err("Invalid task index".to_string())
        }
    }

    pub fn display_today_tasks(&self) {
        let today = Local::now().date_naive();
        let today_tasks: Vec<&Task> = self.tasks.iter()
            .filter(|task| task.date == today && !task.completed)
            .collect();

        if today_tasks.is_empty() {
            println!("No tasks for today!");
        } else {
            println!("Today's tasks:");
            for (index, task) in today_tasks.iter().enumerate() {
                println!("{}. {}", index + 1, task.description);
            }
        }
    }

    pub fn carry_over_tasks(&mut self) {
        let today = Local::now().date_naive();
        for task in &mut self.tasks {
            if !task.completed && task.date < today {
                task.date = today;
            }
        }
    }
}

pub fn load_todo_list() -> TodoList {
    match fs::read_to_string("todo_list.json") {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| TodoList::new()),
        Err(_) => TodoList::new(),
    }
}

pub fn save_todo_list(todo_list: &TodoList) -> io::Result<()> {
    let json = serde_json::to_string(todo_list)?;
    fs::write("todo_list.json", json)
}