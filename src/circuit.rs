use std::{collections::HashMap, fmt};

use crate::{
    error::Error,
    gate::Gate,
    wire::{Wire, WireId, WireInput},
};

pub struct Circuit {
    pub wires: HashMap<WireId, Wire>,
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            wires: HashMap::new(),
        }
    }

    pub fn remove(&mut self, id: &str) {
        self.wires.remove(id);
    }

    pub fn add(&mut self, wire: Wire) -> Result<(), Error> {
        if self.wires.contains_key(&wire.id) {
            Err(Error::IdAlreadyExists(wire.id))
        } else {
            self.wires.insert(wire.id.clone(), wire);
            Ok(())
        }
    }

    pub fn add_wire_with_input(
        &mut self,
        id: impl Into<String>,
        input: WireInput,
    ) -> Result<(), Error> {
        let wire = Wire::new(id, input)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_wire_with_value(&mut self, id: impl Into<String>, value: u16) -> Result<(), Error> {
        let wire = Wire::with_value(id, value)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_wire_from_wire(
        &mut self,
        id: impl Into<String>,
        input_id: impl Into<String>,
    ) -> Result<(), Error> {
        let wire = Wire::from_wire(id, input_id)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_wire_from_gate(&mut self, id: impl Into<String>, gate: Gate) -> Result<(), Error> {
        let wire = Wire::from_gate(id, gate)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_gate_and(
        &mut self,
        output: impl Into<String>,
        input1: impl Into<String>,
        input2: impl Into<String>,
    ) -> Result<(), Error> {
        let gate = Gate::and(input1, input2)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_gate_or(
        &mut self,
        output: impl Into<String>,
        input1: impl Into<String>,
        input2: impl Into<String>,
    ) -> Result<(), Error> {
        let gate = Gate::or(input1, input2)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_gate_sll(
        &mut self,
        output: impl Into<String>,
        input: impl Into<String>,
        shift: u8,
    ) -> Result<(), Error> {
        let gate = Gate::sll(input, shift)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_gate_slr(
        &mut self,
        output: impl Into<String>,
        input: impl Into<String>,
        shift: u8,
    ) -> Result<(), Error> {
        let gate = Gate::slr(input, shift)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_gate_not(
        &mut self,
        output: impl Into<String>,
        input: impl Into<String>,
    ) -> Result<(), Error> {
        let gate = Gate::not(input)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)?;
        Ok(())
    }

    // TODO: rework
    // TODO: check for loops
    // TODO: add result return type
    pub fn compute_signals(&mut self) -> bool {
        let mut ids: Vec<String> = self.wires.keys().map(|id| id.into()).collect();
        while let Some(id) = ids.last() {
            if let Some(wire) = self.wires.get(id) {
                match &wire.input {
                    WireInput::Value(value) => {
                        self.wires.get_mut(id).unwrap().signal = Some(*value); // TODO: add fn
                        ids.pop();
                    }
                    WireInput::Wire(input_id) => {
                        if let Some(signal) = self.get_signal(input_id) {
                            self.wires.get_mut(id).unwrap().signal = Some(signal);
                            ids.pop();
                        } else {
                            ids.push(input_id.to_string());
                        }
                    }
                    WireInput::Gate(gate) => match gate {
                        Gate::And { input1, input2 } => {
                            if let Some(signal1) = self.get_signal(input1) {
                                if let Some(signal2) = self.get_signal(input2) {
                                    self.wires.get_mut(id).unwrap().signal =
                                        Some(signal1 & signal2);
                                    ids.pop();
                                } else {
                                    ids.push(input2.to_string());
                                }
                            } else {
                                ids.push(input1.to_string());
                                if self.get_signal(input2).is_none() {
                                    ids.push(input2.to_string());
                                }
                            }
                        }
                        Gate::Or { input1, input2 } => {
                            if let Some(signal1) = self.get_signal(input1) {
                                if let Some(signal2) = self.get_signal(input2) {
                                    self.wires.get_mut(id).unwrap().signal =
                                        Some(signal1 | signal2);
                                    ids.pop();
                                } else {
                                    ids.push(input2.to_string());
                                }
                            } else {
                                ids.push(input1.to_string());
                                if self.get_signal(input2).is_none() {
                                    ids.push(input2.to_string());
                                }
                            }
                        }
                        Gate::SLL { input, shift } => {
                            if let Some(signal) = self.get_signal(input) {
                                self.wires.get_mut(id).unwrap().signal = Some(signal << shift);
                                ids.pop();
                            } else {
                                ids.push(input.to_string());
                            }
                        }
                        Gate::SLR { input, shift } => {
                            if let Some(signal) = self.get_signal(input) {
                                self.wires.get_mut(id).unwrap().signal = Some(signal >> shift);
                                ids.pop();
                            } else {
                                ids.push(input.to_string());
                            }
                        }
                        Gate::Not { input } => {
                            if let Some(signal) = self.get_signal(input) {
                                self.wires.get_mut(id).unwrap().signal = Some(!signal);
                                ids.pop();
                            } else {
                                ids.push(input.to_string());
                            }
                        }
                    },
                }
            } else {
                // Unkwown wire id
                break;
            }
        }

        true
    }

    pub fn get_signal(&self, id: &str) -> Option<u16> {
        self.wires.get(id).and_then(|w| w.signal)
    }
}

impl fmt::Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for wire in self.wires.values() {
            write!(f, "{}\n", wire)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_circuit() {
        let w1 = Wire::with_value("a", 1).unwrap();
        let w2 = Wire::from_wire("b", "a").unwrap();
        // let w2_ = Wire::new_wire_from_component("b", "a");

        let mut circuit = Circuit::new();
        assert!(circuit.add(w1).is_ok());
        assert!(circuit.add(w2).is_ok());
        // assert!(!circuit.add(w2_));

        assert_eq!(circuit.get_signal("z"), None);
        assert_eq!(circuit.get_signal("a"), None);
        assert_eq!(circuit.get_signal("b"), None);
        circuit.compute_signals();
        assert_eq!(circuit.get_signal("a"), Some(1));
        assert_eq!(circuit.get_signal("b"), Some(1));

        let g = Gate::not("b").unwrap();
        let c = Wire::from_gate("c", g).unwrap();
        let _ = circuit.add(c);
        assert_eq!(circuit.get_signal("c"), None);
        circuit.compute_signals();
        assert_eq!(circuit.get_signal("c"), Some(0xfffe));
        println!("{}", circuit);
    }

    #[test]
    fn instructions_example() {
        let x = Wire::with_value("x", 123).unwrap();
        let y = Wire::with_value("y", 456).unwrap();
        let gd = Gate::and("x", "y").unwrap();
        let ge = Gate::or("x", "y").unwrap();
        let gf = Gate::sll("x", 2).unwrap();
        let gg = Gate::slr("y", 2).unwrap();
        let gh = Gate::not("x").unwrap();
        let gi = Gate::not("y").unwrap();
        let d = Wire::from_gate("d", gd).unwrap();
        let e = Wire::from_gate("e", ge).unwrap();
        let f = Wire::from_gate("f", gf).unwrap();
        let g = Wire::from_gate("g", gg).unwrap();
        let h = Wire::from_gate("h", gh).unwrap();
        let i = Wire::from_gate("i", gi).unwrap();

        let mut circuit = Circuit::new();
        circuit.add(x).unwrap();
        circuit.add(y).unwrap();
        circuit.add(d).unwrap();
        circuit.add(e).unwrap();
        circuit.add(f).unwrap();
        circuit.add(g).unwrap();
        circuit.add(h).unwrap();
        circuit.add(i).unwrap();

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
        circuit.add_wire_with_value("x", 123).unwrap();
        circuit.add_wire_with_value("y", 456).unwrap();
        circuit.add_gate_and("d", "x", "y").unwrap();
        circuit.add_gate_or("e", "x", "y").unwrap();
        circuit.add_gate_sll("f", "x", 2).unwrap();
        circuit.add_gate_slr("g", "y", 2).unwrap();
        circuit.add_gate_not("h", "x").unwrap();
        circuit.add_gate_not("i", "y").unwrap();
        circuit.compute_signals();

        println!("{}", circuit);

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
