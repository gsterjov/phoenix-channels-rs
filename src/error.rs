use std::io;
use serde_json;
use websocket::result::WebSocketError;
use websocket::client::ParseError;


#[derive(Debug)]
pub enum MessageError {
    WebSocket(WebSocketError),
    Json(serde_json::Error),
}

impl From<WebSocketError> for MessageError {
    fn from(e: WebSocketError) -> Self {
        return MessageError::WebSocket(e);
    }
}

impl From<serde_json::Error> for MessageError {
    fn from(e: serde_json::Error) -> Self {
        return MessageError::Json(e);
    }
}


#[derive(Debug)]
pub enum ConnectError {
    WebSocket(WebSocketError),
    Parse(ParseError),
    IO(io::Error),
}

impl From<WebSocketError> for ConnectError {
    fn from(e: WebSocketError) -> Self {
        return ConnectError::WebSocket(e);
    }
}

impl From<ParseError> for ConnectError {
    fn from(e: ParseError) -> Self {
        return ConnectError::Parse(e);
    }
}

impl From<io::Error> for ConnectError {
    fn from(e: io::Error) -> Self {
        return ConnectError::IO(e);
    }
}


#[derive(Debug)]
pub enum JoinError {
    WebSocket(WebSocketError),
    Json(serde_json::Error),
}

impl From<serde_json::Error> for JoinError {
    fn from(e: serde_json::Error) -> Self {
        return JoinError::Json(e);
    }
}

impl From<WebSocketError> for JoinError {
    fn from(e: WebSocketError) -> Self {
        return JoinError::WebSocket(e);
    }
}
