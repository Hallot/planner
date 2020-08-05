use serde::Serialize;

use crate::task;

#[derive(Serialize)]
pub struct StaffedTask {
    task: task::Task,
    team: Vec<task::Participant>
}

#[derive(Serialize)]
pub struct Planning {
    pub tasks: Vec<StaffedTask>
}