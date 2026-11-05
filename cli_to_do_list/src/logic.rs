use crate::database::Database;
use crate::database::Task;
use crate::database::TaskState;

use std::result::Result;


#[derive(Debug)]
pub enum Exceptions {

    EmptyList,
    EmptyTitle,
    InvalidTaskID,
    StateDidNotChange

}

#[derive(Debug)]
pub enum Response {

    Error(Exceptions),
    Success(i32)
}


pub fn add_task(db: &mut Database, title: String) -> Result<Response, Response> {

    if title.trim().is_empty() {

        return Err(Response::Error(Exceptions::EmptyTitle));

    }

    db.add(title);

    Ok(Response::Success(201))

}

pub fn update_task(db: &mut Database, id: u32, new_title: Option<String>, new_state: Option<TaskState>) -> Result<Response, Response> {

    // Check if the new title is empty, if so return an error before proceeding to the DB
    if let Some(check_title) = &new_title {

        if check_title.trim().is_empty() {

            return Err(Response::Error(Exceptions::EmptyTitle));

        }
    }

    // check  if state didn't change
    if let Some(check_state) = &new_state {

        if let Some(task) = db.tasks.iter().find(|task| task.id == id) { // Find the task with the given ID in the database through Logic.rs to control the response flow

            if task.state == *check_state { // The pointer is dereferenced to compare the actual value of the TaskState enum variant

                return Err(Response::Error(Exceptions::StateDidNotChange));

            }

        }
    }
    
    // Update the task in the database and return the appropriate response based on whether the update was successful or not
    if db.update(id, new_title, new_state) {

        Ok(Response::Success(200))

    } else {

        Err(Response::Error(Exceptions::InvalidTaskID))

    }

}

pub fn remove_task(db: &mut Database, id: u32) -> Result<Response, Response> {

    if db.delete(id){

        Ok(Response::Success(204))

    } else {
        
        Err(Response::Error(Exceptions::InvalidTaskID))

    }

}

pub fn view_tasks(db: &mut Database) -> Result<Response, Response> {

    if db.tasks.is_empty() {

        return Err(Response::Error(Exceptions::EmptyList));

    } else {
    
        for task in &db.tasks {

            println!("ID: {}, Title: {}, State: {:?}", task.id, task.title, task.state);

        }

        Ok(Response::Success(200))

    }

}