//! lol
//! # Licence

#![warn(clippy::all)]
#![allow(clippy::suspicious_else_formatting)]

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![warn(missing_docs)]
#![warn(future_incompatible)]

use
{
  serde::
  {
    Deserialize,
    Serialize,
  },
  std::
  {
    fs::
    {
      File,
    },
    io::
    {
      Error,
      ErrorKind,
      Write,
    },
    time::
    {
      SystemTime,
    },
  },
};

/// Proof of work.
#[derive( Deserialize,  Serialize )]
pub enum      Proof
{
  /// Just trust, that this `Task` was done at the given `time`.
  Trust
  {
    /// This `Task` was allegedly done at this time.
    time:                               SystemTime,
  },
}

/// What you get for achieving this `Task`.
#[derive( Deserialize,  Serialize )]
pub struct    Credit                    ( Points  );

/// Stored points for everything so far.
#[derive( Deserialize,  Serialize )]
pub struct    Points                    ( usize );

/// Token to reference and access a `Task` in the `listOfTasks`.
#[derive( Deserialize,  Serialize )]
pub struct    TaskID                    ( usize );

/// Token to reference and access a `User` in the `listOfUsers`
#[derive( Deserialize,  Serialize )]
pub struct    UserID                    ( usize );

/// Token to reference and access a `Team` in the `listOfTeams`
#[derive( Deserialize,  Serialize )]
pub struct    TeamID                    ( usize );

/// Different types of achievements.
#[derive( Deserialize,  Serialize )]
pub enum      AchievementType
{
}

/// Data about an achievement a user might earn.
#[derive( Deserialize,  Serialize )]
pub struct    Achievement
{
  time:                                 SystemTime,
  this:                                 AchievementType,
}

/// Data about a single team stored in the `listOfTeams`.
#[derive( Deserialize,  Serialize )]
pub struct    Team
{
  name:                                 String,
  points:                               Points,
  achievements:                         Vec < Achievement >,
}

/// Data about a single user stored in the `listOfUsers`.
#[derive( Deserialize,  Serialize )]
pub struct    User
{
  name:                                 String,
  points:                               Points,
  achievements:                         Vec < Achievement >,
}

impl          User
{
  /// …
  ///
  /// # Arguments
  /// * `` –
  pub fn        earnCredit
  (
    &mut self,
    _credit:                            Credit,
  )
  ->  Self
  {
    unimplemented!()
  }
}

/// Data about registration of `Task`.
#[derive( Deserialize,  Serialize )]
pub struct    Registration
{
  byUser:                               UserID,
  time:                                 SystemTime,
}

/// Data about assignment of `Task`.
#[derive( Deserialize,  Serialize )]
pub struct    Assignment
{
  byUser:                               UserID,
  toTeam:                               TeamID,
  time:                                 SystemTime,
}

/// Data about deadline of `Task`.
#[derive( Deserialize,  Serialize )]
pub struct    Deadline
{
  earlyBird:                            ( Credit, SystemTime, ),
  finalBird:                            ( Credit, SystemTime, ),
  usualBird:                            Credit,
}

/// Data of a single task in the `listOfTasks`.
#[derive( Deserialize,  Serialize )]
pub struct    Task
{
  registed:                             Registration,
  assignment:                           Vec     < Assignment  >,
  deadline:                             Deadline,
  proof:                                Option  < Proof       >,
}

/// Procrastinator
#[derive( Deserialize,  Serialize )]
pub struct    Procrastinator
{
  /// List of `User`s of this `Procrastinator`-Instance.
  pub listOfUsers:                      Vec < Option  < User  > >,
  /// List of `Team`s of this `Procrastinator`-Instance.
  pub listOfTeams:                      Vec < Option  < Team  > >,
  /// List of `Task`s of this `Procrastinator`-Instance.
  pub listOfTasks:                      Vec < Option  < Task  > >,
}

