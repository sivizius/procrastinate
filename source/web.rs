use
{
  crate::
  {
    http::
    {
      request::Request,
      response::Response,
      status::Status,
      version::Version,
    },
  },
  async_std::
  {
    net::
    {
      TcpListener,
      ToSocketAddrs,
    },
    prelude::*,
    task,
  },
  std::
  {
    io::
    {
      Error,
    },
  },
  typed_html::
  {
    html,
    text,
    dom::
    {
      DOMTree,
    },
  },
};

/// Handle Hypter Text Transfer Protocol Requests.
///
/// # Arguments
/// * `request`                         – the actual request.
async fn      handleHTTPrequest
(
  request:                              Request,
)
->  Result
    <
      Response,
      String,
    >
{
  let mut content: DOMTree  < String  >
  =   html!
      (
        <html>
          <head>
            <title>"Procrastinate"</title>
          </head>
          <body>
            <h1>"Hello World!"</h1>
            <p>
            {
              text!
              (
                "Path: {}",
                request.path
              )
            }
            </p>
            <ul>
            {
              request
                .query
                .iter()
                .map
                (
                  | entry |
                  html!
                  (
                    <li>
                    {
                      text!
                      (
                        "{}={}",
                        entry.key,
                        entry.value
                      )
                    }
                    </li>
                  )
                )
            }
            </ul>
          </body>
        </html>
      );
  Ok
  (
    Response()
      .version    ( Version::HTTP_10      )
      .status     ( Status::Ok            )
      .addHeader
      (
        "Content-Type".to_owned(),
        "text/html".to_owned(),
      )
      .content    ( content.to_string ( ) )
  )
}

/// …
///
/// # Arguments
/// * `` –
pub fn          webServer
<
  Address,
>
(
  address:                              Address,
)
->  Result
    <
      (),
      Error,
    >
where
  Address:                              ToSocketAddrs,
{
  task::block_on
  (
    async
    {
      let     socket                    =   TcpListener::bind ( address ).await?;
      println!("Waiting for connections");
      loop
      {
        let
        (
          mut stream,
          address
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
                    &mut stream,
                  ).await
            {
              Ok  ( request )
              =>  match handleHTTPrequest ( request ).await
                  {
                    Ok  ( response  )
                    =>  match
                          match {
                                  let     response
                                  =   format!
                                      (
                                        "{}",
                                        response,
                                      );
                                  //println!  ( "{}", response  );
                                  stream
                                  .write  ( response.as_bytes ( ) )
                                  .await
                                }
                          {
                            Ok    ( _     )
                            =>  stream
                                  .flush().await,
                            Err   ( error )
                            =>  Err ( error ),
                          }
                        {
                          Ok  ( _       ) =>  println!  ( "Success! {}",    address ),
                          Err ( message ) =>  eprintln! ( "Send Fail: {}",  message ),
                        },
                    Err ( message   )
                    =>  eprintln! ( "Ouput Fail: {}", message ),
                  },
              Err ( message )
              =>  eprintln! ( "Input Fail: {}", message ),
            }
          }
        );
      }
    }
  )
}
