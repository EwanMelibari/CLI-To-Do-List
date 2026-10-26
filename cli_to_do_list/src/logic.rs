use crate::database::Database;
use crate::database::Task;

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

    Ok(Response::Success(200))

}