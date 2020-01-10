use
{
  crate::
  {
    http::
    {
      request::Request,
      response::Response,
      status::Status,
    },
  },
  async_std::
  {
    sync::
    {
      Arc,
    },
  },
  indoc::
  {
    indoc,
  },
  procrastinator::
  {
    Procrastinator,
  },
  /*serde_json::
  {

  },*/
  typed_html::
  {
    html,
    text,
    unsafe_text,
    dom::
    {
      DOMTree,
    },
  },
};

/// …
///
/// # Arguments
/// * `` –
fn            frontend                  < Closure >
(
  request:                              &Request,
  title:                                String,
  body:                                 Closure,
)
->  Vec < u8  >
where
  Closure:                              FnOnce  ( &Request ) ->  String,
{
  let     document:                     DOMTree  < String  >
  =   html!
      (
        <html>
          <head>
            <title>
            {
              text!
              (
                "Procrastinator – {}",
                title
              )
            }
            </title>
            <link rel="stylesheet" href="common.css" />
            <meta charset="utf-8" />
          </head>
          <body>
            <div class="frontend">
              <div class="head">
                <img class="logo" src="logo.png" />
                "Procrastinator"
              </div>
              <div class="menu">"Navigation"</div>
              <div class="body">
              {
                unsafe_text!
                (
                  body(&request)
                )
              }
              </div>
              <div class="foot">
                "Project by _sivizius – "
                <a href="https://github.com/sivizius/procrastinator">"Procrastinator on GitHub"</a>
                " – 2020"
              </div>
            </div>
          </body>
        </html>
      );
  document
    .to_string        ( )
    .into_bytes       ( )
}

/// Handle Hypter Text Transfer Protocol Requests.
///
/// # Arguments
/// * `request`                         – the actual request.
pub fn        handleHTTPrequest
(
  request:                              Request,
  _procrastinator:                       Arc < Procrastinator  >,
)
->  Response
{
  let
  (
    contentCode,
    contentType,
    contentBody,
  )
  =   match request
              .path
              .as_str()
      {
        | "/favicon.ico"
        | "/logo.png"
        =>  (
              Status::Ok,
              "image/png",
              include_bytes!  ( "assets/sba.png" )
                .to_vec(),
            ),
        | "/common.css"
        =>  (
              Status::Ok,
              "text/css",
              indoc!
              ("
                body
                {
                  background:           #000;
                  color:                #f80;
                }
                .logo
                {
                  height:               48px;
                }
                .head
                {
                  grid-area:            head;
                  font-size:            48px;
                }
                .menu
                {
                  grid-area:            menu;
                }
                .body
                {
                  grid-area:            body;
                }
                .foot
                {
                  grid-area:            foot;
                  align:                center;
                }
                .frontend
                {
                  display:              grid;
                  grid-template-areas:
                    'head head head head head head'
                    'menu body body body body body'
                    'foot foot foot foot foot foot';
                  grid-gap:             2px;
                  grid-template-rows:   auto 1fr auto;
                  height:               100%;
                  background:           #f80;
                }
                .frontend > div
                {
                  background:           #000;
                  padding:              2px 0;
                }
              ")
                .as_bytes ( )
                .to_vec   ( ),
            ),
        | "/"
        | "/index"
        =>  (
              Status::Ok,
              "text/html",
              frontend
              (
                &request,
                "Hello World!".to_owned(),
                | _ |
                {
                  let     document:     DOMTree  < String  >
                  =   html!
                      (
                        <h1>"Hello World"</h1>
                      );
                  document.to_string()
                },
              ),
            ),
        _
        =>  (
              Status::NotFound,
              "text/html",
              frontend
              (
                &request,
                "Not Found *shrug*".to_owned(),
                | _ |
                {
                  let     document:     DOMTree  < String  >
                  =   html!
                      (
                        <h1>"404 – Not Found"</h1>
                      );
                  document.to_string()
                },
              ),
            ),
      };
  Response()
    .version    ( request.version )
    .status     ( contentCode     )
    .content
    (
      contentType,
      contentBody,
    )
}
