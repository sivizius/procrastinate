//! lol
//! # Licence

#![warn(clippy::all)]
#![allow(clippy::suspicious_else_formatting)]

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![warn(missing_docs)]
#![warn(future_incompatible)]

#![recursion_limit="256"]

mod     http;
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
  },
};

/// Start all services.
///
/// # Arguments
/// * `` –
fn            startAllServices
(
  procrastinator:                       Arc < Procrastinator  >,
)
->  Result
    <
      (),
      Error,
    >
{
  let     webServerAlpha
  =   task::spawn
      (
        {
          http::server
          (
            SocketAddr::from
            (
              (
                [ 127,  0,  0,  1 ],
                8080,
              )
            ),
            procrastinator.clone(),
            Arc::new  ( &web::handleHTTPrequest  )
          )
        }
      );
  let     webServerBeta
  =   task::spawn
      (
        {
          http::server
          (
            SocketAddr::from
            (
              (
                [ 127,  0,  0,  1 ],
                8000,
              )
            ),
            procrastinator,
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
          Arc::new
          (
            procrastinator,
          ),
        ),
    Err ( error )
    =>  Err ( error ),
  }
}
