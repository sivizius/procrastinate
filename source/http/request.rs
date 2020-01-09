use
{
  super::
  {
    KeyValuePair,
    method::Method,
    version::Version,
  },
  async_std::
  {
    net::
    {
      TcpStream,
    },
    prelude::*,
  },
};

/// …
enum          PathState
{
  Start,
  Key,
  Value,
}

/// …
struct        Path
{
  pub path:                             String,
  pub query:                            Vec < KeyValuePair  >,
}

/// …
pub struct    Request
{
  pub method:                           Method,
  pub path:                             String,
  pub querySeperator:                   char,
  pub query:                            Vec < KeyValuePair  >,
  pub version:                          Version,
  pub header:                           Vec < KeyValuePair  >,
  pub body:                             String,
}

/// Constructor for `Request` …
pub fn  Request ()
->  Request
{
  Request
  {
    method:                               Method::None,
    path:                                 "".to_owned(),
    querySeperator:                       '&',
    query:                                Vec::new(),
    version:                              Version::HTTP_10,
    header:                               Vec::new(),
    body:                                 "".to_owned(),
  }
}

impl          Request
{
  /// Try to parse Hyper Text Transfer Protocol Request Method from Transmission Control Protocol Stream.
  ///
  /// # Arguments
  /// * `stream`                        – Transmission Control Protocol Stream.
  async fn      parseMethod
  (
    mut stream:                         &mut TcpStream,
  )
  ->  Option  < Method  >
  {
    match Self::getChar ( &mut stream ).await
    {
      Some  ( 'G' )
      =>  match Self::getChar ( &mut stream ).await
          {
            Some  ( 'E' )
            =>  match Self::getChar ( &mut stream ).await
                {
                  Some  ( 'T' )
                  =>  match Self::getChar ( &mut stream ).await
                      {
                        Some  ( ' ' )
                        =>  Some  ( Method::Get ),
                        _
                        =>  None,
                      },
                  _
                  =>  None,
                },
            _
            =>  None,
          },
      Some  ( 'P' )
      =>  match Self::getChar ( &mut stream ).await
          {
            Some  ( 'O' )
            =>  match Self::getChar ( &mut stream ).await
                {
                  Some  ( 'S' )
                  =>  match Self::getChar ( &mut stream ).await
                      {
                        Some  ( 'T' )
                        =>  match Self::getChar ( &mut stream ).await
                            {
                              Some  ( ' ' )
                              =>  Some  ( Method::Post ),
                              _
                              =>  None,
                            },
                        _
                        =>  None,
                      },
                  _
                  =>  None,
                },
            _
            =>  None,
          },
      _
      =>  None,
    }
  }

  /// Try to parse Hyper Text Transfer Protocol Version from Transmission Control Protocol Stream.
  ///
  /// # Arguments
  /// * `stream`                        – Transmission Control Protocol Stream.
  async fn      parseVersion
  (
    mut stream:                         &mut TcpStream,
  )
  ->  Option  < Version >
  {
    match match Self::getChar ( &mut stream ).await
          {
            Some  ( 'H' )
            =>  match Self::getChar ( &mut stream ).await
                {
                  Some  ( 'T' )
                  =>  match Self::getChar ( &mut stream ).await
                      {
                        Some  ( 'T' )
                        =>  match Self::getChar ( &mut stream ).await
                            {
                              Some  ( 'P' )
                              =>  match Self::getChar ( &mut stream ).await
                                  {
                                    Some  ( '/' )
                                    =>  match Self::getChar ( &mut stream ).await
                                        {
                                          Some  ( '0' )
                                          =>  match Self::getChar ( &mut stream ).await
                                              {
                                                Some  ( '.' )
                                                =>  match Self::getChar ( &mut stream ).await
                                                    {
                                                      Some  ( '9' )
                                                      =>  Some  ( Version::HTTP_09  ),
                                                      _
                                                      =>  None,
                                                    },
                                                _
                                                =>  None,
                                              },
                                          Some  ( '1' )
                                          =>  match Self::getChar ( &mut stream ).await
                                              {
                                                Some  ( '.' )
                                                =>  match Self::getChar ( &mut stream ).await
                                                    {
                                                      Some  ( '0' )
                                                      =>  Some  ( Version::HTTP_10  ),
                                                      Some  ( '1' )
                                                      =>  Some  ( Version::HTTP_11  ),
                                                      _
                                                      =>  None,
                                                    },
                                                _
                                                =>  None,
                                              },
                                          Some  ( '2' )
                                          =>  match Self::getChar ( &mut stream ).await
                                              {
                                                Some  ( '.' )
                                                =>  match Self::getChar ( &mut stream ).await
                                                    {
                                                      Some  ( '0' )
                                                      =>  Some  ( Version::HTTP_2  ),
                                                      _
                                                      =>  None,
                                                    },
                                                _
                                                =>  None,
                                              },
                                          Some  ( '3' )
                                          =>  match Self::getChar ( &mut stream ).await
                                              {
                                                Some  ( '.' )
                                                =>  match Self::getChar ( &mut stream ).await
                                                    {
                                                      Some  ( '0' )
                                                      =>  Some  ( Version::HTTP_3  ),
                                                      _
                                                      =>  None,
                                                    },
                                                _
                                                =>  None,
                                              },
                                          _
                                          =>  None,
                                        },
                                    _
                                    =>  None,
                                  },
                              _
                              =>  None,
                            },
                        _
                        =>  None,
                      },
                  _
                  =>  None,
                },
            _
            =>  None,
          }
    {
      Some  ( version )
      =>  match Self::getChar ( &mut stream ).await
          {
            Some  ( '\r'  )
            =>  match Self::getChar ( &mut stream ).await
                {
                  Some  ( '\n'  )
                  =>  Some  ( version ),
                  _
                  =>  None,
                },
            _
            =>  None,
          },
      _
      =>  None,
    }
  }

