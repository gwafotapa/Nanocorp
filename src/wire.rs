use std::fmt; // use crate::signal::Signal;

use crate::{error::Error, gate::Gate};

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

impl fmt::Display for Wire {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.input {
            WireInput::Value(value) => {
                write!(f, "{} -> {}", value, self.id)
            }
            WireInput::Wire(input_id) => {
                write!(f, "{} -> {}", input_id, self.id)
            }
            WireInput::Gate(gate) => match gate {
                Gate::And { input1, input2 } => {
                    write!(f, "{} AND {} -> {}", input1, input2, self.id)
                }
                Gate::Or { input1, input2 } => {
                    write!(f, "{} OR {} -> {}", input1, input2, self.id)
                }
                Gate::SLL { input, shift } => {
                    write!(f, "{} LSHIFT {} -> {}", input, shift, self.id)
                }
                Gate::SLR { input, shift } => {
                    write!(f, "{} RSHIFT {} -> {}", input, shift, self.id)
                }
                Gate::Not { input } => {
                    write!(f, "NOT {} -> {}", input, self.id)
                }
            },
        }
    }
}

impl From<&str> for Wire {
    fn from(s: &str) -> Self {
        let (input, output) = s.split_once(" -> ").unwrap();
        let inputs: Vec<&str> = input.split(' ').collect();
        let wire_input = match inputs.len() {
            1 => {
                if inputs[0].as_bytes()[0].is_ascii_lowercase() {
                    WireInput::Wire(inputs[0].to_string())
                } else {
                    WireInput::Value(inputs[0].parse::<u16>().unwrap())
                }
            }
            2 => WireInput::Gate(Gate::not(inputs[1]).unwrap()),
            3 => WireInput::Gate(match inputs[1] {
                "AND" => Gate::and(inputs[0], inputs[2]).unwrap(),
                "OR" => Gate::or(inputs[0], inputs[2]).unwrap(),
                "LSHIFT" => Gate::sll(inputs[0], inputs[2].parse::<u8>().unwrap()).unwrap(),
                "RSHIFT" => Gate::slr(inputs[0], inputs[2].parse::<u8>().unwrap()).unwrap(),
                _ => panic!("Cannot convert string \"{}\" to wire", s),
            }),
            _ => panic!("Cannot convert string \"{}\" to wire", s),
        };
        Wire::new(output, wire_input).unwrap()
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
        let w1 = Wire::from("456 -> y");
        let w2 = Wire::with_value("y", 456).unwrap();
        assert_eq!(w1, w2);

        let w1 = Wire::from("x LSHIFT 2 -> f");
        let w2 = Wire::from_gate_sll("f", "x", 2).unwrap();
        assert_eq!(w1, w2);

        let w1 = Wire::from("NOT; x -> h");
        let w2 = Wire::from_gate_not("h", "x").unwrap();
        assert_eq!(w1, w2);

        let w1 = Wire::from("x OR y -> e");
        let w2 = Wire::from_gate_or("e", "x", "y").unwrap();
        assert_eq!(w1, w2);

        let w1 = Wire::from("y RSHIFT 2 -> g");
        let w2 = Wire::from_gate_slr("g", "y", 2).unwrap();
        assert_eq!(w1, w2);

        let w1 = Wire::from("NOT y -> i");
        let w2 = Wire::from_gate_not("i", "y").unwrap();
        assert_eq!(w1, w2);

        let w1 = Wire::from("123 -> x");
        let w2 = Wire::with_value("x", 123).unwrap();
        assert_eq!(w1, w2);

        let w1 = Wire::from("x AND y -> d");
        let w2 = Wire::from_gate_and("d", "x", "y").unwrap();
        assert_eq!(w1, w2);
    }
}
