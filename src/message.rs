use serde_json;
use websocket::OwnedMessage;
use websocket::result::WebSocketResult;

use event::EventKind;
use error::MessageError;


#[derive(Debug, Serialize, Deserialize)]
pub struct PhoenixMessage
{
    join_ref: Option<u32>,
    message_ref: u32,
    topic: String,
    event: EventKind,
    payload: serde_json::Value,
}


#[derive(Debug)]
pub enum Message {
    Json(PhoenixMessage),
    Binary,
    Close,
    Ping,
    Pong,
}


impl Message {
    pub fn from_owned(owned: OwnedMessage) -> Result<Self, MessageError> {
        let message = match owned {
            OwnedMessage::Text(text) => Message::Json(serde_json::from_str(&text)?),
            OwnedMessage::Binary(_) => Message::Binary,
            OwnedMessage::Close(_) => Message::Close,
            OwnedMessage::Ping(_) => Message::Ping,
            OwnedMessage::Pong(_) => Message::Pong,
        };

        return Ok(message);
    }

    pub fn from_result(result: WebSocketResult<OwnedMessage>) -> Result<Self, MessageError> {
        return Message::from_owned(result?);
    }
}
