use
{
  super::
  {
    WebInterface,
    frontend::
    {
      frontend,
    },
  },
  http_async::
  {
    request::
    {
      Request,
    },
    status::
    {
      Status,
    },
  },
  typed_html::
  {
    html,
    dom::
    {
      DOMTree,
    },
  },
};

/// …
///
/// # Arguments
/// * `request`                         – the actual request,
/// * `interface`                       – interface configuration.
pub fn        page404
(
  request:                              &Request,
  interface:                            &WebInterface,
)
->  (
      Status,
      &'static str,
      Vec < u8  >
    )
{
  (
    Status::NotFound,
    "text/html",
    frontend
    (
      &request,
      &interface,
      "Not Found *shrug*".to_owned(),
      | _ |
      {
        let     document:               DOMTree  < String  >
        =   html!
            (
              <h1>"404 – Page Not Found!"</h1>
            );
        document.to_string()
      },
    ),
  )
}
