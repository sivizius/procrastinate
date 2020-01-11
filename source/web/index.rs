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
    unsafe_text,
    dom::
    {
      DOMTree,
    },
  },
};

/// Build Index Page.
///
/// # Arguments
/// * `request`                         – the actual request,
/// * `interface`                       – interface configuration.
pub fn        index
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
    Status::Ok,
    "text/html",
    frontend
    (
      &request,
      &interface,
      "Hello World!".to_owned(),
      | _ |
      {
        let     tasks
        =   interface
              .procrastinator
              .listOfTasks
              .iter()
              .filter_map
              (
                | entry |
                if  let Some  ( entry )
                    =   entry
                {
                  Some
                  (
                    format!
                    (
                      "<div class=\"taskItem\">{}</div>",
                      entry
                        .title
                        .to_owned()
                    )
                  )
                }
                else
                {
                  None
                }
              )
              .fold
              (
                "".to_owned(),
                |
                  mut list,
                  task,
                |
                {
                  list
                    .push_str ( &task );
                  list
                }
              );
        let     document:     DOMTree  < String  >
        =   html!
            (
              <div>
              {
                unsafe_text!
                (
                  "{}",
                  {
                    tasks
                  }
                )
              }
              </div>
            );
        document.to_string()
      },
    ),
  )
}
