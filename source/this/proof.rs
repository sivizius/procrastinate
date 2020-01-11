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
