//! Inter Task Communication via Requests and Responses in a Server Client Manner.
//! 1.  Initially a `Server` is started with some Initial Configuration and a Request Handler as a Seperate Task.
//! 2.  Every Task that might communicate to this `Server` `obtains`s a `Client` to this `Server`,
//!       which is just a C2S Sender Channel, where the Task could send a `Request` to.
//! 3.  To communicate, the Task `send`s a `Request` to the C2S Sender Channel,
//!       which returns a `Connection`, just a S2C Receiver Channel,
//!         where the Task could listen to for a `Response`.
//! 4.  The `Server` calls the Handler on every `Request` received on the C2S Receiver`Channel
//!       with the Configuration, this `Request` and a S2C Sender Channel.
//! 5.  The Handler processes the `Request` and returns a `Response` to the Caller,
//!       which in turn sends this `Response` to the requesting Task via the received S2C Sender Channel.
//! ```
//!         Server <============> Client
//!   C2S Receiver <----Port----- C2S Sender
//!   S2C Sender   --Connection-> S2C Receiver
//! ```

use
{
  futures::
  {
    future::
    {
      channel::
      {
        mpsc::
        {
          unbounded,
          UnboundedReceiver,
          UnboundedSender,
        },
      },
    },
  },
};

/// Messages from Client to Server.
mod request;
/// Messages from Server to Client.
mod response;

struct    Unknown                       ();

/// A `Client`, that could communicate with a `Server`.
pub struct    Client                    ( UnboundedSender < String  > );

impl          Client
{
  /// Sends a Request to the `Server` and returns a `Receiver` to listen to `Response`s.
  ///
  /// # Arguments
  /// * `` –
  pub fn        send
  (
    &self,
    request:                            String,
  )
  ->  Result
      <
        UnboundedReceiver < String  >,
        TrySendError,
      >
  {
    self
      .0
      .try_send()
  }
}

/// A `Server` a `Client` could commuicate with.
pub struct    Server
{
  port:                                 UnboundedSender < String  >,
  task:                                 Unknown,
}

/// Constructs and runs a `Server` as a Task.
///
/// # Arguments
/// * `` –
pub fn  Server
(

)
->  Server
{
  let
  (
    receiver,
    sender,
  )                                     =   unbounded ( );
  Server
  {
    port:                               sender,
    task:
      task::spawn
      (
        move async
        {
          while let Ok  ( request )     =   receiver.try_next ( )
          {
            if  let Some  ( request )   =   request
            {
              //  handle request
            }
          }
        }
      )
  }
}

impl          Server
{
  /// Obtains a `Client` of the `Server`.
  ///
  /// # Arguments
  /// * `` –
  pub fn        obtain
  (
    &self,
  )
  ->  Client
  {
    Client
    (
      self
        .port
        .clone  ( )
    )
  }

  /// Wait until `Server` has finished.
  ///
  /// # Arguments
  /// * `` –
  pub fn        wait
  (
    mut self,
  )
  ->  Unknown
  {
    task::block_on
    (
      self
        .task
    )
  }
}
