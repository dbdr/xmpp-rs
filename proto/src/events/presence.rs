use events::{PresenceType, Event, EventTrait, StanzaEvent};
use elementtree::Element;
use std::io;
use std::str::FromStr;
use jid::{Jid, ToJid};
use events::FromXmlElement;
use events::ToXmlElement;

#[derive(Debug, Clone, XmppEvent)]
#[stanza(is="presence")]
pub struct Presence {
    to: Option<Jid>,
    from: Option<Jid>,
    presence_type: Option<PresenceType>,
}

impl Default for Presence {
    fn default() -> Self {
        Self::new()
    }
}

impl Presence {
    pub fn new() -> Presence {
        Presence {
            to: None,
            from: None,
            presence_type: None,
        }
    }

    pub fn set_type(&mut self, presence_type: Option<PresenceType>) -> Result<&mut Self, io::Error> {
        self.presence_type = presence_type;
        Ok(self)
    }

    pub fn get_type(&self) -> Option<&PresenceType> {
        self.presence_type.as_ref()
    }

    pub fn set_from<'a, T: ToJid + ?Sized>(&'a mut self, jid: Option<&T>) -> Result<&'a mut Self, io::Error> {
        self.from = match jid.to_jid() {
            Ok(jid) => Some(jid),
            Err(e) => return Err(e),
        };
        Ok(self)
    }

    pub fn get_from(&self) -> Option<&Jid> {
        self.from.as_ref()
    }

    pub fn set_to<'a, T: ToJid + ?Sized>(&'a mut self, jid: Option<&T>) -> Result<&'a mut Self, io::Error> {
        self.to = match jid.to_jid() {
            Ok(jid) => Some(jid),
            Err(e) => return Err(e),
        };
        Ok(self)
    }

    pub fn get_to(&self) -> Option<&Jid> {
        self.to.as_ref()
    }
}

impl ToXmlElement for Presence {
    type Error = io::Error;
    fn to_element(&self) -> Result<Element, Self::Error> {
        let mut root = Element::new("presence");

        if let Some(presence_type) = self.get_type() {
            root.set_attr("type", presence_type.to_string());
        }

        Ok(root)
    }
}

impl FromXmlElement for Presence {
    type Error = io::Error;
    fn from_element(e: Element) -> Result<Presence, Self::Error> {
        let presence_type = match e.get_attr("type") {
            Some(t) => {
                match PresenceType::from_str(t) {
                    Ok(t) => Some(t),
                    Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, e)),
                }
            }
            None => None,
        };

        let to = match Jid::from_str(e.get_attr("to").unwrap_or("")) {
            Ok(j) => Some(j),
            Err(_) => None,
        };

        let from = match Jid::from_str(e.get_attr("from").unwrap_or("")) {
            Ok(j) => Some(j),
            Err(_) => None,
        };

        Ok(Presence {
               from: from,
               to: to,
               presence_type: presence_type,
           })
    }
}
