pub mod method;
pub mod request;
pub mod response;
pub mod status;
pub mod version;

use
{
  crate::
  {
    http::
    {
      request::Request,
      response::Response,
    },
  },
  async_std::
  {
    io::
    {
      Error,
    },
    net::
    {
      TcpListener,
      ToSocketAddrs,
    },
    prelude::*,
    sync::
    {
      Arc,
    },
    task,
  },
  std::
  {
    fmt::
    {
      Display,
    },
  },
};

/// …
#[derive(Debug)]
pub struct    KeyValuePair
{
  pub key:                              String,
  pub value:                            String,
}

/// …
///
/// # Arguments
/// * `` –
pub async fn  server
<
  Address,
  Data,
  Handler,
>
(
  address:                              Address,
  this:                                 Arc < Data    >,
  handler:                              Arc < Handler >,
)
->  Result
    <
      (),
      Error,
    >
where
  Address:                              ToSocketAddrs + Display + Send + Sync + Clone + 'static,
  Data:                                 Send + Sync + 'static,
  Handler:                              Send + Sync + 'static +
    Fn
    (
      Request,
      Arc < Data  >,
    )
    ->  Response,
{
  let     socket
  =   TcpListener::bind ( &address   )
        .await
        .expect ( "Failed to bind"  );
  println!("Waiting for connections");
  loop
  {
    let     this                        =   this.clone    ( );
    let     handler                     =   handler.clone ( );
    let     address                     =   address.clone ( );
    let
    (
      mut tcpStream,
      client
    )
    =   socket
          .accept()
          .await
          .unwrap();
    task::spawn
    (
      async move
      {
        match Request().parse
              (
                &mut tcpStream,
              ).await
        {
          Ok  ( request )
          =>  match match tcpStream
                            .write
                            (
                              handler
                              (
                                request,
                                this,
                              )
                                .into_vector  ( )
                                .as_slice     ( )
                            )
                            .await
                    {
                      Ok    ( _     )
                      =>  tcpStream
                            .flush().await,
                      Err   ( error )
                      =>  Err ( error ),
                    }
              {
                Ok  ( _       )
                =>  println!
                    (
                      "Success! {} -> {}",
                      address,
                      client,
                    ),
                Err ( error )
                =>  eprintln!
                    (
                      "Send Fail: {}",
                      error,
                    ),
              },
          Err ( message )
          =>  eprintln! ( "Input Fail: {}", message ),
        }
      }
    );
  }
}
