use std::{collections::HashMap, mem};

use super::{
    wire::{wire_id::WireId, Wire},
    Circuit,
};
use crate::error::{Error, Result};

#[derive(Clone, Debug, Default)]
pub struct CircuitBuilder {
    wires: HashMap<WireId, Wire>,
}

impl CircuitBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(&mut self) -> Circuit {
        let mut circuit = Circuit::new();
        circuit.wires = mem::take(&mut self.wires);
        circuit.uncomputed = circuit.wires.keys().cloned().collect();
        circuit
    }

    fn add(&mut self, wire: Wire) -> Result<&mut CircuitBuilder> {
        if self.wires.contains_key(wire.id()) {
            Err(Error::WireIdAlreadyExists(wire.id().to_string()))
        } else {
            self.wires.insert(wire.id().to_owned(), wire);
            Ok(self)
        }
    }

    pub fn add_wire(&mut self, s: &str) -> Result<&mut CircuitBuilder> {
        self.add(Wire::try_from(s)?)
    }

    pub fn add_wire_with_value<S: Into<String>>(
        &mut self,
        id: S,
        value: u16,
    ) -> Result<&mut CircuitBuilder> {
        self.add(Wire::with_value(id, value)?)
    }

    pub fn add_wire_from_wire<S: Into<String>, T: Into<String>>(
        &mut self,
        id: S,
        input_id: T,
    ) -> Result<&mut CircuitBuilder> {
        self.add(Wire::from_wire(id, input_id)?)
    }

    pub fn add_gate_and<S: Into<String>, T: Into<String>, U: Into<String>>(
        &mut self,
        output: S,
        input1: T,
        input2: U,
    ) -> Result<&mut CircuitBuilder> {
        self.add(Wire::from_gate_and(output, input1, input2)?)
    }

    pub fn add_gate_and_value<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        value: u16,
    ) -> Result<&mut CircuitBuilder> {
        self.add(Wire::from_gate_and_value(output, input, value)?)
    }

    pub fn add_gate_or<S: Into<String>, T: Into<String>, U: Into<String>>(
        &mut self,
        output: S,
        input1: T,
        input2: U,
    ) -> Result<&mut CircuitBuilder> {
        self.add(Wire::from_gate_or(output, input1, input2)?)
    }

    pub fn add_gate_or_value<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        value: u16,
    ) -> Result<&mut CircuitBuilder> {
        self.add(Wire::from_gate_or_value(output, input, value)?)
    }

    pub fn add_gate_lshift<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        shift: u8,
    ) -> Result<&mut CircuitBuilder> {
        self.add(Wire::from_gate_lshift(output, input, shift)?)
    }

    pub fn add_gate_rshift<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        shift: u8,
    ) -> Result<&mut CircuitBuilder> {
        self.add(Wire::from_gate_rshift(output, input, shift)?)
    }

    pub fn add_gate_not<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
    ) -> Result<&mut CircuitBuilder> {
        self.add(Wire::from_gate_not(output, input)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::wire::signal::Signal;

    #[test]
    fn one_liner() -> Result<()> {
        let mut circuit = CircuitBuilder::new()
            .add_wire_with_value("x", 123)?
            .add_wire_with_value("y", 456)?
            .add_gate_and("d", "x", "y")?
            .add_gate_or("e", "x", "y")?
            .add_gate_lshift("f", "x", 2)?
            .add_gate_rshift("g", "y", 2)?
            .add_gate_not("h", "x")?
            .add_gate_not("i", "y")?
            .build();

        assert!(circuit.compute_signals().is_ok());

        assert_eq!(circuit.signal("d"), Signal::Value(72));
        assert_eq!(circuit.signal("e"), Signal::Value(507));
        assert_eq!(circuit.signal("f"), Signal::Value(492));
        assert_eq!(circuit.signal("g"), Signal::Value(114));
        assert_eq!(circuit.signal("h"), Signal::Value(65412));
        assert_eq!(circuit.signal("i"), Signal::Value(65079));
        assert_eq!(circuit.signal("x"), Signal::Value(123));
        assert_eq!(circuit.signal("y"), Signal::Value(456));
        Ok(())
    }

    #[test]
    fn complex_configuration() -> Result<()> {
        let mut builder = CircuitBuilder::new();
        builder.add_wire_with_value("x", 123)?;
        builder.add_wire_with_value("y", 456)?;
        builder.add_gate_and("d", "x", "y")?;
        builder.add_gate_or("e", "x", "y")?;
        builder.add_gate_lshift("f", "x", 2)?;
        builder.add_gate_rshift("g", "y", 2)?;
        builder.add_gate_not("h", "x")?;
        builder.add_gate_not("i", "y")?;
        let mut circuit = builder.build();

        assert!(circuit.compute_signals().is_ok());

        assert_eq!(circuit.signal("d"), Signal::Value(72));
        assert_eq!(circuit.signal("e"), Signal::Value(507));
        assert_eq!(circuit.signal("f"), Signal::Value(492));
        assert_eq!(circuit.signal("g"), Signal::Value(114));
        assert_eq!(circuit.signal("h"), Signal::Value(65412));
        assert_eq!(circuit.signal("i"), Signal::Value(65079));
        assert_eq!(circuit.signal("x"), Signal::Value(123));
        assert_eq!(circuit.signal("y"), Signal::Value(456));
        Ok(())
    }

    #[test]
    fn xor() -> Result<()> {
        let mut circuit = CircuitBuilder::new()
            .add_wire("2536 -> x")?
            .add_wire("9711 -> y")?
            .add_wire("x OR y -> o")?
            .add_wire("x AND y -> a")?
            .add_wire("NOT a -> na")?
            .add_wire("o AND na -> xor")?
            .build();

        assert!(circuit.compute_signals().is_ok());
        assert_eq!(circuit.signal("xor"), Signal::Value(2536 ^ 9711));
        Ok(())
    }
}
