use
{
  super::
  {
    WebInterface,
  },
  http_async::
  {
    request::
    {
      Request,
    },
  },
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

/// Build Page.
///
/// # Arguments
/// * `request`                         – the actual request,
/// * `interface`                       – interface configuration.
pub fn        frontend                  < Closure >
(
  request:                              &Request,
  interface:                            &WebInterface,
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
            <link rel="stylesheet" href="calendar.css"    />
            <link rel="stylesheet" href="common.css"      />
            <link rel="stylesheet" href="navigation.css"  />
            <meta charset="utf-8" />
          </head>
          <body>
            <div class="frontend">
              <div class="head">
                <a href="/index">
                  <img class="logo" src="logo.png" />
                  " Procrastinator"
                </a>
              </div>
              <div class="menu">
                <h1>"Navigation"</h1>
                <div class="navList">
                {
                  interface
                    .navigation
                    .iter()
                    .map
                    (
                      | entry |
                      html!
                      (
                        <div class="navItemOuter">
                        {
                          unsafe_text!
                          (
                            "<a href=\"{}\"><div class=\"navItemInner\"><img src=\"{}\"/> {}</div></a>",
                            {
                              entry.link
                            },
                            {
                              entry.logo
                            },
                            {
                              entry.name
                            }
                          )
                        }
                        </div>
                      )
                    )
                }
                </div>
              </div>
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
