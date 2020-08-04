pub enum TaskType {
    EEP,
    TAVI,
    CathLab,
    SC,
    RX,
    Admin,
}

pub struct Task {
    id: u64,
    name: String,
    r#type: TaskType,
    day: u32, // Index in the month, starting from 1
    duration: u32, // [h]
    participants_required: u32,
}

pub struct Participant {
    id: u64,
    name: String,
    skills: Vec<TaskType>,
    hours_per_week: u32,
    days_unavailable: Vec<u32>,
}