//! lol
//! # Licence

#![warn(clippy::all)]
#![allow(clippy::suspicious_else_formatting)]

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![warn(missing_docs)]
#![warn(future_incompatible)]

use
{
  hyper::
  {
    Body,
    Method,
    Request,
    Response,
    Server,
    StatusCode,
    service::
    {
      make_service_fn,
      service_fn,
    },
  },
  std::
  {
    convert::
    {
      Infallible,
    },
    net::
    {
      SocketAddr,
    },
  },
  typed_html::
  {
    dom::
    {
      DOMTree,
    },
    html,
  },
};

/// Return successfully an `Infallible`
macro_rules!  infallible
{
  ( $result:expr  ) =>  ( Ok::<_, Infallible> ( $result  ) );
}

/// Handle Hypter Text Transfer Protocol Requests.
///
/// # Arguments
/// * `request`                         – the actual request.
async
fn      handleHTTPrequest
(
  request:                              Request  < Body  >,
)
->  Result
    <
      Response  < Body  >,
      Infallible,
    >
{
  match
  (
    request
      .method(),
    request
      .uri()
      .path()
  )
  {
    ( &Method::GET, "/" )
    =>  {
          Ok
          (
            Response::new
            (
              Body::from
              (
                (
                  html!
                  (
                    <h1>"Hello World!"</h1>
                  ) as DOMTree<String>
                ).to_string()
              )
            )
          )
        },
    _
    =>  {
          let mut error404              =   Response::default();
          *error404.status_mut()        =   StatusCode::NOT_FOUND;
          Ok(error404)
        },
  }
}

/// Entry Point.
#[tokio::main]
async fn main ()
->  Result
    <
      (),
      hyper::error::Error,//Box < dyn std::error::Error + Send + Sync >,
    >
{
  Server::bind
  (
    &SocketAddr::from
    (
      (
        [127, 0, 0, 1],
        8080
      )
    )
  )
    .serve
    (
      make_service_fn
      (
        | _connection |
        async
        {
          infallible!
          (
            service_fn  ( handleHTTPrequest )
          )
        }
      )
    )
    .await
}
