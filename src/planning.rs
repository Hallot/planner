use serde::Serialize;

use crate::task;

#[derive(Serialize)]
pub struct StaffedTask {
    pub task: task::Task,
    pub team: Vec<task::Participant>
}

#[derive(Serialize)]
pub struct Planning {
    pub tasks: Vec<StaffedTask>
}