//! lol
//! # Licence

#![warn(clippy::all)]
#![allow(clippy::suspicious_else_formatting)]

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![warn(missing_docs)]
#![warn(future_incompatible)]

#![recursion_limit="1024"]

/// The Web Interface.
mod     web;

use
{
  async_std::
  {
    io::
    {
      Error,
    },
    net::
    {
      SocketAddr,
    },
    sync::
    {
      Arc,
    },
    task::
    {
      block_on,
      spawn,
    },
  },
  futures::
  {
    join,
  },
  procrastinator::
  {
    loadProcrastinatorConfigFromFile,
    Procrastinator,
    this::
    {
      users::
      {
        UserID,
      },
    },
  },
  web::
  {
    handleHTTPrequest,
    WebInterface,
  },
};

/// Constructs a `WebInterface` for `Procrastinator`.
///
/// # Arguments
/// * `procrastinator`                  – instance of `Procrastinator` that will be used by the services.
fn            procrastinatorWebInterface
(
  procrastinator:                       Procrastinator,
)
->  Arc < WebInterface  >
{
  Arc::new
  (
    WebInterface
    (
      procrastinator
        .registerTask
        (
          "Test".to_owned(),
          "Test Task".to_owned(),
          UserID  ( 0 ),
        )
        .registerTask
        (
          "Foo".to_owned(),
          "Foo Bar".to_owned(),
          UserID  ( 0 ),
        ),
    ),
  )
}

/// Starts a `http_async::server` for `Procrastinator`.
///
/// # Arguments
/// * `` –
fn            procrastinatorWebServer
(
  address:                              [ u8; 4 ],
  port:                                 u16,
  webInterface:                         &Arc < WebInterface  >,
)
->  async_std::task::JoinHandle
    <
      Result
      <
        (),
        Error,
      >
    >
{
  //let     handleHTTPrequest             =   &handleHTTPrequest.clone  ( );
  spawn
  (
    {
      http_async::server
      (
        SocketAddr::from
        (
          (
            address,
            port,
          )
        ),
        webInterface.clone(),
        Arc::new  ( &handleHTTPrequest )
      )
    }
  )
}

/// Start all services.
///
/// # Arguments
/// * `procrastinator`                  – instance of `Procrastinator` that will be used by the services.
fn            startAllServices
(
  procrastinator:                       Procrastinator,
)
->  Result
    <
      (),
      Error,
    >
{
  let     webInterface                  =   procrastinatorWebInterface  ( procrastinator  );
  let     webServerAlpha
  =   procrastinatorWebServer
      (
        [ 127,  0,  0,  1 ],
        8080,
        &webInterface
      );
  /*let     webServerBeta
  =   procrastinatorWebServer
      (
        [ 127,  0,  0,  1 ],
        8000,
        &webInterface
      );*/
  let
  (
    alpha,
    beta,
  )
  =   block_on
      (
        async
        {
          join!
          (
            webServerAlpha,
            procrastinatorWebServer
            (
              [ 127,  0,  0,  1 ],
              8000,
              &webInterface
            ),
          )
        }
      );
  alpha?;
  beta?;
  Ok(())
}

/// Entry Point.
fn main ()
->  Result
    <
      (),
      Error,
    >
{
  match loadProcrastinatorConfigFromFile  ( "build/procrastinator.ron"  )
  {
    Ok  ( procrastinator  )             =>  startAllServices  ( procrastinator  ),
    Err ( error )                       =>  Err               ( error           ),
  }
}