  /// Try to parse path from from Transmission Control Protocol Stream.
  ///
  /// # Arguments
  /// * `stream`                        – Transmission Control Protocol Stream.
  async fn      parsePath
  (
    mut stream:                         &mut TcpStream,
    querySeperator:                     char,
  )
  ->  Option  < Path  >
  {
    let mut result                      =   None;
    let mut path                        =   "".to_owned ( );
    let mut key                         =   "".to_owned ( );
    let mut value                       =   "".to_owned ( );
    let mut query                       =   Vec::new    ( );
    let mut state                       =   PathState::Start;
    while let Some  ( char  )           =   Self::getChar ( &mut stream ).await
    {
      match char
      {
        ' '
        =>  {
              match state
              {
                | PathState::Value
                | PathState::Key
                =>  if !key.is_empty()
                    {
                      query
                        .push
                        (
                          KeyValuePair
                          {
                            key:        key.clone(),
                            value:      value.clone(),
                          }
                        );
                    },
                PathState::Start
                =>  {},
              }
              result
              =   Some
                  (
                    Path
                    {
                      path,
                      query,
                    }
                  );
              break;
            },
        '?'
        =>  match state
            {
              PathState::Start          =>  state = PathState::Key,
              PathState::Key            =>  key.push      ( char  as  char  ),
              PathState::Value          =>  value.push    ( char  as  char  ),
            },
        '='
        =>  match state
            {
              PathState::Start          =>  path.push     ( char  as  char  ),
              PathState::Key            =>  state = PathState::Value,
              PathState::Value          =>  value.push    ( char  as  char  ),
            },
        '\r' | '\n'
        =>  break,
        _
        =>  if  char  ==  querySeperator
            {
              match state
              {
                PathState::Start        =>  path.push     ( char  as  char  ),
                | PathState::Key
                | PathState::Value
                =>  {
                      state             =   PathState::Key;
                      if !key.is_empty()
                      {
                        query
                          .push
                          (
                            KeyValuePair
                            {
                              key:      key.clone(),
                              value:    value.clone(),
                            }
                          );
                      }
                      key               =   "".to_owned ( );
                      value             =   "".to_owned ( );
                    },
              }
            }
            else
            {
              match state
              {
                PathState::Start        =>  path.push     ( char  as  char  ),
                PathState::Key          =>  key.push      ( char  as  char  ),
                PathState::Value        =>  value.push    ( char  as  char  ),
              }
            },
      }
    }
    result
  }

  /// Try to read a single `char` from Transmission Control Protocol stream.
  ///
  /// # Arguments
  /// * `stream`                        – Transmission Control Protocol Stream.
  pub async fn  getChar
  (
    stream:                             &mut TcpStream,
  )
  ->  Option  < char  >
  {
    let mut buffer                      =   vec!  [ 0u8;  1 ];
    if  let Ok  ( length  )             =   stream.read  ( &mut buffer ).await
    {
      if  length  ==  1
      {
        Some  ( buffer  [ 0 ] as  char  )
      }
      else
      {
        None
      }
    }
    else
    {
      None
    }
  }

  /// Parse `TcpStream` as Hyper Text Transfer Protocol Request.
  ///
  /// # Arguments
  /// * `stream`                          – Transmission Control Protocol Stream.
  pub async fn  parse
  (
    mut self,
    mut stream:                           &mut TcpStream,
  )
  ->  Result
      <
        Self,
        String,
      >
  {
    if  let Some  ( method  )             =   Self::parseMethod   ( &mut stream                       ).await
    {
      self.method                         =   method;
      if let Some ( path )                =   Self::parsePath     ( &mut stream,  self.querySeperator ).await
      {
        println!
        (
          "{}?({:?})",
          path.path,
          path.query,
        );
        self.path                         =   path.path;
        self.query                        =   path.query;
        if let Some ( version )           =   Self::parseVersion  ( &mut stream                       ).await
        {
          self.version                    =   version;
          Ok  ( self  )
        }
        else
        {
          Err ( "Could not parse version".to_owned  ( ) )
        }
      }
      else
      {
        Err   ( "Could not parse path".to_owned     ( ) )
      }
    }
    else
    {
      Err     ( "Could not parse method.".to_owned  ( ) )
    }
  }
}
