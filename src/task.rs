// src/task.rs
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub enum TaskState {
    NotDone,
    Done,
    Deleted,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Task {
    pub description: String,
    pub state: TaskState,
    pub date: NaiveDate,
}

impl Task {
    pub fn new(description: String, date: NaiveDate) -> Self {
        Task {
            description,
            state: TaskState::NotDone,
            date,
        }
    }
    pub fn change_state(&mut self, new_state: TaskState) {
        self.state = new_state;
    }
}