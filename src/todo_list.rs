use chrono::{Local, NaiveDate, Duration};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use crate::task::{Task, TaskState};

#[derive(Debug, Serialize, Deserialize)]
pub struct DayTasks {
    undone: Vec<Task>,
    done: Vec<Task>,
}

impl DayTasks {
    fn new() -> Self {
        DayTasks {
            undone: Vec::new(),
            done: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    tasks: HashMap<NaiveDate, DayTasks>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { tasks: HashMap::new() }
    }

    pub fn add_task(&mut self, description: String, date: NaiveDate) {
        let task = Task::new(description, date);
        self.tasks.entry(date)
            .or_insert_with(DayTasks::new)
            .undone.push(task);
    }

    pub fn get_tasks_for_date(&self, date: NaiveDate) -> (&[Task], &[Task]) {
        self.tasks.get(&date)
            .map(|day_tasks| (&day_tasks.undone[..], &day_tasks.done[..]))
            .unwrap_or((&[], &[]))
    }

    pub fn change_task_state(&mut self, date: NaiveDate, index: usize, new_state: TaskState) -> Result<String, String> {
        if let Some(day_tasks) = self.tasks.get_mut(&date) {
            let (source, destination) = match new_state {
                TaskState::Done => (&mut day_tasks.undone, &mut day_tasks.done),
                TaskState::NotDone => (&mut day_tasks.done, &mut day_tasks.undone),
                TaskState::Deleted => return Err("Use delete_task to remove a task".to_string()),
            };

            if index < source.len() {
                let mut task = source.remove(index);
                let description = task.description.clone();
                task.change_state(new_state);
                destination.push(task);
                Ok(description)
            } else {
                Err("Invalid task index".to_string())
            }
        } else {
            Err("No tasks for the specified date".to_string())
        }
    }

    pub fn delete_task(&mut self, date: NaiveDate, index: usize, is_done: bool) -> Result<String, String> {
        if let Some(day_tasks) = self.tasks.get_mut(&date) {
            let source = if is_done { &mut day_tasks.done } else { &mut day_tasks.undone };
            if index < source.len() {
                let task = source.remove(index);
                Ok(task.description)
            } else {
                Err("Invalid task index".to_string())
            }
        } else {
            Err("No tasks for the specified date".to_string())
        }
    }

    pub fn edit_task(&mut self, date: NaiveDate, index: usize, is_done: bool, new_description: String) -> Result<String, String> {
        if let Some(day_tasks) = self.tasks.get_mut(&date) {
            let source = if is_done { &mut day_tasks.done } else { &mut day_tasks.undone };
            if index < source.len() {
                let old_description = std::mem::replace(&mut source[index].description, new_description);
                Ok(old_description)
            } else {
                Err("Invalid task index".to_string())
            }
        } else {
            Err("No tasks for the specified date".to_string())
        }
    }


    pub fn carry_over_tasks(&mut self) {
        let today = Local::now().date_naive();
        let yesterday = today - Duration::days(1);
        
        let tasks_to_carry = if let Some(yesterday_tasks) = self.tasks.get(&yesterday) {
            yesterday_tasks.undone.clone()
        } else {
            Vec::new()
        };

        for task in tasks_to_carry {
            self.add_task(task.description.clone(), today);
        }
    }

    pub fn archive_old_tasks(&mut self) -> Vec<Task> {
        let week_ago = Local::now().date_naive() - Duration::days(7);
        let mut archived_tasks = Vec::new();

        self.tasks.retain(|&date, day_tasks| {
            if date < week_ago {
                archived_tasks.extend(day_tasks.undone.drain(..));
                archived_tasks.extend(day_tasks.done.drain(..));
                false
            } else {
                true
            }
        });

        archived_tasks
    }
}

pub fn load_todo_list() -> io::Result<TodoList> {
    match fs::read_to_string("todo_list.json") {
        Ok(contents) => serde_json::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(TodoList::new()),
        Err(e) => Err(e),
    }
}

pub fn save_todo_list(todo_list: &TodoList) -> io::Result<()> {
    let json = serde_json::to_string(todo_list)?;
    fs::write("todo_list.json", json)
}

pub fn reset_todo_list() -> io::Result<()> {
    fs::remove_file("todo_list.json")?;
    fs::remove_file("archive.json")?;
    Ok(())
}

pub fn save_archived_tasks(archived_tasks: &[Task]) -> io::Result<()> {
    let mut existing_archived_tasks = match fs::read_to_string("archived_tasks.json") {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    };

    existing_archived_tasks.extend_from_slice(archived_tasks);

    let json = serde_json::to_string(&existing_archived_tasks)?;
    fs::write("archived_tasks.json", json)
}