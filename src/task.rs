use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum TaskType {
    EEP,
    TAVI,
    CathLab,
    SC,
    RX,
    Admin,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Task {
    pub task_type: TaskType,
    pub day: u32, // Index in the month, starting from 1
    pub duration: u32, // [h]
    pub participants_required: u32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Participant {
    pub name: String,
    pub skills: Vec<TaskType>,
    pub hours_per_week: u32,
    pub days_unavailable: Vec<u32>,
}