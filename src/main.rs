use std::fs;
use std::io::{self, Write};
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,
    #[serde(with = "date_serializer")]
    date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    tasks: Vec<Task>,
}
mod date_serializer {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Serializer, Deserializer};

    pub fn serialize<S>(
        date: &NaiveDate,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.format("%Y-%m-%d").to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)
    }
}

impl TodoList {
    fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        let task = Task {
            description,
            completed: false,
            date: Local::now().date_naive(),
        };
        self.tasks.push(task);
    }

    fn complete_task(&mut self, index: usize) -> Result<(), String> {
        if index < self.tasks.len() {
            self.tasks[index].completed = true;
            Ok(())
        } else {
            Err("Invalid task index".to_string())
        }
    }

    fn display_today_tasks(&self) {
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

    fn carry_over_tasks(&mut self) {
        let today = Local::now().date_naive();
        for task in &mut self.tasks {
            if !task.completed && task.date < today {
                task.date = today;
            }
        }
    }
}

fn load_todo_list() -> TodoList {
    match fs::read_to_string("todo_list.json") {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| TodoList::new()),
        Err(_) => TodoList::new(),
    }
}

fn save_todo_list(todo_list: &TodoList) -> io::Result<()> {
    let json = serde_json::to_string(todo_list)?;
    fs::write("todo_list.json", json)
}

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