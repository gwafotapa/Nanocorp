use std::fmt; // use crate::signal::Signal;

use crate::{
    error::{ParseWireError, WireError, WireIdError},
    gate::Gate,
};

pub type WireId = String;

#[derive(Debug, PartialEq)]
pub enum WireInput {
    Value(u16),
    Wire(WireId),
    Gate(Gate),
}

#[derive(Debug, PartialEq)]
pub struct Wire {
    pub id: WireId,
    pub input: WireInput,
    pub signal: Option<u16>, // TODO: should be private ?
}

impl Wire {
    // pub fn no_input(id: S) -> Result<Self, Error> {
    //     let id = id.into();
    //     id.bytes().all(|b| b.is_ascii_lowercase()).then_some(Self {
    //         id,
    //         input: None,
    //         signal: None,
    //     })
    // }

    pub fn new<S: Into<String>>(id: S, input: WireInput) -> Result<Self, WireError> {
        match input {
            // None => Self::no_input(id),
            WireInput::Value(value) => Self::with_value(id, value),
            WireInput::Wire(input_id) => Self::from_wire(id, input_id),
            WireInput::Gate(gate) => Self::from_gate(id, gate),
        }
    }

    pub fn with_value<S: Into<String>>(id: S, value: u16) -> Result<Self, WireError> {
        let id = id.into();
        if id.bytes().all(|b| b.is_ascii_lowercase()) {
            Ok(Self {
                id,
                input: WireInput::Value(value),
                signal: None,
            })
        } else {
            Err(WireIdError(id).into())
        }
    }

    pub fn from_wire<S: Into<String>, T: Into<String>>(
        id: S,
        input_id: T,
    ) -> Result<Self, WireError> {
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
            Err(WireIdError(id).into())
        }
    }

    pub fn from_gate<S: Into<String>>(id: S, gate: Gate) -> Result<Self, WireError> {
        let id = id.into();
        if id.bytes().all(|b| b.is_ascii_lowercase()) {
            Ok(Self {
                id,
                input: WireInput::Gate(gate),
                signal: None,
            })
        } else {
            Err(WireIdError(id).into())
        }
    }

    pub fn from_gate_and<S: Into<String>, T: Into<String>, U: Into<String>>(
        id: S,
        input1: T,
        input2: U,
    ) -> Result<Self, WireError> {
        let gate = Gate::and(input1, input2)?;
        Wire::from_gate(id, gate)
    }

    pub fn from_gate_and_value<S: Into<String>, T: Into<String>>(
        id: S,
        input: T,
        value: u16,
    ) -> Result<Self, WireError> {
        let gate = Gate::and_value(input, value)?;
        Wire::from_gate(id, gate)
    }

    pub fn from_gate_or<S: Into<String>, T: Into<String>, U: Into<String>>(
        id: S,
        input1: T,
        input2: U,
    ) -> Result<Self, WireError> {
        let gate = Gate::or(input1, input2)?;
        Wire::from_gate(id, gate)
    }

    pub fn from_gate_or_value<S: Into<String>, T: Into<String>>(
        // TODO: test
        id: S,
        input: T,
        value: u16,
    ) -> Result<Self, WireError> {
        let gate = Gate::or_value(input, value)?;
        Wire::from_gate(id, gate)
    }

    pub fn from_gate_sll<S: Into<String>, T: Into<String>>(
        id: S,
        input: T,
        shift: u8,
    ) -> Result<Self, WireError> {
        let gate = Gate::sll(input, shift)?;
        Wire::from_gate(id, gate)
    }

    pub fn from_gate_slr<S: Into<String>, T: Into<String>>(
        id: S,
        input: T,
        shift: u8,
    ) -> Result<Self, WireError> {
        let gate = Gate::slr(input, shift)?;
        Wire::from_gate(id, gate)
    }

    pub fn from_gate_not<S: Into<String>, T: Into<String>>(
        id: S,
        input: T,
    ) -> Result<Self, WireError> {
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

impl fmt::Display for Wire {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.input {
            WireInput::Value(value) => {
                write!(f, "{} -> {}", value, self.id)
            }
            WireInput::Wire(input_id) => {
                write!(f, "{} -> {}", input_id, self.id)
            }
            WireInput::Gate(gate) => {
                write!(f, "{} -> {}", gate, self.id)
            }
        }
    }
}

impl TryFrom<&str> for Wire {
    type Error = ParseWireError;

    fn try_from(s: &str) -> Result<Self, ParseWireError> {
        let (input, output) = s
            .split_once(" -> ")
            .ok_or(ParseWireError::MissingArrow(s.to_string()))?;
        let inputs: Vec<&str> = input.split(' ').collect();
        let wire_input = match inputs.len() {
            1 => {
                if let Ok(value) = inputs[0].parse::<u16>() {
                    WireInput::Value(value)
                } else {
                    WireInput::Wire(inputs[0].to_string())
                }
            }
            _ => WireInput::Gate(Gate::try_from(input)?),
        };
        Ok(Wire::new(output, wire_input)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn ids() {
    // assert!(Wire::no_input("A").is_none());
    // assert!(Wire::no_input("3").is_none());
    // assert!(Wire::no_input("nano corp").is_none());
    // assert!(Wire::no_input("nanocorp").is_some());
    // assert!(Wire::no_input("wire!").is_none());
    // assert!(Wire::no_input("z\n").is_none());
    // }

    // #[test]
    // fn inputs() {
    // let w1 = Wire::new("a", WireInput::Value(1)).unwrap();
    // assert_eq!(w1.signal, Some(1));

    // let w2 = Wire::new("b", WireInput::Wire(&w1)).unwrap();
    // assert_eq!(w2.signal, Some(1));
    // }

    #[test]
    fn from() {
        let w1 = Wire::try_from("456 -> y").unwrap();
        let w2 = Wire::with_value("y", 456).unwrap();
        assert_eq!(w1, w2);

        let w1 = Wire::try_from("x LSHIFT 2 -> f").unwrap();
        let w2 = Wire::from_gate_sll("f", "x", 2).unwrap();
        assert_eq!(w1, w2);

        let w1 = Wire::try_from("NOT x -> h").unwrap();
        let w2 = Wire::from_gate_not("h", "x").unwrap();
        assert_eq!(w1, w2);

        let w1 = Wire::try_from("x OR y -> e").unwrap();
        let w2 = Wire::from_gate_or("e", "x", "y").unwrap();
        assert_eq!(w1, w2);

        let w1 = Wire::try_from("y RSHIFT 2 -> g").unwrap();
        let w2 = Wire::from_gate_slr("g", "y", 2).unwrap();
        assert_eq!(w1, w2);

        let w1 = Wire::try_from("NOT y -> i").unwrap();
        let w2 = Wire::from_gate_not("i", "y").unwrap();
        assert_eq!(w1, w2);

        let w1 = Wire::try_from("123 -> x").unwrap();
        let w2 = Wire::with_value("x", 123).unwrap();
        assert_eq!(w1, w2);

        let w1 = Wire::try_from("x AND y -> d").unwrap();
        let w2 = Wire::from_gate_and("d", "x", "y").unwrap();
        assert_eq!(w1, w2);
    }
}
