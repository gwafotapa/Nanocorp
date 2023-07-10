use std::{collections::HashMap, mem};

use super::{
    wire::{wire_id::WireId, Wire},
    Circuit,
};

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

    pub fn add(&mut self, wire: Wire) -> &mut CircuitBuilder {
        self.wires.insert(wire.id().to_owned(), wire);
        self
    }

    // pub fn add_wire_with_input<S: Into<String>>(
    //     &mut self,
    //     id: S,
    //     input: WireInput,
    // ) -> &mut CircuitBuilder {
    //     let wire = Wire::new(id, input).unwrap();
    //     self.add(wire);
    //     self
    // }

    pub fn add_wire_with_value<S: Into<String>>(
        &mut self,
        id: S,
        value: u16,
    ) -> &mut CircuitBuilder {
        self.add(Wire::with_value(id, value).unwrap());
        self
    }

    pub fn add_wire_from_wire<S: Into<String>, T: Into<String>>(
        &mut self,
        id: S,
        input_id: T,
    ) -> &mut CircuitBuilder {
        self.add(Wire::from_wire(id, input_id).unwrap());
        self
    }

    // fn add_wire_from_gate<S: Into<String>>(&mut self, id: S, gate: Gate) -> &mut CircuitBuilder {
    //     let wire = Wire::from_gate(id, gate).unwrap();
    //     self.add(wire);
    //     self
    // }

    pub fn add_gate_and<S: Into<String>, T: Into<String>, U: Into<String>>(
        &mut self,
        output: S,
        input1: T,
        input2: U,
    ) -> &mut CircuitBuilder {
        self.add(Wire::from_gate_and(output, input1, input2).unwrap());
        self
    }

    pub fn add_gate_and_value<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        value: u16,
    ) -> &mut CircuitBuilder {
        self.add(Wire::from_gate_and_value(output, input, value).unwrap());
        self
    }

    pub fn add_gate_or<S: Into<String>, T: Into<String>, U: Into<String>>(
        &mut self,
        output: S,
        input1: T,
        input2: U,
    ) -> &mut CircuitBuilder {
        self.add(Wire::from_gate_or(output, input1, input2).unwrap());
        self
    }

    pub fn add_gate_or_value<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        value: u16,
    ) -> &mut CircuitBuilder {
        self.add(Wire::from_gate_or_value(output, input, value).unwrap());
        self
    }

    pub fn add_gate_lshift<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        shift: u8,
    ) -> &mut CircuitBuilder {
        self.add(Wire::from_gate_lshift(output, input, shift).unwrap());
        self
    }

    pub fn add_gate_rshift<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        shift: u8,
    ) -> &mut CircuitBuilder {
        self.add(Wire::from_gate_rshift(output, input, shift).unwrap());
        self
    }

    pub fn add_gate_not<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
    ) -> &mut CircuitBuilder {
        self.add(Wire::from_gate_not(output, input).unwrap());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::wire::signal::Signal;

    #[test]
    fn one_liner() {
        let mut circuit = CircuitBuilder::new()
            .add_wire_with_value("x", 123)
            .add_wire_with_value("y", 456)
            .add_gate_and("d", "x", "y")
            .add_gate_or("e", "x", "y")
            .add_gate_lshift("f", "x", 2)
            .add_gate_rshift("g", "y", 2)
            .add_gate_not("h", "x")
            .add_gate_not("i", "y")
            .build();

        assert!(circuit.compute_signals().is_ok());

        assert_eq!(circuit.signal_from("d"), Signal::Value(72));
        assert_eq!(circuit.signal_from("e"), Signal::Value(507));
        assert_eq!(circuit.signal_from("f"), Signal::Value(492));
        assert_eq!(circuit.signal_from("g"), Signal::Value(114));
        assert_eq!(circuit.signal_from("h"), Signal::Value(65412));
        assert_eq!(circuit.signal_from("i"), Signal::Value(65079));
        assert_eq!(circuit.signal_from("x"), Signal::Value(123));
        assert_eq!(circuit.signal_from("y"), Signal::Value(456));
    }

    #[test]
    fn complex_configuration() {
        let mut builder = CircuitBuilder::new();
        builder.add_wire_with_value("x", 123);
        builder.add_wire_with_value("y", 456);
        builder.add_gate_and("d", "x", "y");
        builder.add_gate_or("e", "x", "y");
        builder.add_gate_lshift("f", "x", 2);
        builder.add_gate_rshift("g", "y", 2);
        builder.add_gate_not("h", "x");
        builder.add_gate_not("i", "y");
        let mut circuit = builder.build();

        assert!(circuit.compute_signals().is_ok());

        assert_eq!(circuit.signal_from("d"), Signal::Value(72));
        assert_eq!(circuit.signal_from("e"), Signal::Value(507));
        assert_eq!(circuit.signal_from("f"), Signal::Value(492));
        assert_eq!(circuit.signal_from("g"), Signal::Value(114));
        assert_eq!(circuit.signal_from("h"), Signal::Value(65412));
        assert_eq!(circuit.signal_from("i"), Signal::Value(65079));
        assert_eq!(circuit.signal_from("x"), Signal::Value(123));
        assert_eq!(circuit.signal_from("y"), Signal::Value(456));
    }

    #[test]
    fn xor() {
        let mut circuit = CircuitBuilder::new()
            .add_wire_with_value("x", 0xbae5)
            .add_wire_with_value("y", 0x10e6)
            .add_gate_or("xoy", "x", "y")
            .add_gate_and("xay", "x", "y")
            .add_gate_not("nxay", "xay")
            .add_gate_and("xor", "xoy", "nxay")
            .build();

        assert!(circuit.compute_signals().is_ok());
        assert_eq!(circuit.signal_from("xor"), Signal::Value(0xaa03));
    }
}
