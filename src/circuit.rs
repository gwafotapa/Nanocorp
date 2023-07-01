use crate::{
    gate::Gate,
    wire::{Source, Wire},
};

pub struct Circuit<'a> {
    // wires: Vec<&'a Wire<'a>>,
    pub wires: Vec<&'a mut Wire<'a>>,
    pub gates: Vec<&'a mut Gate<'a>>,
}

impl<'a> Circuit<'a> {
    pub fn new() -> Self {
        Self {
            wires: vec![],
            gates: vec![],
        }
    }

    // fn add_wire(&mut self, wire: &'a Wire<'a>) {
    //     self.wires.push(wire);
    // }
    pub fn add_wire(&mut self, wire: &'a mut Wire<'a>) -> bool {
        if self.wires.contains(&wire) {
            false
        } else if let Some(source) = wire.source.as_ref() {
            match source {
                Source::Value(_) => {
                    self.wires.push(wire);
                    true
                }
                Source::Wire(source_wire) => {
                    if self.wires.iter().any(|w| w == source_wire) {
                        false
                    } else {
                        self.wires.push(wire);
                        true
                    }
                } // Source::Gate(gate) => {
                  //     if self.gates.contains(gate) {
                  //         self.wires.push(gate);
                  //         true
                  //     } else {
                  //         false
                  //     }
                  // }
            }
        } else {
            false
        }
    }

    pub fn add_gate(&mut self, gate: &'a mut Gate<'a>) {
        self.gates.push(gate);
    }

    pub fn sort_components(&mut self) {}

    pub fn compute_signals(&mut self) {
        for wire in &mut self.wires {
            wire.set_signal();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_circuit() {
        let w1 = Wire::source_value("a", 1).unwrap();
        let w2 = Wire::source_wire("b", &w1).unwrap();
        let not_w2 = Gate::not(&w2);

        let mut circuit = Circuit::new();
        circuit.add_wire(&w1);
        assert!(circuit.add_wire(&w2));
        assert!(!circuit.add_wire(&w2));
        circuit.add_gate(&not_w2);
    }
}
