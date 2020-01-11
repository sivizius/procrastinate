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
    task,
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
    WebInterface,
  },
};

/// Start all services.
///
/// # Arguments
/// * `` –
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
  let     webInterface
  =   Arc::new
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
      );
  let     webServerAlpha
  =   task::spawn
      (
        {
          http_async::server
          (
            SocketAddr::from
            (
              (
                [ 127,  0,  0,  1 ],
                8080,
              )
            ),
            webInterface.clone(),
            Arc::new  ( &web::handleHTTPrequest  )
          )
        }
      );
  let     webServerBeta
  =   task::spawn
      (
        {
          http_async::server
          (
            SocketAddr::from
            (
              (
                [ 127,  0,  0,  1 ],
                8000,
              )
            ),
            webInterface,
            Arc::new  ( &web::handleHTTPrequest  )
          )
        }
      );
  println!
  (
    "
      Services:
        Alpha:  {}
        Beta:   {}
    ",
    webServerAlpha.task().id(),
    webServerBeta.task().id(),
  );
  task::block_on(webServerAlpha)?;
  task::block_on(webServerBeta)?;
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
    Ok  ( procrastinator  )
    =>  startAllServices
        (
          procrastinator,
        ),
    Err ( error )
    =>  Err ( error ),
  }
}
