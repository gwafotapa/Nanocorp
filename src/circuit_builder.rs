use std::{collections::HashMap, mem};

use crate::{
    circuit::Circuit,
    component::{Component, ComponentId, WireSource},
};

pub struct CircuitBuilder {
    components: HashMap<ComponentId, Component>,
}

impl CircuitBuilder {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn build(&mut self) -> Circuit {
        let mut circuit = Circuit::new();
        mem::swap(&mut circuit.components, &mut self.components);
        circuit
    }

    pub fn add_component(&mut self, component: Component) -> &mut CircuitBuilder {
        self.components.insert(component.id.clone(), component);
        self
    }

    pub fn add_wire(&mut self, id: impl Into<String>, source: WireSource) -> &mut CircuitBuilder {
        let wire = Component::new_wire(id, source).unwrap();
        self.add_component(wire);
        self
    }

    pub fn add_wire_with_value(
        &mut self,
        id: impl Into<String>,
        value: u16,
    ) -> &mut CircuitBuilder {
        let wire = Component::new_wire_with_value(id, value).unwrap();
        self.add_component(wire);
        self
    }

    pub fn add_wire_from_component(
        &mut self,
        id: impl Into<String>,
        component_id: impl Into<String>,
    ) -> &mut CircuitBuilder {
        let wire = Component::new_wire_from_component(id, component_id).unwrap();
        self.add_component(wire);
        self
    }

    pub fn add_gate_and(
        &mut self,
        id: impl Into<String>,
        source1: impl Into<String>,
        source2: impl Into<String>,
    ) -> &mut CircuitBuilder {
        let gate = Component::new_gate_and(id, source1, source2).unwrap();
        self.add_component(gate);
        self
    }

    pub fn add_wired_gate_and(
        &mut self,
        id: impl Into<String>,
        source1: impl Into<String>,
        source2: impl Into<String>,
    ) -> &mut CircuitBuilder {
        let id = id.into();
        let uid = &id.to_ascii_uppercase();
        self.add_gate_and(uid, source1, source2);
        self.add_wire_from_component(id, uid);
        self
    }

    pub fn add_gate_or(
        &mut self,
        id: impl Into<String>,
        source1: impl Into<String>,
        source2: impl Into<String>,
    ) -> &mut CircuitBuilder {
        let gate = Component::new_gate_or(id, source1, source2).unwrap();
        self.add_component(gate);
        self
    }

    pub fn add_wired_gate_or(
        &mut self,
        id: impl Into<String>,
        source1: impl Into<String>,
        source2: impl Into<String>,
    ) -> &mut CircuitBuilder {
        let id = id.into();
        let uid = &id.to_ascii_uppercase();
        self.add_gate_or(uid, source1, source2);
        self.add_wire_from_component(id, uid);
        self
    }

    pub fn add_gate_sll(
        &mut self,
        id: impl Into<String>,
        source: impl Into<String>,
        shift: u8,
    ) -> &mut CircuitBuilder {
        let gate = Component::new_gate_sll(id, source, shift).unwrap();
        self.add_component(gate);
        self
    }

    pub fn add_wired_gate_sll(
        &mut self,
        id: impl Into<String>,
        source: impl Into<String>,
        shift: u8,
    ) -> &mut CircuitBuilder {
        let id = id.into();
        let uid = &id.to_ascii_uppercase();
        self.add_gate_sll(uid, source, shift);
        self.add_wire_from_component(id, uid);
        self
    }

    pub fn add_gate_slr(
        &mut self,
        id: impl Into<String>,
        source: impl Into<String>,
        shift: u8,
    ) -> &mut CircuitBuilder {
        let gate = Component::new_gate_slr(id, source, shift).unwrap();
        self.add_component(gate);
        self
    }

    pub fn add_wired_gate_slr(
        &mut self,
        id: impl Into<String>,
        source: impl Into<String>,
        shift: u8,
    ) -> &mut CircuitBuilder {
        let id = id.into();
        let uid = &id.to_ascii_uppercase();
        self.add_gate_slr(uid, source, shift);
        self.add_wire_from_component(id, uid);
        self
    }

    pub fn add_gate_not(
        &mut self,
        id: impl Into<String>,
        source: impl Into<String>,
    ) -> &mut CircuitBuilder {
        let gate = Component::new_gate_not(id, source).unwrap();
        self.add_component(gate);
        self
    }

    pub fn add_wired_gate_not(
        &mut self,
        id: impl Into<String>,
        source: impl Into<String>,
    ) -> &mut CircuitBuilder {
        let id = id.into();
        let uid = &id.to_ascii_uppercase();
        self.add_gate_not(uid, source);
        self.add_wire_from_component(id, uid);
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
            .add_wired_gate_and("d", "x", "y")
            .add_wired_gate_or("e", "x", "y")
            .add_wired_gate_sll("f", "x", 2)
            .add_wired_gate_slr("g", "y", 2)
            .add_wired_gate_not("h", "x")
            .add_wired_gate_not("i", "y")
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
        builder.add_wired_gate_and("d", "x", "y");
        builder.add_wired_gate_or("e", "x", "y");
        builder.add_wired_gate_sll("f", "x", 2);
        builder.add_wired_gate_slr("g", "y", 2);
        builder.add_wired_gate_not("h", "x");
        builder.add_wired_gate_not("i", "y");
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
