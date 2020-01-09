//! lol
//! # Licence

#![warn(clippy::all)]
#![allow(clippy::suspicious_else_formatting)]

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![warn(missing_docs)]
#![warn(future_incompatible)]

#![recursion_limit="256"]


use
{
  std::
  {
    time::
    {
      SystemTime,
    },
  },
};

/// Proof of work.
pub enum      Proof
{
  Trust
  {
    time:                               SystemTime,
  },
}

pub struct    Points                    ( usize );
pub struct    TaskID                    ( usize );
pub struct    UserID                    ( usize );

/// …
pub struct    User
{
  name:                                 String,
  points:                               Points,
}

/// Data about registration of `Task`.
pub struct    Registration
{
  byUser:                               UserID,
  time:                                 SystemTime,
}

/// Data about assignment of `Task`.
pub struct    Assignment
{
  byUser:                               UserID,
  toUser:                               UserID,
  time:                                 SystemTime,
}

/// Data about deadline of `Task`.
pub struct    Deadline
{
  earlyBird:                            ( Points, SystemTime, ),
  finalBird:                            ( Points, SystemTime, ),
}

/// One single task.
pub struct    Task
{
  registed:                             Registration,
  assignment:                           Option  < Assignment    >,
  deadline:                             Deadline,
}

/// …
pub struct    ToDoList
{
  listOfUsers:                          Vec < User  >,
  listOfTasks:                          Vec < Task  >,
}

/// Constructor for `ToDoList` …
///
/// # Arguments
/// * `` –
pub fn  ToDoList  ()
->  ToDoList
{
  ToDoList
  {
    listOfUsers:                        Vec::new(),
    listOfTasks:                        Vec::new(),
  }
}

impl          ToDoList
{

}
