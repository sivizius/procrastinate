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
  /*serde_json::
  {

  },*/
  typed_html::
  {
    html,
    dom::
    {
      DOMTree,
    },
  },
};

/// Build Calendar Page.
///
/// # Arguments
/// * `request`                         – the actual request,
/// * `interface`                       – interface configuration.
pub fn        calendar
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
      "Calendar".to_owned(),
      | _ |
      {
        let     document:     DOMTree  < String  >
        =   html!
            (
              <div class="calendar">
                <div class="calPrev">"←"</div>
                <div class="calThis">"Calendar"</div>
                <div class="calNext">"→"</div>
                <div class="calTMon calTitle">"Monday"</div>
                <div class="calTTue calTitle">"Tuesday"</div>
                <div class="calTWed calTitle">"Wednesday"</div>
                <div class="calTThu calTitle">"Thursday"</div>
                <div class="calTFri calTitle">"Friday"</div>
                <div class="calTSat calTitle">"Saturday"</div>
                <div class="calTSun calTitle">"Sunday"</div>
                <div class="cal0Mon">""</div>
                <div class="cal0Tue"></div>
                <div class="cal0Wed"></div>
                <div class="cal0Thu"></div>
                <div class="cal0Fri"></div>
                <div class="cal0Sat"></div>
                <div class="cal0Sun"></div>
                <div class="cal1Mon"></div>
                <div class="cal1Tue"></div>
                <div class="cal1Wed"></div>
                <div class="cal1Thu"></div>
                <div class="cal1Fri"></div>
                <div class="cal1Sat"></div>
                <div class="cal1Sun"></div>
                <div class="cal2Mon"></div>
                <div class="cal2Tue"></div>
                <div class="cal2Wed"></div>
                <div class="cal2Thu"></div>
                <div class="cal2Fri"></div>
                <div class="cal2Sat"></div>
                <div class="cal2Sun"></div>
                <div class="cal3Mon"></div>
                <div class="cal3Tue"></div>
                <div class="cal3Wed"></div>
                <div class="cal3Thu"></div>
                <div class="cal3Fri"></div>
                <div class="cal3Sat"></div>
                <div class="cal3Sun"></div>
                <div class="cal4Mon"></div>
                <div class="cal4Tue"></div>
                <div class="cal4Wed"></div>
                <div class="cal4Thu"></div>
                <div class="cal4Fri"></div>
                <div class="cal4Sat"></div>
                <div class="cal4Sun"></div>
                <div class="cal5Mon"></div>
                <div class="cal5Tue"></div>
                <div class="cal5Wed"></div>
                <div class="cal5Thu"></div>
                <div class="cal5Fri"></div>
                <div class="cal5Sat"></div>
                <div class="cal5Sun"></div>
              </div>
            );
        document.to_string()
      },
    ),
  )
}
