
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

    pub tasks: Vec<Task>
    
}

impl Database{

    
}