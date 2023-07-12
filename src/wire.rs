pub(super) mod gate;
pub mod signal;
pub(super) mod wire_id;
pub(super) mod wire_input;

use std::fmt::{self, Display, Formatter};

use crate::error::{Error, Result};
use gate::Gate;
use signal::Signal;
use wire_id::WireId;
use wire_input::WireInput;

#[derive(Clone, Debug)]
pub(super) struct Wire {
    id: WireId,
    input: WireInput,
    signal: Signal,
}

impl Wire {
    fn new(id: WireId, input: WireInput) -> Result<Self> {
        match &input {
            WireInput::Value(_) => {}
            WireInput::Wire(input_id) => {
                if &id == input_id {
                    return Err(Error::InputMatchesOutput(id.to_string()));
                }
            }
            WireInput::Gate(gate) => {
                if gate.has_input(&id) {
                    return Err(Error::InputMatchesOutput(id.to_string()));
                }
            }
        }
        Ok(Self {
            id,
            input,
            signal: Signal::default(),
        })
    }

    pub fn with_value<S: Into<String>>(id: S, value: u16) -> Result<Self> {
        Self::new(WireId::new(id)?, WireInput::Value(value))
    }

    pub fn from_wire<S: Into<String>, T: Into<String>>(id: S, input_id: T) -> Result<Self> {
        Self::new(WireId::new(id)?, WireInput::Wire(WireId::new(input_id)?))
    }

    pub(crate) fn from_gate<S: Into<String>>(id: S, gate: Gate) -> Result<Self> {
        Self::new(WireId::new(id)?, WireInput::Gate(gate))
    }

    pub fn from_gate_and<S: Into<String>, T: Into<String>, U: Into<String>>(
        id: S,
        input1: T,
        input2: U,
    ) -> Result<Self> {
        Wire::from_gate(id, Gate::and(input1, input2)?)
    }

    pub fn from_gate_and_value<S: Into<String>, T: Into<String>>(
        id: S,
        input: T,
        value: u16,
    ) -> Result<Self> {
        Wire::from_gate(id, Gate::and_value(input, value)?)
    }

    pub fn from_gate_or<S: Into<String>, T: Into<String>, U: Into<String>>(
        id: S,
        input1: T,
        input2: U,
    ) -> Result<Self> {
        Wire::from_gate(id, Gate::or(input1, input2)?)
    }

    pub fn from_gate_or_value<S: Into<String>, T: Into<String>>(
        id: S,
        input: T,
        value: u16,
    ) -> Result<Self> {
        Wire::from_gate(id, Gate::or_value(input, value)?)
    }

    pub fn from_gate_lshift<S: Into<String>, T: Into<String>>(
        id: S,
        input: T,
        shift: u8,
    ) -> Result<Self> {
        Wire::from_gate(id, Gate::lshift(input, shift)?)
    }

    pub fn from_gate_rshift<S: Into<String>, T: Into<String>>(
        id: S,
        input: T,
        shift: u8,
    ) -> Result<Self> {
        Wire::from_gate(id, Gate::rshift(input, shift)?)
    }

    pub fn from_gate_not<S: Into<String>, T: Into<String>>(id: S, input: T) -> Result<Self> {
        Wire::from_gate(id, Gate::not(input)?)
    }

    pub(super) fn id(&self) -> &WireId {
        &self.id
    }

    pub(super) fn input(&self) -> &WireInput {
        &self.input
    }

    pub fn signal(&self) -> &Signal {
        &self.signal
    }

    pub(super) fn set_signal(&mut self, signal: Signal) {
        self.signal = signal;
    }
}

impl TryFrom<&str> for Wire {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        let (input, output) = s
            .split_once(" -> ")
            .ok_or(Error::ParseArrow(s.to_string()))?;
        let inputs: Vec<&str> = input.split(' ').collect();
        match inputs.len() {
            1 => {
                if let Ok(value) = inputs[0].parse::<u16>() {
                    Wire::with_value(output, value)
                } else if let Ok(value) = inputs[0].parse::<u64>() {
                    Err(Error::TooLargeValue(value))
                } else {
                    Wire::from_wire(output, inputs[0])
                }
            }
            _ => Wire::from_gate(output, Gate::try_from(input)?),
        }
    }
}

