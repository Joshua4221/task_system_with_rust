use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Clone)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub status: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateTask {
    pub title: String,
    pub status: String,
    pub description: String,
}

#[derive(Clone)]
pub struct TaskController {
    task: Arc<Mutex<Vec<Option<Task>>>>,
}

impl TaskController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            task: Arc::default(),
        })
    }

    pub async fn creat_task(&self, task: CreateTask) -> Result<Task> {
        let mut store = self.task.lock().unwrap();

        let id = store.len() as u64;
        let task_data = Task {
            id,
            title: task.title,
            description: task.description,
            status: task.status,
        };

        store.push(Some(task_data.clone()));

        Ok(task_data)
    }

    pub async fn get_all_task(&self) -> Result<Vec<Task>> {
        let store = self.task.lock().unwrap();

        let tasks = store.iter().filter_map(|tk| tk.clone()).collect();

        Ok(tasks)
    }

    pub async fn get_single_task(&self, id: u64) -> Result<Task> {
        let mut store = self.task.lock().unwrap();

        let task = store.get_mut(id as usize).and_then(|tk| tk.clone());

        task.ok_or(Error::FailToGetTaskWithThisId)
    }

    pub async fn delete_task(&self, id: u64) -> Result<Task> {
        let mut store = self.task.lock().unwrap();

        let task_to_delete = store.get_mut(id as usize).and_then(|tk| tk.take());

        task_to_delete.ok_or(Error::FailToDeleteTaskById)
    }

    pub async fn update_task(&self, id: u64, task: CreateTask) -> Result<Task> {
        let mut store = self.task.lock().unwrap();

        if let Some(Some(existing_task)) = store.get_mut(id as usize) {
            existing_task.title = task.title;
            existing_task.description = task.description;
            existing_task.status = task.status;

            Ok(existing_task.clone())
        } else {
            Err(Error::FailToUpdateTask)
        }
    }
}
