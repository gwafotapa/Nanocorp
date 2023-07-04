// use crate::signal::Signal;
use crate::{error::Error, gate::Gate};

pub type WireId = String;

pub enum WireInput {
    Value(u16),
    Wire(WireId),
    Gate(Gate),
}

pub struct Wire {
    pub id: WireId,
    pub input: WireInput,
    pub signal: Option<u16>, // TODO: should be private ?
}

impl Wire {
    // pub fn no_input(id: impl Into<String>) -> Result<Self, Error> {
    //     let id = id.into();
    //     id.bytes().all(|b| b.is_ascii_lowercase()).then_some(Self {
    //         id,
    //         input: None,
    //         signal: None,
    //     })
    // }

    pub fn new(id: impl Into<String>, input: WireInput) -> Result<Self, Error> {
        match input {
            // None => Self::no_input(id),
            WireInput::Value(value) => Self::with_value(id, value),
            WireInput::Wire(input_id) => Self::from_wire(id, input_id),
            WireInput::Gate(gate) => Self::from_gate(id, gate),
        }
    }

    pub fn with_value(id: impl Into<String>, value: u16) -> Result<Self, Error> {
        let id = id.into();
        if id.bytes().all(|b| b.is_ascii_lowercase()) {
            Ok(Self {
                id,
                input: WireInput::Value(value),
                signal: None,
            })
        } else {
            Err(Error::WrongFormatId(id))
        }
    }

    pub fn from_wire(id: impl Into<String>, input_id: impl Into<String>) -> Result<Self, Error> {
        let id = id.into();
        let input_id = input_id.into();
        if id.bytes().all(|b| b.is_ascii_lowercase())
            && input_id.bytes().all(|b| b.is_ascii_lowercase())
        {
            Ok(Self {
                id,
                input: WireInput::Wire(input_id),
                signal: None,
            })
        } else {
            Err(Error::WrongFormatId(id))
        }
    }

    pub fn from_gate(id: impl Into<String>, gate: Gate) -> Result<Self, Error> {
        let id = id.into();
        if id.bytes().all(|b| b.is_ascii_lowercase()) {
            Ok(Self {
                id,
                input: WireInput::Gate(gate),
                signal: None,
            })
        } else {
            Err(Error::WrongFormatId(id))
        }
    }

    pub fn from_gate_and(
        id: impl Into<String>,
        input1: impl Into<String>,
        input2: impl Into<String>,
    ) -> Result<Self, Error> {
        let gate = Gate::and(input1, input2)?;
        Wire::from_gate(id, gate)
    }

    pub fn from_gate_or(
        id: impl Into<String>,
        input1: impl Into<String>,
        input2: impl Into<String>,
    ) -> Result<Self, Error> {
        let gate = Gate::or(input1, input2)?;
        Wire::from_gate(id, gate)
    }

    pub fn from_gate_sll(
        id: impl Into<String>,
        input: impl Into<String>,
        shift: u8,
    ) -> Result<Self, Error> {
        let gate = Gate::sll(input, shift)?;
        Wire::from_gate(id, gate)
    }

    pub fn from_gate_slr(
        id: impl Into<String>,
        input: impl Into<String>,
        shift: u8,
    ) -> Result<Self, Error> {
        let gate = Gate::slr(input, shift)?;
        Wire::from_gate(id, gate)
    }

    pub fn from_gate_not(id: impl Into<String>, input: impl Into<String>) -> Result<Self, Error> {
        let gate = Gate::not(input)?;
        Wire::from_gate(id, gate)
    }

    // pub fn compute_signal(&self) -> Option<u16> {
    //     if let Some(input) = self.input {
    //         match input {
    //             WireInput::Value(value) => Some(value),
    //             WireInput::Wire(wire) => {
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
//         self.input
//             .map(|s| match s {
//                 WireInput::Value(value) => Some(value),
//                 WireInput::Wire(wire) => wire.signal(),
//             })
//             .flatten()
//         // match self.input {
//         //     WireInput::Value(value) => Some(value),
//         //     WireInput::Wire(wire) => wire.signal(),
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
//         // assert!(Wire::no_input("A").is_none());
//         // assert!(Wire::no_input("3").is_none());
//         // assert!(Wire::no_input("nano corp").is_none());
//         // assert!(Wire::no_input("nanocorp").is_some());
//         // assert!(Wire::no_input("wire!").is_none());
//         // assert!(Wire::no_input("z\n").is_none());
//     }

//     #[test]
//     fn inputs() {
//         // let w1 = Wire::new("a", WireInput::Value(1)).unwrap();
//         // assert_eq!(w1.signal, Some(1));

//         // let w2 = Wire::new("b", WireInput::Wire(&w1)).unwrap();
//         // assert_eq!(w2.signal, Some(1));
//     }
// }