impl Display for Wire {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wire_id() {
        assert!(matches!(
            Wire::from_wire("", "w"),
            Err(Error::InvalidWireId(_))
        ));
        assert!(matches!(
            Wire::from_wire("w", ""),
            Err(Error::InvalidWireId(_))
        ));
        assert!(matches!(
            Wire::with_value("A", 3),
            Err(Error::InvalidWireId(_))
        ));
        assert!(matches!(
            Wire::from_wire("a", "2"),
            Err(Error::InvalidWireId(_))
        ));
        assert!(matches!(
            Wire::with_value("2", 2),
            Err(Error::InvalidWireId(_))
        ));
        assert!(matches!(
            Wire::with_value("nano corp", 9),
            Err(Error::InvalidWireId(_))
        ));
        assert!(matches!(
            Wire::with_value("wire!", 2),
            Err(Error::InvalidWireId(_))
        ));
        assert!(matches!(
            Wire::with_value("z\n", 0),
            Err(Error::InvalidWireId(_))
        ));
        assert!(matches!(
            Wire::try_from("1 ->  -> b"),
            Err(Error::InvalidWireId(_))
        ));
        assert!(matches!(
            Wire::try_from("a -> b -> c"),
            Err(Error::InvalidWireId(_))
        ));
        assert!(matches!(
            Wire::try_from(" -> w"),
            Err(Error::InvalidWireId(_))
        ));
        assert!(matches!(
            Wire::try_from("NOT -> w"),
            Err(Error::InvalidWireId(_))
        ));

        assert!(Wire::with_value("nanocorp", 9).is_ok());
        assert!(Wire::from_wire("nano", "corp").is_ok());
    }

    #[test]
    fn signal_value() {
        assert!(Wire::try_from("65535 -> w").is_ok());
        assert!(matches!(
            Wire::try_from("65536 -> w"),
            Err(Error::TooLargeValue(65536))
        ));
    }

    #[test]
    fn shift_amount() {
        assert!(Wire::from_gate_lshift("lshift", "w", 0).is_ok());
        assert!(Wire::from_gate_rshift("rshift", "w", 15).is_ok());
        assert!(Wire::try_from("a LSHIFT 0 -> w").is_ok());
        assert!(Wire::try_from("a RSHIFT 15 -> w").is_ok());
        assert!(matches!(
            Wire::from_gate_rshift("rshift", "w", 16),
            Err(Error::TooLargeShift(16))
        ));
        assert!(matches!(
            Wire::try_from("a LSHIFT 16 -> w"),
            Err(Error::TooLargeShift(16))
        ));
    }

    #[test]
    fn parse_gate() {
        assert!(matches!(
            Wire::try_from("a AND NOT b -> w"),
            Err(Error::ParseGate(_))
        ));
        assert!(matches!(
            Wire::try_from("a OR -> w"),
            Err(Error::ParseGate(_))
        ));
        assert!(matches!(
            Wire::try_from("a NOT b -> w"),
            Err(Error::ParseGate(_))
        ));
    }

    #[test]
    fn parse_shift() {
        assert!(matches!(
            Wire::try_from("a RSHIFT b -> w"),
            Err(Error::ParseShift(_))
        ));
        assert!(matches!(
            Wire::try_from("a RSHIFT -2 -> w"),
            Err(Error::ParseShift(_))
        ));
    }

    #[test]
    fn parse_arrow() {
        assert!(matches!(Wire::try_from(""), Err(Error::ParseArrow(_))));
        assert!(matches!(Wire::try_from("x w"), Err(Error::ParseArrow(_))));
        assert!(matches!(Wire::try_from("x-> w"), Err(Error::ParseArrow(_))));
        assert!(matches!(Wire::try_from("x ->w"), Err(Error::ParseArrow(_))));
    }

