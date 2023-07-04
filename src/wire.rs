// use crate::signal::Signal;
use crate::{error::Error, gate::Gate};

pub type WireId = String;

pub enum Source {
    Value(u16),
    Wire(WireId),
    Gate(Gate),
}

pub struct Wire {
    pub id: WireId,
    pub source: Source,
    pub signal: Option<u16>, // TODO: should be private ?
}

impl Wire {
    // pub fn no_source(id: impl Into<String>) -> Result<Self, Error> {
    //     let id = id.into();
    //     id.bytes().all(|b| b.is_ascii_lowercase()).then_some(Self {
    //         id,
    //         source: None,
    //         signal: None,
    //     })
    // }

    pub fn with_value(id: impl Into<String>, value: u16) -> Result<Self, Error> {
        let id = id.into();
        if id.bytes().all(|b| b.is_ascii_lowercase()) {
            Ok(Self {
                id,
                source: Source::Value(value),
                signal: None,
            })
        } else {
            Err(Error::WrongFormatId(id))
        }
    }

    pub fn sourced_from_wire(
        id: impl Into<String>,
        other_id: impl Into<String>,
    ) -> Result<Self, Error> {
        let id = id.into();
        let other_id = other_id.into();
        if id.bytes().all(|b| b.is_ascii_lowercase())
            && other_id.bytes().all(|b| b.is_ascii_lowercase())
        {
            Ok(Self {
                id,
                source: Source::Wire(other_id),
                signal: None,
            })
        } else {
            Err(Error::WrongFormatId(id))
        }
    }

    pub fn sourced_from_gate(id: impl Into<String>, gate: Gate) -> Result<Self, Error> {
        let id = id.into();
        if id.bytes().all(|b| b.is_ascii_lowercase()) {
            Ok(Self {
                id,
                source: Source::Gate(gate),
                signal: None,
            })
        } else {
            Err(Error::WrongFormatId(id))
        }
    }

    pub fn new(id: impl Into<String>, source: Source) -> Result<Self, Error> {
        match source {
            // None => Self::no_source(id),
            Source::Value(value) => Self::with_value(id, value),
            Source::Wire(other_id) => Self::sourced_from_wire(id, other_id),
            Source::Gate(gate) => Self::sourced_from_gate(id, gate),
        }
    }

    // pub fn compute_signal(&self) -> Option<u16> {
    //     if let Some(source) = self.source {
    //         match source {
    //             Source::Value(value) => Some(value),
    //             Source::Wire(wire) => {
    //                 wire.compute_signal();
    //                 wire.signal
    //             }
    //         }
    //     } else {
    //         None
    //     }
    // }

    // pub fn set_signal(&mut self, value: u16) {
    //     self.signal = Some(value);
    // }
}

// impl<'a> Signal for Wire<'a> {
//     fn signal(&self) -> Option<u16> {
//         self.source
//             .map(|s| match s {
//                 Source::Value(value) => Some(value),
//                 Source::Wire(wire) => wire.signal(),
//             })
//             .flatten()
//         // match self.source {
//         //     Source::Value(value) => Some(value),
//         //     Source::Wire(wire) => wire.signal(),
//         // }
//     }
// }

// impl PartialEq for Wire {
//     fn eq(&self, other: &Self) -> bool {
//         self.id == other.id
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn ids() {
//         // assert!(Wire::no_source("A").is_none());
//         // assert!(Wire::no_source("3").is_none());
//         // assert!(Wire::no_source("nano corp").is_none());
//         // assert!(Wire::no_source("nanocorp").is_some());
//         // assert!(Wire::no_source("wire!").is_none());
//         // assert!(Wire::no_source("z\n").is_none());
//     }

//     #[test]
//     fn sources() {
//         // let w1 = Wire::new("a", Source::Value(1)).unwrap();
//         // assert_eq!(w1.signal, Some(1));

//         // let w2 = Wire::new("b", Source::Wire(&w1)).unwrap();
//         // assert_eq!(w2.signal, Some(1));
//     }
// }
