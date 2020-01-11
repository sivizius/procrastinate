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
    teams::
    {
      TeamID,
    },
    users::
    {
      UserID,
    },
  },
};

/// Data about registration of `Task`.
#[derive( Deserialize,  Serialize )]
pub struct    Registration
{
  pub byUser:                           UserID,
  pub time:                             SystemTime,
}

/// Data about assignment of `Task`.
#[derive( Deserialize,  Serialize )]
pub struct    Assignment
{
  pub byUser:                           UserID,
  pub toTeam:                           TeamID,
  pub time:                             SystemTime,
}
