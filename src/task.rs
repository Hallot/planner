use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum TaskType {
    EEP,
    TAVI,
    CathLab,
    SC,
    RX,
    Admin,
}

#[derive(Deserialize, Serialize)]
pub struct Task {
    r#type: TaskType,
    day: u32, // Index in the month, starting from 1
    duration: u32, // [h]
    participants_required: u32,
}

#[derive(Deserialize, Serialize)]
pub struct Participant {
    name: String,
    skills: Vec<TaskType>,
    hours_per_week: u32,
    days_unavailable: Vec<u32>,
}