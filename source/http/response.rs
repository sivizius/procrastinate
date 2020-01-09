use
{
  super::
  {
    KeyValuePair,
    status::Status,
    version::Version,
  },
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
pub struct    Response
{
  pub version:                          Version,
  pub status:                           Status,
  pub header:                           Vec < KeyValuePair  >,
  pub content:                          String,
}

/// Constructor for `Response` …
///
/// # Arguments
/// * `` –
pub fn  Response
(

)
->  Response
{
  Response
  {
    version:                            Version::HTTP_10,
    status:                             Status::Ok,
    header:                             Vec::new(),
    content:                            "".to_owned(),
  }
}

impl          Response
{
  /// Add a `header`-value to `Response` in an OOP-Style.
  ///
  /// # Arguments
  /// * `key`                           – key of new header entry,
  /// * `value`                         – value of new header entry.
  pub fn        addHeader
  (
    mut self,
    key:                                String,
    value:                              String,
  )
  ->  Self
  {
    self
      .header
      .push
      (
        KeyValuePair
        {
          key,
          value,
        }
      );
    self
  }

  /// Set `version` of `Response` in an OOP-Style.
  ///
  /// # Arguments
  /// * `version`                       – hyper text protocol version.
  pub fn        version
  (
    mut self,
    version:                            Version,
  )
  ->  Self
  {
    self.version                        =   version;
    self
  }

  /// Set `status` of `Response` in an OOP-Style.
  ///
  /// # Arguments
  /// * `status`                       – hyper text protocol respone status code.
  pub fn        status
  (
    mut self,
    status:                             Status,
  )
  ->  Self
  {
    self.status                         =   status;
    self
  }

  /// Set `content` of `Response` in an OOP-Style.
  ///
  /// # Arguments
  /// * `content`                       – hyper text protocol respone.
  pub fn        content
  (
    mut self,
    content:                            String,
  )
  ->  Self
  {
    let     length                      =   &content.len  ( );
    self.content                        =   content;
    self
      .addHeader
      (
        "Content-Length".to_owned(),
        format!
        (
          "{}",
          length,
        ),
      )
  }
}

impl          Display                   for Response
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
          "{} {}\r\n{}\r\n{}",
          self.version,
          self.status,
          {
            let mut result                      =   "".to_owned ( );
            for pair                            in  &self.header
            {
              result
                .push_str
                (
                  &format!
                  (
                    "{}: {}\r\n",
                    pair.key,
                    pair.value,
                  )
                );
            }
            result
          },
          self.content,
        )
      )
  }
}
