use crate::task;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct InputData {
    pub tasks: Vec<task::Task>,
    pub staff: Vec<task::Participant>
}