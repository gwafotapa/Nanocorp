use std::collections::HashMap;

use crate::component::{Component, ComponentId, ComponentKind, WireSource};

pub struct Circuit {
    // pub wires: Vec<Wire>,
    components: HashMap<ComponentId, Component>,
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            // wires: vec![],
            // map: HashMap::new(),
            components: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, component: Component) -> bool {
        if self.components.contains_key(&component.id) {
            false
        } else {
            self.components.insert(component.id.clone(), component);
            true
        }
    }

    pub fn remove_component(&mut self, id: &str) {
        self.components.remove(id);
    }

    pub fn add_wire(&mut self, id: impl Into<String>, source: WireSource) -> bool {
        let wire = Component::new_wire(id, source);
        self.add_component(wire)
    }

    pub fn add_wire_with_value(&mut self, id: impl Into<String>, value: u16) -> bool {
        let wire = Component::new_wire_with_value(id, value);
        self.add_component(wire)
    }

    pub fn add_wire_from_component(
        &mut self,
        id: impl Into<String>,
        component_id: impl Into<String>,
    ) -> bool {
        let wire = Component::new_wire_from_component(id, component_id);
        self.add_component(wire)
    }

    pub fn add_gate_and(
        &mut self,
        id: impl Into<String>,
        source1: impl Into<String>,
        source2: impl Into<String>,
    ) -> bool {
        let gate = Component::new_gate_and(id, source1, source2);
        self.add_component(gate)
    }

    pub fn add_wired_gate_and(
        &mut self,
        id: impl Into<String>,
        source1: impl Into<String>,
        source2: impl Into<String>,
    ) -> bool {
        let id = id.into();
        let uid = &id.to_ascii_uppercase();
        if !self.add_gate_and(uid, source1, source2) {
            return false;
        }
        if !self.add_wire_from_component(id, uid) {
            self.remove_component(&uid);
            return false;
        }
        true
    }

    pub fn add_gate_or(
        &mut self,
        id: impl Into<String>,
        source1: impl Into<String>,
        source2: impl Into<String>,
    ) -> bool {
        let gate = Component::new_gate_or(id, source1, source2);
        self.add_component(gate)
    }

    pub fn add_wired_gate_or(
        &mut self,
        id: impl Into<String>,
        source1: impl Into<String>,
        source2: impl Into<String>,
    ) -> bool {
        let id = id.into();
        let uid = &id.to_ascii_uppercase();
        if !self.add_gate_or(uid, source1, source2) {
            return false;
        }
        if !self.add_wire_from_component(id, uid) {
            self.remove_component(&uid);
            return false;
        }
        true
    }

    pub fn add_gate_sll(
        &mut self,
        id: impl Into<String>,
        source: impl Into<String>,
        shift: u8,
    ) -> bool {
        let gate = Component::new_gate_sll(id, source, shift);
        self.add_component(gate)
    }

    pub fn add_wired_gate_sll(
        &mut self,
        id: impl Into<String>,
        source: impl Into<String>,
        shift: u8,
    ) -> bool {
        let id = id.into();
        let uid = &id.to_ascii_uppercase();
        if !self.add_gate_sll(uid, source, shift) {
            return false;
        }
        if !self.add_wire_from_component(id, uid) {
            self.remove_component(&uid);
            return false;
        }
        true
    }

    pub fn add_gate_slr(
        &mut self,
        id: impl Into<String>,
        source: impl Into<String>,
        shift: u8,
    ) -> bool {
        let gate = Component::new_gate_slr(id, source, shift);
        self.add_component(gate)
    }

    pub fn add_wired_gate_slr(
        &mut self,
        id: impl Into<String>,
        source: impl Into<String>,
        shift: u8,
    ) -> bool {
        let id = id.into();
        let uid = &id.to_ascii_uppercase();
        if !self.add_gate_slr(uid, source, shift) {
            return false;
        }
        if !self.add_wire_from_component(id, uid) {
            self.remove_component(&uid);
            return false;
        }
        true
    }

    pub fn add_gate_not(&mut self, id: impl Into<String>, source: impl Into<String>) -> bool {
        let gate = Component::new_gate_not(id, source);
        self.add_component(gate)
    }

    pub fn add_wired_gate_not(&mut self, id: impl Into<String>, source: impl Into<String>) -> bool {
        let id = id.into();
        let uid = &id.to_ascii_uppercase();
        if !self.add_gate_not(uid, source) {
            return false;
        }
        if !self.add_wire_from_component(id, uid) {
            self.remove_component(&uid);
            return false;
        }
        true
    }

    // TODO: rework
    pub fn compute_signals(&mut self) -> bool {
        let mut ids: Vec<String> = self.components.keys().map(|id| id.into()).collect();
        while let Some(id) = ids.last() {
            if let Some(component) = self.components.get(id) {
                match &component.kind {
                    ComponentKind::Wire { source } => {
                        match source {
                            WireSource::Value(value) => {
                                self.components.get_mut(id).unwrap().signal = Some(*value); // TODO: add fn
                                ids.pop();
                            }
                            WireSource::Id(other) => {
                                if let Some(signal) = self.get_signal(other) {
                                    self.components.get_mut(id).unwrap().signal = Some(signal);
                                    ids.pop();
                                } else {
                                    ids.push(other.to_string());
                                }
                            }
                        }
                    }
                    ComponentKind::GateAnd { source1, source2 } => {
                        if let Some(signal1) = self.get_signal(source1) {
                            if let Some(signal2) = self.get_signal(source2) {
                                self.components.get_mut(id).unwrap().signal =
                                    Some(signal1 & signal2);
                                ids.pop();
                            } else {
                                ids.push(source2.to_string());
                            }
                        } else {
                            ids.push(source1.to_string());
                            if self.get_signal(source2).is_none() {
                                ids.push(source2.to_string());
                            }
                        }
                    }
                    ComponentKind::GateOr { source1, source2 } => {
                        if let Some(signal1) = self.get_signal(source1) {
                            if let Some(signal2) = self.get_signal(source2) {
                                self.components.get_mut(id).unwrap().signal =
                                    Some(signal1 | signal2);
                                ids.pop();
                            } else {
                                ids.push(source2.to_string());
                            }
                        } else {
                            ids.push(source1.to_string());
                            if self.get_signal(source2).is_none() {
                                ids.push(source2.to_string());
                            }
                        }
                    }
                    ComponentKind::GateSLL { source, shift } => {
                        if let Some(signal) = self.get_signal(source) {
                            self.components.get_mut(id).unwrap().signal = Some(signal << shift);
                            ids.pop();
                        } else {
                            ids.push(source.to_string());
                        }
                    }
                    ComponentKind::GateSLR { source, shift } => {
                        if let Some(signal) = self.get_signal(source) {
                            self.components.get_mut(id).unwrap().signal = Some(signal >> shift);
                            ids.pop();
                        } else {
                            ids.push(source.to_string());
                        }
                    }
                    ComponentKind::GateNot { source } => {
                        if let Some(signal) = self.get_signal(source) {
                            self.components.get_mut(id).unwrap().signal = Some(!signal);
                            ids.pop();
                        } else {
                            ids.push(source.to_string());
                        }
                    }
                }
            }
        }

        true
    }

    // fn compute_signal(&mut self, id: impl AsRef<str>) -> bool {
    //     if let Some(wire) = self.wires.get_mut(id.as_ref()) {
    //         if let Some(source) = &wire.source {
    //             match source {
    //                 Source::Value(value) => {
    //                     wire.signal = Some(*value);
    //                     true
    //                 }
    //                 Source::Wire(wire_id) => {
    //                     if self.compute_signal(wire_id) {
    //                         wire.signal = self.get_signal(<String as AsRef<str>>::as_ref(wire_id));
    //                         true
    //                     } else {
    //                         false
    //                     }
    //                 }
    //                 Source::Gate(gate) => true,
    //             }
    //         } else {
    //             false
    //         }
    //     } else {
    //         false
    //     }
    // }

    fn get_signal(&self, id: impl AsRef<str>) -> Option<u16> {
        self.components.get(id.as_ref()).and_then(|w| w.signal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_circuit() {
        let w1 = Component::new_wire_with_value("a", 1);
        let w2 = Component::new_wire_from_component("b", "a");
        // let w2_ = Component::new_wire_from_component("b", "a");

        let mut circuit = Circuit::new();
        assert!(circuit.add_component(w1));
        assert!(circuit.add_component(w2));
        // assert!(!circuit.add_component(w2_));

        assert_eq!(circuit.get_signal("z"), None);
        assert_eq!(circuit.get_signal("a"), None);
        assert_eq!(circuit.get_signal("b"), None);
        circuit.compute_signals();
        assert_eq!(circuit.get_signal("a"), Some(1));
        assert_eq!(circuit.get_signal("b"), Some(1));

        let g = Component::new_gate_not("c", "b");
        circuit.add_component(g);
        assert_eq!(circuit.get_signal("c"), None);
        circuit.compute_signals();
        assert_eq!(circuit.get_signal("c"), Some(0xfffe));
    }

    #[test]
    fn instructions_example() {
        let x = Component::new_wire_with_value("x", 123);
        let y = Component::new_wire_with_value("y", 456);
        let gd = Component::new_gate_and("D", "x", "y");
        let ge = Component::new_gate_or("E", "x", "y");
        let gf = Component::new_gate_sll("F", "x", 2);
        let gg = Component::new_gate_slr("G", "y", 2);
        let gh = Component::new_gate_not("H", "x");
        let gi = Component::new_gate_not("I", "y");
        let d = Component::new_wire_from_component("d", "D");
        let e = Component::new_wire_from_component("e", "E");
        let f = Component::new_wire_from_component("f", "F");
        let g = Component::new_wire_from_component("g", "G");
        let h = Component::new_wire_from_component("h", "H");
        let i = Component::new_wire_from_component("i", "I");

        let mut circuit = Circuit::new();
        circuit.add_component(x);
        circuit.add_component(y);
        circuit.add_component(gd);
        circuit.add_component(ge);
        circuit.add_component(gf);
        circuit.add_component(gg);
        circuit.add_component(gh);
        circuit.add_component(gi);
        circuit.add_component(d);
        circuit.add_component(e);
        circuit.add_component(f);
        circuit.add_component(g);
        circuit.add_component(h);
        circuit.add_component(i);

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
    fn instructions_example_2() {
        let mut circuit = Circuit::new();
        circuit.add_wire_with_value("x", 123);
        circuit.add_wire_with_value("y", 456);
        circuit.add_wired_gate_and("d", "x", "y");
        circuit.add_wired_gate_or("e", "x", "y");
        circuit.add_wired_gate_sll("f", "x", 2);
        circuit.add_wired_gate_slr("g", "y", 2);
        circuit.add_wired_gate_not("h", "x");
        circuit.add_wired_gate_not("i", "y");
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
