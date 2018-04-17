use slog;
use serde_json;

use websocket::OwnedMessage;
use websocket::sender::Writer;
use websocket::stream::sync::TcpStream;

use error::{JoinError, MessageError};


pub struct Sender
{
    logger: slog::Logger,
    writer: Writer<TcpStream>,
    join_ref: u32,
    message_ref: u32,
}

impl Sender {
    pub fn new(writer: Writer<TcpStream>, logger: slog::Logger) -> Sender {
        Sender {
            logger: logger,
            writer: writer,
            join_ref: 0,
            message_ref: 0,
        }
    }

    pub fn join(&mut self, channel: &str) -> Result<u32, JoinError> {
        let phx_message = json!([self.join_ref, self.message_ref, channel, "phx_join", {}]);

        self.join_ref += 1;
        self.message_ref += 1;

        // serialise the message and use it to join the channel
        let serialised = serde_json::to_string(&phx_message)?;
        debug!(self.logger, "join()"; "payload" => &serialised);
        let message = OwnedMessage::Text(serialised);

        self.writer.send_message(&message)?;
        return Ok(self.join_ref);
    }

    pub fn heartbeat(&mut self) -> Result<(u32), MessageError> {
        let phx_message = json!([(), self.message_ref, "phoenix", "heartbeat", {}]);

        self.message_ref += 1;

        let serialised = serde_json::to_string(&phx_message)?;
        debug!(self.logger, "heartbeat()"; "payload" => &serialised);
        let message = OwnedMessage::Text(serialised);

        self.writer.send_message(&message)?;
        return Ok(self.message_ref);
    }
}
