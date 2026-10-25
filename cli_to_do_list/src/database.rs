
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

    pub fn new() -> Database {
        Database { 
            tasks: Vec::new() 
        }
    }

    pub fn add(){

    }

    pub fn update(){

    }

    pub fn delete(){

    }

    pub fn get_all(){

    }

    
}