use std::collections::HashMap;

use crate::{
    gate::Gate,
    wire::{Source, Wire, WireId},
};

pub struct Circuit {
    // pub wires: Vec<Wire>,
    wires: HashMap<WireId, Wire>,
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            // wires: vec![],
            // map: HashMap::new(),
            wires: HashMap::new(),
        }
    }

    pub fn add_wire(&mut self, wire: Wire) -> bool {
        if self.wires.contains_key(&wire.id) {
            false
        } else {
            self.wires.insert(wire.id.clone(), wire);
            true
        }
    }

    // fn get_wire(&self, id: impl Into<String>) -> Option<&Wire> {
    //     let id = id.into();
    //     for wire in &self.wires {
    //         if wire.id == id {
    //             return Some(wire);
    //         }
    //     }
    //     None
    // }

    // TODO: rework
    pub fn compute_signals(&mut self) -> bool {
        let mut ids: Vec<String> = self.wires.keys().map(|id| id.into()).collect();
        while let Some(id) = ids.last() {
            if let Some(wire) = self.wires.get(id) {
                if let Some(source) = &wire.source {
                    match source {
                        Source::Value(value) => {
                            self.wires.get_mut(id).unwrap().signal = Some(*value); // TODO: add fn
                            ids.pop();
                        }
                        Source::Wire(wire_id) => {
                            if let Some(signal) = self.get_signal(wire_id) {
                                self.wires.get_mut(id).unwrap().signal = Some(signal);
                                ids.pop();
                            } else {
                                ids.push(wire_id.to_string());
                            }
                        }
                        Source::Gate(gate) => match gate {
                            Gate::And { wire1, wire2 } => {
                                if let Some(signal1) = self.get_signal(wire1) {
                                    if let Some(signal2) = self.get_signal(wire2) {
                                        self.wires.get_mut(id).unwrap().signal =
                                            Some(signal1 & signal2);
                                        ids.pop();
                                    } else {
                                        ids.push(wire2.to_string());
                                    }
                                } else {
                                    ids.push(wire1.to_string());
                                    if self.get_signal(wire2).is_none() {
                                        ids.push(wire2.to_string());
                                    }
                                }
                            }
                            Gate::Or { wire1, wire2 } => {
                                if let Some(signal1) = self.get_signal(wire1) {
                                    if let Some(signal2) = self.get_signal(wire2) {
                                        self.wires.get_mut(id).unwrap().signal =
                                            Some(signal1 | signal2);
                                        ids.pop();
                                    } else {
                                        ids.push(wire2.to_string());
                                    }
                                } else {
                                    ids.push(wire1.to_string());
                                    if self.get_signal(wire2).is_none() {
                                        ids.push(wire2.to_string());
                                    }
                                }
                            }
                            Gate::Not { wire } => {
                                if let Some(signal) = self.get_signal(wire) {
                                    self.wires.get_mut(id).unwrap().signal = Some(!signal);
                                    ids.pop();
                                } else {
                                    ids.push(wire.to_string());
                                }
                            }
                            Gate::SLL { wire, shift } => {
                                if let Some(signal) = self.get_signal(wire) {
                                    self.wires.get_mut(id).unwrap().signal = Some(signal << shift);
                                    ids.pop();
                                } else {
                                    ids.push(wire.to_string());
                                }
                            }
                            Gate::SLR { wire, shift } => {
                                if let Some(signal) = self.get_signal(wire) {
                                    self.wires.get_mut(id).unwrap().signal = Some(signal >> shift);
                                    ids.pop();
                                } else {
                                    ids.push(wire.to_string());
                                }
                            }
                        },
                    }
                } else {
                    return false;
                }
            } else {
                return false;
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
        self.wires.get(id.as_ref()).and_then(|w| w.signal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_circuit() {
        let w1 = Wire::source_value("a", 1).unwrap();
        let w2 = Wire::source_other_wire("b", "a").unwrap();
        let w2_ = Wire::source_other_wire("b", "a").unwrap();

        let mut circuit = Circuit::new();
        assert!(circuit.add_wire(w1));
        assert!(circuit.add_wire(w2));
        assert!(!circuit.add_wire(w2_));

        assert_eq!(circuit.get_signal("z"), None);
        assert_eq!(circuit.get_signal("a"), None);
        assert_eq!(circuit.get_signal("b"), None);
        circuit.compute_signals();
        assert_eq!(circuit.get_signal("a"), Some(1));
        assert_eq!(circuit.get_signal("b"), Some(1));

        let w3 = Wire::source_gate("c", Gate::not("b").unwrap()).unwrap();
        circuit.add_wire(w3);
        assert_eq!(circuit.get_signal("c"), None);
        circuit.compute_signals();
        assert_eq!(circuit.get_signal("c"), Some(0xfffe));
    }

    #[test]
    fn instructions_example() {
        let w1 = Wire::source_value("x", 123).unwrap();
        let w2 = Wire::source_value("y", 456).unwrap();
        let w3 = Wire::source_gate("d", Gate::and("x", "y").unwrap()).unwrap();
        let w4 = Wire::source_gate("e", Gate::or("x", "y").unwrap()).unwrap();
        let w5 = Wire::source_gate("f", Gate::sll("x", 2).unwrap()).unwrap();
        let w6 = Wire::source_gate("g", Gate::slr("y", 2).unwrap()).unwrap();
        let w7 = Wire::source_gate("h", Gate::not("x").unwrap()).unwrap();
        let w8 = Wire::source_gate("i", Gate::not("y").unwrap()).unwrap();

        let mut circuit = Circuit::new();
        circuit.add_wire(w1);
        circuit.add_wire(w2);
        circuit.add_wire(w3);
        circuit.add_wire(w4);
        circuit.add_wire(w5);
        circuit.add_wire(w6);
        circuit.add_wire(w7);
        circuit.add_wire(w8);
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
