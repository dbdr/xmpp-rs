use std::io::{self};
use std::fmt;
use std::str::FromStr;
use std::fmt::Debug;

pub trait FromGeneric {
    type Generic;
    type Out;
    fn from_generic<'a>(event: &'a Self::Generic) -> Result<Self::Out, io::Error>;
}

#[derive(Debug, Clone)]
pub enum IqEvent {
    BindEvent(super::Bind),
    GenericEvent(super::GenericIq),
    PingEvent(super::Ping)
}

#[derive(Debug, Clone)]
pub enum StanzaEvent {
    PresenceEvent(super::Presence),
    IqEvent(Box<IqEvent>),
    IqRequestEvent(Box<IqEvent>),
    IqResponseEvent(Box<IqEvent>),
    MessageEvent(super::Message),
}

#[derive(Debug, Clone)]
pub enum NonStanzaEvent {
    OpenStreamEvent(super::OpenStream),
    CloseStreamEvent,
    ProceedTlsEvent(super::ProceedTls),
    SuccessTlsEvent(super::SuccessTls),
    StartTlsEvent(super::StartTls),
    StreamFeaturesEvent(super::StreamFeatures),
    AuthEvent(super::Auth),
}

#[derive(Debug, Clone)]
pub enum Event {
    Unknown(super::Unknown, String),
    NonStanza(Box<NonStanzaEvent>, String),
    Stanza(Box<StanzaEvent>, String),
}

impl Event {
    pub fn is_message(&self) -> bool {
        match *self {
            Event::Stanza(ref stanza, _) => match **stanza {StanzaEvent::MessageEvent(_) => true, _ => false},
            _ => false
        }
    }

    pub fn is_presence(&self) -> bool {
        match *self {
            Event::Stanza(ref stanza, _) => match **stanza {
                StanzaEvent::PresenceEvent(_) => true,
                _ => false
            },
            _ => false,
        }
    }

    pub fn is_iq(&self) -> bool {
        match *self {
            Event::Stanza(ref stanza, _) => match **stanza {
                StanzaEvent::IqEvent(_) |
                StanzaEvent::IqRequestEvent(_) |
                StanzaEvent::IqResponseEvent(_) => true,
                _ => false
            },
            _ => false,
        }
    }

    pub fn is_non_stanza(&self) -> bool {
        match *self {
            Event::NonStanza(_, _) => true,
            _ => false
        }
    }
}

pub trait EventTrait: Debug + ToString + Clone {
    fn to_event(&self) -> Event;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EventType {
    Iq,
    Message,
    Presence
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IqType {
    Get,
    Set,
    Result,
    Error
}

impl FromStr for IqType {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "get" => Ok(IqType::Get),
            "set" => Ok(IqType::Set),
            "result" => Ok(IqType::Result),
            "error" => Ok(IqType::Error),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported IqType"))
        }
    }
}
impl Into<String> for IqType {
    fn into(self) -> String {
        match self {
            IqType::Get => "get".to_string(),
            IqType::Set => "set".to_string(),
            IqType::Result => "result".to_string(),
            IqType::Error => "error".to_string(),
        }
    }
}

impl fmt::Display for IqType {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", match *self {
            IqType::Get => "get".to_string(),
            IqType::Set => "set".to_string(),
            IqType::Result => "result".to_string(),
            IqType::Error => "error".to_string(),
        })
    }
}
