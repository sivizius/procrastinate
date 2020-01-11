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
};

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

/// What you get for achieving this `Task`.
#[derive( Clone, Deserialize,  Serialize )]
pub struct    Credit                    ( pub Points  );

/// Stored points for everything so far.
#[derive( Clone, Deserialize,  Serialize )]
pub struct    Points                    ( pub usize   );
