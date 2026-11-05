
#[derive(Debug, PartialEq)]
pub enum TaskState {

    Incomplete,
    InProgress,
    Complete

}

pub struct Task {

    pub id: u32,
    pub title: String,
    pub state: TaskState

}

pub struct Database {

    pub tasks: Vec<Task>,
    // To generate unique IDs for tasks, it,s placed here to keep track of the next available ID
    // while Task struct is used to represent individual tasks, the Database struct is used to manage a collection of tasks 
    next_id: u32
}

impl Database{

    // Creates a new instance of the Database struct with an empty vector of tasks.
    pub fn new() -> Database {
        Database { 
            tasks: Vec::new(),
            next_id: 1 // Initialize the first id to be 1 by default
        }
    }

    pub fn add(&mut self, title: String) {

        let task = Task {

            id: self.next_id,
            title,
            state: TaskState::Incomplete // Set the initial state of the task to Incomplete by default

        };

        self.tasks.push(task);

        self.next_id += 1; // Increment the next_id for the next task

    }

    pub fn update(&mut self, id: u32, new_title: Option<String>, new_state: Option<TaskState>) -> bool{

        // find the given ID then update either the titel or the state
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {

            // Update title
            if let Some(title) = new_title {

                task.title = title;

            }

            // update state
            if let Some(state) = new_state {

                task.state = state;

            }
            // Return true to indicate that the update was successful
            true

        } else {
            // Return false to indicate that the task with the given ID was not found
            false

        }


    }

    pub fn delete(&mut self, id: u32) -> bool{

        if let Some(target) = self.tasks.iter().position(|task: &Task| task.id == id) {

            self.tasks.remove(target);

            true

        } else {

            false

        }

    }

    pub fn get_all(&self) -> &Vec<Task> {

        &self.tasks

    }

    
}