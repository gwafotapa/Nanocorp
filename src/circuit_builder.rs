use std::{collections::HashMap, mem};

use crate::{
    circuit::Circuit,
    gate::Gate,
    wire::{Wire, WireId, WireInput},
};

pub struct CircuitBuilder {
    wires: HashMap<WireId, Wire>,
}

impl CircuitBuilder {
    pub fn new() -> Self {
        Self {
            wires: HashMap::new(),
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

    pub fn add_wire_with_input(
        &mut self,
        id: impl Into<String>,
        input: WireInput,
    ) -> &mut CircuitBuilder {
        let wire = Wire::new(id, input).unwrap();
        self.add(wire);
        self
    }

    pub fn add_wire_with_value(
        &mut self,
        id: impl Into<String>,
        value: u16,
    ) -> &mut CircuitBuilder {
        let wire = Wire::with_value(id, value).unwrap();
        self.add(wire);
        self
    }

    pub fn add_wire_from_wire(
        &mut self,
        id: impl Into<String>,
        input_id: impl Into<String>,
    ) -> &mut CircuitBuilder {
        let wire = Wire::from_wire(id, input_id).unwrap();
        self.add(wire);
        self
    }

    pub fn add_wire_from_gate(&mut self, id: impl Into<String>, gate: Gate) -> &mut CircuitBuilder {
        let wire = Wire::from_gate(id, gate).unwrap();
        self.add(wire);
        self
    }

    pub fn add_gate_and(
        &mut self,
        output: impl Into<String>,
        input1: impl Into<String>,
        input2: impl Into<String>,
    ) -> &mut CircuitBuilder {
        let wire = Wire::from_gate_and(output, input1, input2).unwrap();
        self.add(wire);
        self
    }

    pub fn add_gate_and_value(
        &mut self,
        output: impl Into<String>,
        input1: impl Into<String>,
        input2: u16,
    ) -> &mut CircuitBuilder {
        let wire = Wire::from_gate_and_value(output, input1, input2).unwrap();
        self.add(wire);
        self
    }

    pub fn add_gate_or(
        &mut self,
        output: impl Into<String>,
        input1: impl Into<String>,
        input2: impl Into<String>,
    ) -> &mut CircuitBuilder {
        let wire = Wire::from_gate_or(output, input1, input2).unwrap();
        self.add(wire);
        self
    }

    pub fn add_gate_or_value(
        &mut self,
        output: impl Into<String>,
        input1: impl Into<String>,
        input2: u16,
    ) -> &mut CircuitBuilder {
        let wire = Wire::from_gate_or_value(output, input1, input2).unwrap();
        self.add(wire);
        self
    }

    pub fn add_gate_sll(
        &mut self,
        output: impl Into<String>,
        input: impl Into<String>,
        shift: u8,
    ) -> &mut CircuitBuilder {
        let wire = Wire::from_gate_sll(output, input, shift).unwrap();
        self.add(wire);
        self
    }

    pub fn add_gate_slr(
        &mut self,
        output: impl Into<String>,
        input: impl Into<String>,
        shift: u8,
    ) -> &mut CircuitBuilder {
        let wire = Wire::from_gate_slr(output, input, shift).unwrap();
        self.add(wire);
        self
    }

    pub fn add_gate_not(
        &mut self,
        output: impl Into<String>,
        input: impl Into<String>,
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

        assert_eq!(circuit.get_signal("d"), Some(72));
        assert_eq!(circuit.get_signal("e"), Some(507));
        assert_eq!(circuit.get_signal("f"), Some(492));
        assert_eq!(circuit.get_signal("g"), Some(114));
        assert_eq!(circuit.get_signal("h"), Some(65412));
        assert_eq!(circuit.get_signal("i"), Some(65079));
        assert_eq!(circuit.get_signal("x"), Some(123));
        assert_eq!(circuit.get_signal("y"), Some(456));
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

        assert_eq!(circuit.get_signal("d"), Some(72));
        assert_eq!(circuit.get_signal("e"), Some(507));
        assert_eq!(circuit.get_signal("f"), Some(492));
        assert_eq!(circuit.get_signal("g"), Some(114));
        assert_eq!(circuit.get_signal("h"), Some(65412));
        assert_eq!(circuit.get_signal("i"), Some(65079));
        assert_eq!(circuit.get_signal("x"), Some(123));
        assert_eq!(circuit.get_signal("y"), Some(456));
    }
}
