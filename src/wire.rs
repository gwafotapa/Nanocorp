// use crate::signal::Signal;
use crate::gate::Gate;

pub type WireId = String;

pub enum Source {
    Value(u16),
    Wire(WireId),
    Gate(Gate),
}

pub struct Wire {
    pub id: WireId,
    pub source: Option<Source>,
    pub signal: Option<u16>, // TODO: should be private ?
}

impl Wire {
    pub fn no_source(id: impl Into<String>) -> Option<Self> {
        let id = id.into();
        id.bytes().all(|b| b.is_ascii_lowercase()).then_some(Self {
            id,
            source: None,
            signal: None,
        })
    }

    pub fn source_value(id: impl Into<String>, value: u16) -> Option<Self> {
        let id = id.into();
        id.bytes().all(|b| b.is_ascii_lowercase()).then_some(Self {
            id,
            source: Some(Source::Value(value)),
            signal: None,
        })
    }

    pub fn source_other_wire(id: impl Into<String>, other_id: impl Into<String>) -> Option<Self> {
        let id = id.into();
        let other_id = other_id.into();
        (id.bytes().all(|b| b.is_ascii_lowercase())
            && other_id.bytes().all(|b| b.is_ascii_lowercase()))
        .then_some(Self {
            id,
            source: Some(Source::Wire(other_id)),
            signal: None,
        })
    }

    pub fn source_gate(id: impl Into<String>, gate: Gate) -> Option<Self> {
        let id = id.into();
        id.bytes().all(|b| b.is_ascii_lowercase()).then_some(Self {
            id,
            source: Some(Source::Gate(gate)),
            signal: None,
        })
    }

    pub fn new(id: impl Into<String>, source: Option<Source>) -> Option<Self> {
        match source {
            None => Self::no_source(id),
            Some(Source::Value(value)) => Self::source_value(id, value),
            Some(Source::Wire(other_id)) => Self::source_other_wire(id, other_id),
            Some(Source::Gate(gate)) => Self::source_gate(id, gate),
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

impl PartialEq for Wire {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids() {
        assert!(Wire::no_source("A").is_none());
        assert!(Wire::no_source("3").is_none());
        assert!(Wire::no_source("nano corp").is_none());
        assert!(Wire::no_source("nanocorp").is_some());
        assert!(Wire::no_source("wire!").is_none());
        assert!(Wire::no_source("z\n").is_none());
    }

    #[test]
    fn sources() {
        // let w1 = Wire::new("a", Source::Value(1)).unwrap();
        // assert_eq!(w1.signal, Some(1));

        // let w2 = Wire::new("b", Source::Wire(&w1)).unwrap();
        // assert_eq!(w2.signal, Some(1));
    }
}
