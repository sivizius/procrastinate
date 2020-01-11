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
      Credit,
      Points,
    },
  },
};

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

/// Token to reference and access a `User` in the `listOfUsers`
#[derive( Deserialize,  Serialize )]
pub struct    UserID                    ( pub usize );
