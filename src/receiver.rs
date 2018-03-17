use websocket::receiver::Reader;
use websocket::stream::sync::TcpStream;

use message::Message;
use error::MessageError;


pub struct Receiver
{
    reader: Reader<TcpStream>,
}

impl Receiver {
    pub fn new(reader: Reader<TcpStream>) -> Receiver {
        Receiver {
            reader: reader,
        }
    }
}

impl Iterator for Receiver {
    type Item = Result<Message, MessageError>;

    fn next(&mut self) -> Option<Self::Item> {
        // convert all messages to a phoenix parsed message
        // and pass through any errors or non-json data along
        let result = self.reader.incoming_messages().next()?;
        return Some(Message::from_result(result));
    }
}
