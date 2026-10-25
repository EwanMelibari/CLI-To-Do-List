
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

    pub fn update(){

    }

    pub fn delete(){

    }

    pub fn get_all(&self) -> &Vec<Task> {

        &self.tasks

    }

    
}