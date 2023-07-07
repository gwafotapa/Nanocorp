use std::{collections, mem};

use crate::{
    circuit::Circuit,
    gate::Gate,
    wire::{Wire, WireInput},
    wire_id::WireId,
};

pub struct CircuitBuilder {
    wires: collections::HashMap<WireId, Wire>,
}

impl CircuitBuilder {
    pub fn new() -> Self {
        Self {
            wires: collections::HashMap::new(),
        }
    }

    pub fn build(&mut self) -> Circuit {
        let mut circuit = Circuit::new();
        mem::swap(&mut circuit.wires, &mut self.wires);
        circuit
    }

    pub fn add(&mut self, wire: Wire) -> &mut CircuitBuilder {
        self.wires.insert(wire.id.clone(), wire);
        self
    }

    pub fn add_wire_with_input<S: Into<String>>(
        &mut self,
        id: S,
        input: WireInput,
    ) -> &mut CircuitBuilder {
        let wire = Wire::new(id, input).unwrap();
        self.add(wire);
        self
    }

    pub fn add_wire_with_value<S: Into<String>>(
        &mut self,
        id: S,
        value: u16,
    ) -> &mut CircuitBuilder {
        let wire = Wire::with_value(id, value).unwrap();
        self.add(wire);
        self
    }

    pub fn add_wire_from_wire<S: Into<String>, T: Into<String>>(
        &mut self,
        id: S,
        input_id: T,
    ) -> &mut CircuitBuilder {
        let wire = Wire::from_wire(id, input_id).unwrap();
        self.add(wire);
        self
    }

    pub fn add_wire_from_gate<S: Into<String>>(
        &mut self,
        id: S,
        gate: Gate,
    ) -> &mut CircuitBuilder {
        let wire = Wire::from_gate(id, gate).unwrap();
        self.add(wire);
        self
    }

    pub fn add_gate_and<S: Into<String>, T: Into<String>, U: Into<String>>(
        &mut self,
        output: S,
        input1: T,
        input2: U,
    ) -> &mut CircuitBuilder {
        let wire = Wire::from_gate_and(output, input1, input2).unwrap();
        self.add(wire);
        self
    }

    pub fn add_gate_and_value<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        value: u16,
    ) -> &mut CircuitBuilder {
        let wire = Wire::from_gate_and_value(output, input, value).unwrap();
        self.add(wire);
        self
    }

    pub fn add_gate_or<S: Into<String>, T: Into<String>, U: Into<String>>(
        &mut self,
        output: S,
        input1: T,
        input2: U,
    ) -> &mut CircuitBuilder {
        let wire = Wire::from_gate_or(output, input1, input2).unwrap();
        self.add(wire);
        self
    }

    pub fn add_gate_or_value<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        value: u16,
    ) -> &mut CircuitBuilder {
        let wire = Wire::from_gate_or_value(output, input, value).unwrap();
        self.add(wire);
        self
    }

    pub fn add_gate_sll<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        shift: u8,
    ) -> &mut CircuitBuilder {
        let wire = Wire::from_gate_sll(output, input, shift).unwrap();
        self.add(wire);
        self
    }

    pub fn add_gate_slr<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        shift: u8,
    ) -> &mut CircuitBuilder {
        let wire = Wire::from_gate_slr(output, input, shift).unwrap();
        self.add(wire);
        self
    }

    pub fn add_gate_not<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
    ) -> &mut CircuitBuilder {
        let gate = Wire::from_gate_not(output, input).unwrap();
        self.add(gate);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_liner() {
        let mut circuit = CircuitBuilder::new()
            .add_wire_with_value("x", 123)
            .add_wire_with_value("y", 456)
            .add_gate_and("d", "x", "y")
            .add_gate_or("e", "x", "y")
            .add_gate_sll("f", "x", 2)
            .add_gate_slr("g", "y", 2)
            .add_gate_not("h", "x")
            .add_gate_not("i", "y")
            .build();

        circuit.compute_signals();

        assert!(matches!(circuit.signal("d"), Ok(Some(72))));
        assert!(matches!(circuit.signal("e"), Ok(Some(507))));
        assert!(matches!(circuit.signal("f"), Ok(Some(492))));
        assert!(matches!(circuit.signal("g"), Ok(Some(114))));
        assert!(matches!(circuit.signal("h"), Ok(Some(65412))));
        assert!(matches!(circuit.signal("i"), Ok(Some(65079))));
        assert!(matches!(circuit.signal("x"), Ok(Some(123))));
        assert!(matches!(circuit.signal("y"), Ok(Some(456))));
    }

    #[test]
    fn complex_configuration() {
        let mut builder = CircuitBuilder::new();
        builder.add_wire_with_value("x", 123);
        builder.add_wire_with_value("y", 456);
        builder.add_gate_and("d", "x", "y");
        builder.add_gate_or("e", "x", "y");
        builder.add_gate_sll("f", "x", 2);
        builder.add_gate_slr("g", "y", 2);
        builder.add_gate_not("h", "x");
        builder.add_gate_not("i", "y");
        let mut circuit = builder.build();

        circuit.compute_signals();

        assert!(matches!(circuit.signal("d"), Ok(Some(72))));
        assert!(matches!(circuit.signal("e"), Ok(Some(507))));
        assert!(matches!(circuit.signal("f"), Ok(Some(492))));
        assert!(matches!(circuit.signal("g"), Ok(Some(114))));
        assert!(matches!(circuit.signal("h"), Ok(Some(65412))));
        assert!(matches!(circuit.signal("i"), Ok(Some(65079))));
        assert!(matches!(circuit.signal("x"), Ok(Some(123))));
        assert!(matches!(circuit.signal("y"), Ok(Some(456))));
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

        circuit.compute_signals();
        assert!(matches!(circuit.signal("xor"), Ok(Some(0xaa03))));
    }
}
