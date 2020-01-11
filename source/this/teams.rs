use
{
  serde::
  {
    Deserialize,
    Serialize,
  },
  super::
  {
    credit::
    {
      Achievement,
      Points,
    },
  },
};

/// Token to reference and access a `Team` in the `listOfTeams`
#[derive( Deserialize,  Serialize )]
pub struct    TeamID                    ( pub usize );

/// Data about a single team stored in the `listOfTeams`.
#[derive( Deserialize,  Serialize )]
pub struct    Team
{
  name:                                 String,
  points:                               Points,
  achievements:                         Vec < Achievement >,
}
