mod calendar;
mod frontend;
mod index;
mod page404;

use
{
  self::
  {
    calendar::*,
    index::*,
    page404::*,
  },
  http_async::
  {
    request::
    {
      Request,
    },
    response::
    {
      Response,
    },
    status::
    {
      Status,
    },
  },
  async_std::
  {
    sync::
    {
      Arc,
    },
  },
  procrastinator::
  {
    Procrastinator,
  },
};

/// Entry in the Navigation Menu.
struct        NavEntry
{
  name:                                 &'static str,
  link:                                 &'static str,
  logo:                                 &'static str,
}

/// Constructor for `NavEntry`.
///
/// # Arguments
/// * `name`                            – displayed name of entry,
/// * `link`                            – location of entry.
fn            NavEntry
(
  name:                                 &'static str,
  link:                                 &'static str,
  logo:                                 &'static str,
)
->  NavEntry
{
  NavEntry
  {
    name,
    link,
    logo,
  }
}

/// Configuration of the Web Interface.
pub struct    WebInterface
{
  procrastinator:                       Procrastinator,
  navigation:                           Vec < NavEntry  >,
}

/// Constructor for `WebInterface` …
///
/// # Arguments
/// * `` –
pub fn  WebInterface
(
  procrastinator:                       Procrastinator,
)
->  WebInterface
{
  WebInterface
  {
    procrastinator,
    navigation:
      vec!
      (
        NavEntry  ( "Profile",  "myself",   "logo.png"  ),
        NavEntry  ( "Tasks",    "tasks",    "logo.png"  ),
        NavEntry  ( "Calendar", "calendar", "logo.png"  ),
      ),
  }
}

impl          WebInterface
{

}

/// Just send this asset successfully.
macro_rules!  includeAsset
{
  (
    $Type:expr,
    $Path:expr
  )
  =>  {
        (
          Status::Ok,
          $Type,
          include_bytes!  ( $Path )
            .to_vec(),
        )
      };
}

/// Handle Hypter Text Transfer Protocol Requests.
///
/// # Arguments
/// * `request`                         – the actual request.
pub fn        handleHTTPrequest
(
  request:                              Request,
  interface:                            Arc < WebInterface  >,
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
        =>  includeAsset! ( "image/png",  "assets/sba.png"        ),
        | "/calendar.css"
        =>  includeAsset! ( "text/css",   "assets/calendar.css"   ),
        | "/common.css"
        =>  includeAsset! ( "text/css",   "assets/common.css"     ),
        | "/navigation.css"
        =>  includeAsset! ( "text/css",   "assets/navigation.css" ),
        | "/"
        | "/index"
        =>  index
            (
              &request,
              &interface,
            ),
        | "/calendar"
        =>  calendar
            (
              &request,
              &interface,
            ),
        _
        =>  page404
            (
              &request,
              &interface,
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
