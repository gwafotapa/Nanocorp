use std::{fmt, ops::Deref}; // use crate::signal::Signal;

use crate::{error::Error, gate::Gate, wire_id::WireId};

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

    pub fn new<S: Into<String>>(id: S, input: WireInput) -> Result<Self, Error> {
        match input {
            // None => Self::no_input(id),
            WireInput::Value(value) => Self::with_value(id, value),
            // TODO: deref, as_ref or borrow ?
            WireInput::Wire(input_id) => Self::from_wire(id, input_id.deref()),
            WireInput::Gate(gate) => Self::from_gate(id, gate),
        }
    }

    pub fn with_value<S: Into<String>>(id: S, value: u16) -> Result<Self, Error> {
        let id = WireId::try_from(id.into())?;
        Ok(Self {
            id,
            input: WireInput::Value(value),
            signal: None,
        })
    }

    pub fn from_wire<S: Into<String>, T: Into<String>>(id: S, input_id: T) -> Result<Self, Error> {
        let id = WireId::try_from(id.into())?;
        let input_id = WireId::try_from(input_id.into())?;
        Ok(Self {
            id,
            input: WireInput::Wire(input_id),
            signal: None,
        })
    }

    pub fn from_gate<S: Into<String>>(id: S, gate: Gate) -> Result<Self, Error> {
        let id = WireId::try_from(id.into())?;
        Ok(Self {
            id,
            input: WireInput::Gate(gate),
            signal: None,
        })
    }

    pub fn from_gate_and<S: Into<String>, T: Into<String>, U: Into<String>>(
        id: S,
        input1: T,
        input2: U,
    ) -> Result<Self, Error> {
        let gate = Gate::and(input1, input2)?;
        Wire::from_gate(id, gate)
    }

    pub fn from_gate_and_value<S: Into<String>, T: Into<String>>(
        id: S,
        input: T,
        value: u16,
    ) -> Result<Self, Error> {
        let gate = Gate::and_value(input, value)?;
        Wire::from_gate(id, gate)
    }

    pub fn from_gate_or<S: Into<String>, T: Into<String>, U: Into<String>>(
        id: S,
        input1: T,
        input2: U,
    ) -> Result<Self, Error> {
        let gate = Gate::or(input1, input2)?;
        Wire::from_gate(id, gate)
    }

    pub fn from_gate_or_value<S: Into<String>, T: Into<String>>(
        // TODO: test
        id: S,
        input: T,
        value: u16,
    ) -> Result<Self, Error> {
        let gate = Gate::or_value(input, value)?;
        Wire::from_gate(id, gate)
    }

    pub fn from_gate_sll<S: Into<String>, T: Into<String>>(
        id: S,
        input: T,
        shift: u8,
    ) -> Result<Self, Error> {
        let gate = Gate::sll(input, shift)?;
        Wire::from_gate(id, gate)
    }

    pub fn from_gate_slr<S: Into<String>, T: Into<String>>(
        id: S,
        input: T,
        shift: u8,
    ) -> Result<Self, Error> {
        let gate = Gate::slr(input, shift)?;
        Wire::from_gate(id, gate)
    }

    pub fn from_gate_not<S: Into<String>, T: Into<String>>(id: S, input: T) -> Result<Self, Error> {
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

    // fn get_signal(&self) -> Option<u16> {
    //     self.signal
    // }
}

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
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Error> {
        let (input, output) = s
            .split_once(" -> ")
            .ok_or(Error::ParseArrow(s.to_string()))?;
        let inputs: Vec<&str> = input.split(' ').collect();
        match inputs.len() {
            1 => {
                if let Ok(value) = inputs[0].parse::<u16>() {
                    Wire::with_value(output, value)
                } else {
                    Wire::from_wire(output, inputs[0])
                }
            }
            _ => Wire::from_gate(output, Gate::try_from(input)?),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wire_id() {
        assert!(Wire::from_wire("", "w").is_err());
        assert!(Wire::from_wire("w", "").is_err());
        assert!(Wire::with_value("A", 3).is_err());
        assert!(Wire::from_wire("a", "2").is_err());
        assert!(Wire::with_value("2", 2).is_err());
        assert!(Wire::with_value("nano corp", 9).is_err());
        assert!(Wire::with_value("nanocorp", 9).is_ok());
        assert!(Wire::from_wire("nano", "corp").is_ok());
        assert!(Wire::with_value("wire!", 2).is_err());
        assert!(Wire::with_value("z\n", 0).is_err());
    }

    #[test]
    fn shift_amount() {
        assert!(Wire::from_gate_sll("sll", "w", 0).is_ok());
        assert!(Wire::from_gate_slr("slr", "w", 15).is_ok());
        assert!(Wire::from_gate_slr("slr", "w", 16).is_err());
    }

    #[test]
    fn parse_gate() {
        assert!(Wire::try_from(" -> w").is_err());
        assert!(Wire::try_from("NOT -> w").is_err());
        assert!(Wire::try_from("a AND NOT b -> w").is_err());
        assert!(Wire::try_from("a OR -> w").is_err());
        assert!(Wire::try_from("a NOT b -> w").is_err());
    }

    #[test]
    fn parse_shift() {
        assert!(Wire::try_from("a LSHIFT 0 -> w").is_ok());
        assert!(Wire::try_from("a RSHIFT 15 -> w").is_ok());
        assert!(Wire::try_from("a LSHIFT 16 -> w").is_err());
        assert!(Wire::try_from("a RSHIFT a -> w").is_err());
    }

    #[test]
    fn parse_arrow() {
        assert!(Wire::try_from("x-> w").is_err());
        assert!(Wire::try_from("x ->w").is_err());
        assert!(Wire::try_from("1 ->  -> b").is_err());
        assert!(Wire::try_from("a -> b -> c").is_err());
    }

    #[test]
    fn try_from() {
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