/// Try to load the Initial Configuration of Procrastinator from file `path`.
///
/// # Arguments
/// * `path`                            – path to file, that should be loaded.
pub fn        loadProcrastinatorConfigFromFile
(
  path:                                 &'static str,
)
->  Result
    <
      Procrastinator,
      Error,
    >
{
  match File::open  ( path  )
  {
    Ok  ( file  )
    =>  {
          match ron::de::from_reader  ( file  )
          {
            Ok  ( procrastinator  )
            =>  Ok  ( procrastinator  ),
            Err ( error )
            =>  Err
                (
                  Error::new
                  (
                    ErrorKind::Other,
                    error,
                  )
                ),
          }
        },
    Err ( _     )
    =>  match File::create  ( path  )
        {
          Ok  ( mut file  )
          =>  {
                let     procrastinator
                =   Procrastinator  ( );
                match ron::ser::to_string ( &procrastinator )
                {
                  Ok  ( this  )
                  =>  match file
                              .write  ( this.as_bytes ( ) )
                      {
                        Ok  ( _     ) =>  Ok  ( procrastinator  ),
                        Err ( error ) =>  Err ( error           ),
                      },
                  Err ( error )
                  =>  Err
                      (
                        Error::new
                        (
                          ErrorKind::Other,
                          error,
                        )
                      ),
                }
              },
          Err ( error )
          =>  Err ( error )
        },
  }
}

/// Constructor for a very new `Procrastinator`.
pub fn        Procrastinator  ()
->  Procrastinator
{
  Procrastinator
  {
    listOfUsers:                        Vec::new(),
    listOfTeams:                        Vec::new(),
    listOfTasks:                        Vec::new(),
  }
}

impl          Procrastinator
{
  /// Resets all `Points` and `Achievement`s of every `Team` and `User` to an previous state or zero and recalculate them.
  ///
  /// # Arguments
  /// * `` –
  pub fn        calculateCredit
  (
    self,
  )
  ->  Self
  {
    unimplemented!()
  }
  /// Completion of `Task`s:
  /// 1.  `Task` must exist.
  /// 2.  `User` must exist.
  /// 3.  `Task` must not vanished.
  /// 4.  `User` must be in at least one `Team`, that was assigned to this `Task`.
  /// 5.  If `Task` was completed early, every `Team`, `User` is a member of, and every `User` of these `Team`s earns the early `Credit`.
  /// 6.  If `Task` was completed in time, every `Team`, `User` is a member of, and every `User` of these `Team`s earns the final `Credit`.
  /// 7.  Every `Team` and every `User` of each `Team` earn at least the usual `Credit`, if this `Task` was at least completed.
  /// # Arguments
  /// * `` –
  pub fn        didTask
  (
    &mut self,
    task:                               TaskID,
    user:                               UserID,
    _proof:                             Proof,
  )
  ->  Result
      <
        (),
        String,
      >
  {
    let     taskID                      =   task.0;
    let     userID                      =   user.0;
    if  self.listOfTasks.len  ( ) > taskID
    {
      if  self.listOfUsers.len  ( ) > userID
      {
        if  let Some  ( task  )         =   &self.listOfTasks  [ taskID  ]
        {
          if  let Some ( _winner  )
              =   task
                    .assignment
                    .iter()
                    .find
                    (
                      | &this |
                      this.toTeam.0 ==  userID
                    )
          {
            if  let Some  ( _user )     =   &self.listOfUsers  [ userID  ]
            {
              let     _now               =   SystemTime::now();
              Ok  ( ( ) )
            }
            else
            {
              Err
              (
                format!
                (
                  "User #{} cannot do task #{}, because they vanished.",
                  userID,
                  taskID,
                )
              )
            }
          }
          else
          {
            Err
            (
              format!
              (
                "User #{} cannot do task #{}, because they was not assigned to this task.",
                userID,
                taskID,
              )
            )
          }
        }
        else
        {
          Err
          (
            format!
            (
              "User #{} cannot do task #{}, because it vanished.",
              userID,
              taskID,
            )
          )
        }
      }
      else
      {
        Err
        (
          format!
          (
            "User #{} cannot do task #{}, because they does not exist.",
            userID,
            taskID,
          )
        )
      }
    }
    else
    {
      Err
      (
        format!
        (
          "User #{} cannot do task #{}, because it does not exist.",
          userID,
          taskID,
        )
      )
    }
  }
}
