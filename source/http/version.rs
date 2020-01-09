use
{
  std::
  {
    fmt::
    {
      self,
      Display,
    },
  },
};

/// …
#[allow(non_camel_case_types)]
pub enum      Version
{
  HTTP_09,
  HTTP_10,
  HTTP_11,
  HTTP_2,
  HTTP_3,
}

impl          Display                   for Version
{
  fn fmt
  (
    &self,
    formatter:                          &mut fmt::Formatter<'_>
  )
  ->  fmt::Result
  {
    formatter
      .write_str
      (
        &format!
        (
          "HTTP/{}",
          match self
          {
            Version::HTTP_09            =>  "0.9",
            Version::HTTP_10            =>  "1.0",
            Version::HTTP_11            =>  "1.1",
            Version::HTTP_2             =>  "2.0",
            Version::HTTP_3             =>  "3.0",
          }
        )
      )
  }
}