    #[test]
    fn input_matches_output() {
        assert!(matches!(
            Wire::try_from("w -> w"),
            Err(Error::InputMatchesOutput(_))
        ));
        assert!(matches!(
            Wire::from_gate_and("w", "w", "x"),
            Err(Error::InputMatchesOutput(_))
        ));
        assert!(matches!(
            Wire::from_gate_and("w", "x", "w"),
            Err(Error::InputMatchesOutput(_))
        ));
        assert!(matches!(
            Wire::from_gate_or("w", "w", "x"),
            Err(Error::InputMatchesOutput(_))
        ));
        assert!(matches!(
            Wire::from_gate_or("w", "x", "w"),
            Err(Error::InputMatchesOutput(_))
        ));
        assert!(matches!(
            Wire::try_from("w AND 1 -> w"),
            Err(Error::InputMatchesOutput(_))
        ));
        assert!(matches!(
            Wire::try_from("1 OR w -> w"),
            Err(Error::InputMatchesOutput(_))
        ));
        assert!(matches!(
            Wire::from_gate_lshift("w", "w", 1),
            Err(Error::InputMatchesOutput(_))
        ));
        assert!(matches!(
            Wire::from_gate_rshift("w", "w", 1),
            Err(Error::InputMatchesOutput(_))
        ));
        assert!(matches!(
            Wire::from_gate_not("w", "w"),
            Err(Error::InputMatchesOutput(_))
        ));
    }

    #[test]
    fn try_from() -> Result<()> {
        let w1 = Wire::try_from("456 -> y")?;
        let w2 = Wire::with_value("y", 456)?;
        assert_eq!(w1.id, w2.id);
        assert_eq!(w1.input, w2.input);
        assert_eq!(w1.signal, w2.signal);

        let w1 = Wire::try_from("x LSHIFT 2 -> f")?;
        let w2 = Wire::from_gate_lshift("f", "x", 2)?;
        assert_eq!(w1.id, w2.id);
        assert_eq!(w1.input, w2.input);
        assert_eq!(w1.signal, w2.signal);

        let w1 = Wire::try_from("NOT x -> h")?;
        let w2 = Wire::from_gate_not("h", "x")?;
        assert_eq!(w1.id, w2.id);
        assert_eq!(w1.input, w2.input);
        assert_eq!(w1.signal, w2.signal);

        let w1 = Wire::try_from("x OR y -> e")?;
        let w2 = Wire::from_gate_or("e", "x", "y")?;
        assert_eq!(w1.id, w2.id);
        assert_eq!(w1.input, w2.input);
        assert_eq!(w1.signal, w2.signal);

        let w1 = Wire::try_from("y RSHIFT 2 -> g")?;
        let w2 = Wire::from_gate_rshift("g", "y", 2)?;
        assert_eq!(w1.id, w2.id);
        assert_eq!(w1.input, w2.input);
        assert_eq!(w1.signal, w2.signal);

        let w1 = Wire::try_from("NOT y -> i")?;
        let w2 = Wire::from_gate_not("i", "y")?;
        assert_eq!(w1.id, w2.id);
        assert_eq!(w1.input, w2.input);
        assert_eq!(w1.signal, w2.signal);

        let w1 = Wire::try_from("123 -> x")?;
        let w2 = Wire::with_value("x", 123)?;
        assert_eq!(w1.id, w2.id);
        assert_eq!(w1.input, w2.input);
        assert_eq!(w1.signal, w2.signal);

        let w1 = Wire::try_from("x AND y -> d")?;
        let w2 = Wire::from_gate_and("d", "x", "y")?;
        assert_eq!(w1.id, w2.id);
        assert_eq!(w1.input, w2.input);
        assert_eq!(w1.signal, w2.signal);
        Ok(())
    }
}
