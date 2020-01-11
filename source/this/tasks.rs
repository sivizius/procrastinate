use
{
  serde::
  {
    Deserialize,
    Serialize,
  },
  std::
  {
    time::
    {
      SystemTime,
    },
  },
  super::
  {
    control::
    {
      Assignment,
      Registration,
    },
    credit::
    {
      Credit,
    },
    proof::
    {
      Proof,
    },
  },
};

/// Data about deadline of `Task`.
#[derive( Deserialize,  Serialize )]
pub struct    Deadline
{
  pub earlyBird:                        ( Credit, SystemTime, ),
  pub finalBird:                        ( Credit, SystemTime, ),
  pub usualBird:                        Credit,
}

/// Data of a single task in the `listOfTasks`.
#[derive( Deserialize,  Serialize )]
pub struct    Task
{
  pub title:                            String,
  pub description:                      String,
  pub registration:                     Registration,
  pub assignment:                       Vec     < Assignment  >,
  pub deadline:                         Deadline,
  pub proof:                            Option  < Proof       >,
}

/// Token to reference and access a `Task` in the `listOfTasks`.
#[derive( Deserialize,  Serialize )]
pub struct    TaskID                    ( pub usize );
