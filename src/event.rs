use serde::de::{Deserialize, Deserializer, Visitor, Error, Unexpected};
use serde::ser::{Serialize, Serializer};

use std::fmt;


#[derive(Debug)]
pub enum EventKind {
    Close,
    Error,
    Join,
    Leave,
    Reply,
}


impl<'de> Deserialize<'de> for EventKind {
    fn deserialize<D>(deserializer: D) -> Result<EventKind, D::Error>
    where D: Deserializer<'de>
    {
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = EventKind;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a string with a value of [phx_close, phx_error, phx_join, phx_leave, phx_reply]")
            }

            fn visit_str<E>(self, value: &str) -> Result<EventKind, E>
            where E: Error
            {
                match value {
                    "phx_close" => Ok(EventKind::Close),
                    "phx_error" => Ok(EventKind::Error),
                    "phx_join" => Ok(EventKind::Join),
                    "phx_leave" => Ok(EventKind::Leave),
                    "phx_reply" => Ok(EventKind::Reply),
                    s => Err(E::invalid_value(Unexpected::Str(s), &self)),
                }
            }
        }

        deserializer.deserialize_str(FieldVisitor)
    }
}


impl Serialize for EventKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
    {
        let kind = match *self {
            EventKind::Close => "phx_close",
            EventKind::Error => "phx_error",
            EventKind::Join => "phx_join",
            EventKind::Leave => "phx_leave",
            EventKind::Reply => "phx_reply",
        };

        serializer.serialize_str(kind)
    }
}
