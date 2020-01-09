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
  std::
  {
    io::
    {
      Error,
    },
    net::
    {
      SocketAddr,
    },
  },
  web::
  {
    webServer,
  },
};

/// Entry Point.
fn main ()
->  Result
    <
      (),
      Error,
    >
{
  webServer
  (
    SocketAddr::from
    (
      (
        [ 127,  0,  0,  1 ],
        8080,
      )
    ),
  )
}
