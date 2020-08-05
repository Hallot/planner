use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::input_data;
use crate::planning;

pub fn simulated_annealing(input : &input_data::InputData) -> planning::Planning {
    let planning = generate_random_planning(&input);

    planning
}

fn generate_random_planning (input : &input_data::InputData) -> planning::Planning {
    let mut planning = planning::Planning {
        tasks: Vec::new()
    };

    let mut random_tasks = input.tasks.clone();
    random_tasks.shuffle(&mut thread_rng());
    
    for task in random_tasks.iter() {
        let mut staffed_task = planning::StaffedTask {
            task: task.clone(),
            team: Vec::new()
        };
        for participant in input.staff.iter() {
            if staffed_task.team.len() as u32 == staffed_task.task.participants_required {
                break;
            }
            // todo: make a method
            // todo: deal with participant been already assigned to a task at the same time
            if participant.skills.contains(&task.task_type) && !participant.days_unavailable.contains(&task.day) {
                staffed_task.team.push(participant.clone());
            }
        }
        planning.tasks.push(staffed_task);
    }

    planning
}