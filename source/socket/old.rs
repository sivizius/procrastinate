use
{
  async_std::
  {
    sync::
    {
      Arc,
    },
  },
  futures::
  {
    future::
    {
      channel::
      {
        mpsc,
        oneshot,
      },
    },
  },
};

/// Request from `Client` to `Server`.
pub struct    Requests
<
  RequestData,
  ResponseData,
>
{
  port:                                 oneshot::Sender < ResponseData  >,
  data:                                 RequestData,
}

/// Response from `Server` to `Client`.
pub struct    Response                  < ResponseData  > ( ResponseData  );

///
pub struct    Server
<
  RequestData,
  ResponseData,
>
{
  receiver:                             mpsc::Receiver  < RequestData   >,
  sender:                               mpsc::Sender    < ResponseData  >,
  task:                                 (),
}

/// Constructor for a `Server`.
pub fn        Server
(
  this:                                 ServerData,
  handler:                              Handler,
)
<
  Handler,
  RequestData,
  ResponseData,
  ServerData,
>
->  Server
where
  Handler:                              Send + Sync + 'static +
    Fn
    (
      Request < RequestData >,
      Arc < ServerData  >,
    )
    ->  Response,
{
  let
  (
    receiver:                           mpsc::Receiver  < Request < RequestData > >,
    sender:                             mpsc::Sender    < Request < RequestData > >,
  )                                     =   mpsc::channel ( );
  Server
  {
    receiver,
    sender,
    task:
      task::spawn
      (
        async
        {

        }
      )
  }
}

impl          Server
{
  /// …
  ///
  /// # Arguments
  /// * `` –
  pub fn        block_on
  (
    mut self,
  )
  ->  TypeName
  {
    task::block_on  ( self.task )?
  }

  /// Connect to a `Server` and returns a channel to send data to.
  pub fn        connect
  <
    RequestData,
  >
  (
    &self,
  )
  ->  mpsc::Sender    < Request < RequestData > >,
  {
    self
      .sender
      .clone()
  }
}
