use crate::database::Database;
use crate::database::Task;
use crate::logic::Exceptions::InvalidTaskID;

use std::result::Result;

pub enum Exceptions {

    EmptyList,
    EmptyTitle,
    InvalidTaskID,
    StateDidNotChange

}

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

pub fn update_task(){

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