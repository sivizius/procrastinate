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
      Registration,
    },
  },
};

/// One single Appointment.
#[derive( Deserialize,  Serialize )]
pub struct    Appointment
{
  title:                                String,
  description:                          String,
  start:                                SystemTime,
  stop:                                 SystemTime,
  registration:                         Registration,
}
