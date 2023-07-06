use std::{
    collections::HashMap,
    fmt,
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use crate::{
    error::{CircuitError, ParseCircuitError},
    gate::Gate,
    wire::{Wire, WireId, WireInput},
};

#[derive(Debug, PartialEq)]
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

    pub fn add(&mut self, wire: Wire) -> Result<(), CircuitError> {
        if self.wires.contains_key(&wire.id) {
            Err(CircuitError::IdAlreadyExists(wire.id))
        } else {
            self.wires.insert(wire.id.clone(), wire);
            Ok(())
        }
    }

    pub fn add_wire_with_input<S: Into<String>>(
        &mut self,
        id: S,
        input: WireInput,
    ) -> Result<(), CircuitError> {
        let wire = Wire::new(id, input)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_wire_with_value<S: Into<String>>(
        &mut self,
        id: S,
        value: u16,
    ) -> Result<(), CircuitError> {
        let wire = Wire::with_value(id, value)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_wire_from_wire<S: Into<String>, T: Into<String>>(
        &mut self,
        id: S,
        input_id: T,
    ) -> Result<(), CircuitError> {
        let wire = Wire::from_wire(id, input_id)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_wire_from_gate<S: Into<String>>(
        &mut self,
        id: S,
        gate: Gate,
    ) -> Result<(), CircuitError> {
        let wire = Wire::from_gate(id, gate)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_gate_and<S: Into<String>, T: Into<String>, U: Into<String>>(
        &mut self,
        output: S,
        input1: T,
        input2: U,
    ) -> Result<(), CircuitError> {
        let gate = Gate::and(input1, input2)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_gate_and_value<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        value: u16,
    ) -> Result<(), CircuitError> {
        let gate = Gate::and_value(input, value)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_gate_or<S: Into<String>, T: Into<String>, U: Into<String>>(
        &mut self,
        output: S,
        input1: T,
        input2: U,
    ) -> Result<(), CircuitError> {
        let gate = Gate::or(input1, input2)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_gate_or_value<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        value: u16,
    ) -> Result<(), CircuitError> {
        let gate = Gate::or_value(input, value)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_gate_sll<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        shift: u8,
    ) -> Result<(), CircuitError> {
        let gate = Gate::sll(input, shift)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_gate_slr<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        shift: u8,
    ) -> Result<(), CircuitError> {
        let gate = Gate::slr(input, shift)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_gate_not<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
    ) -> Result<(), CircuitError> {
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
                        self.set_signal(id, Some(*value));
                        ids.pop();
                    }
                    WireInput::Wire(input_id) => {
                        if let Some(signal) = self.get_signal(input_id) {
                            self.set_signal(id, Some(signal));
                            ids.pop();
                        } else {
                            ids.push(input_id.to_string());
                        }
                    }
                    WireInput::Gate(gate) => match gate {
                        Gate::And { input1, input2 } => {
                            if let Some(signal1) = self.get_signal(input1) {
                                if let Some(signal2) = self.get_signal(input2) {
                                    self.set_signal(id, Some(signal1 & signal2));
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
                        Gate::AndValue { input, value } => {
                            if let Some(signal1) = self.get_signal(input) {
                                self.set_signal(id, Some(signal1 & value));
                                ids.pop();
                            } else {
                                ids.push(input.to_string());
                            }
                        }
                        Gate::Or { input1, input2 } => {
                            if let Some(signal1) = self.get_signal(input1) {
                                if let Some(signal2) = self.get_signal(input2) {
                                    self.set_signal(id, Some(signal1 | signal2));
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
                        Gate::OrValue { input, value } => {
                            if let Some(signal1) = self.get_signal(input) {
                                self.set_signal(id, Some(signal1 | value));
                                ids.pop();
                            } else {
                                ids.push(input.to_string());
                            }
                        }
                        Gate::SLL { input, shift } => {
                            if let Some(signal) = self.get_signal(input) {
                                self.set_signal(id, Some(signal << shift));
                                ids.pop();
                            } else {
                                ids.push(input.to_string());
                            }
                        }
                        Gate::SLR { input, shift } => {
                            if let Some(signal) = self.get_signal(input) {
                                self.set_signal(id, Some(signal >> shift));
                                ids.pop();
                            } else {
                                ids.push(input.to_string());
                            }
                        }
                        Gate::Not { input } => {
                            if let Some(signal) = self.get_signal(input) {
                                self.set_signal(id, Some(!signal));
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

    fn set_signal(&mut self, id: &str, signal: Option<u16>) -> bool {
        // self.wires
        //     .get_mut(id)
        //     .map(|wire| wire.signal = signal)
        //     .is_some()
        if let Some(wire) = self.wires.get_mut(id) {
            wire.signal = signal;
            true
        } else {
            false
        }
    }

    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self, ParseCircuitError> {
        let s = fs::read_to_string(path)?;
        Self::try_from(s.as_str())
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error> {
        let data = self.to_string();
        let mut f = File::create(path)?;
        f.write_all(data.as_bytes())
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

impl TryFrom<&str> for Circuit {
    type Error = ParseCircuitError;

    fn try_from(s: &str) -> Result<Self, ParseCircuitError> {
        let mut circuit = Circuit::new();
        for wire in s.trim_end().split('\n') {
            circuit.add(wire.try_into()?)?
        }
        Ok(circuit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conflicting_wires() {
        let mut circuit = Circuit::new();
        assert!(circuit.add_wire_with_value("w", 0).is_ok());
        assert!(circuit.add_wire_with_value("w", 1).is_err());
    }

    #[test]
    fn simple_circuit() {
        let mut circuit = Circuit::new();
        let w1 = Wire::with_value("a", 1).unwrap();
        let w2 = Wire::from_wire("b", "a").unwrap();
        assert!(circuit.add(w1).is_ok());
        assert!(circuit.add(w2).is_ok());
        assert_eq!(circuit.get_signal("z"), None);
        assert_eq!(circuit.get_signal("a"), None);
        assert_eq!(circuit.get_signal("b"), None);

        circuit.compute_signals();
        assert_eq!(circuit.get_signal("a"), Some(1));
        assert_eq!(circuit.get_signal("b"), Some(1));

        let g = Gate::not("b").unwrap();
        let c = Wire::from_gate("c", g).unwrap();
        circuit.add(c).unwrap();
        assert_eq!(circuit.get_signal("c"), None);

        circuit.compute_signals();
        assert_eq!(circuit.get_signal("c"), Some(0xfffe));
        println!("{}", circuit);
    }

    #[test]
    fn nanocorp_example_1() -> Result<(), CircuitError> {
        let x = Wire::with_value("x", 123)?;
        let y = Wire::with_value("y", 456)?;
        let gd = Gate::and("x", "y")?;
        let ge = Gate::or("x", "y")?;
        let gf = Gate::sll("x", 2)?;
        let gg = Gate::slr("y", 2)?;
        let gh = Gate::not("x")?;
        let gi = Gate::not("y")?;
        let d = Wire::from_gate("d", gd)?;
        let e = Wire::from_gate("e", ge)?;
        let f = Wire::from_gate("f", gf)?;
        let g = Wire::from_gate("g", gg)?;
        let h = Wire::from_gate("h", gh)?;
        let i = Wire::from_gate("i", gi)?;

        let mut circuit = Circuit::new();
        circuit.add(x)?;
        circuit.add(y)?;
        circuit.add(d)?;
        circuit.add(e)?;
        circuit.add(f)?;
        circuit.add(g)?;
        circuit.add(h)?;
        circuit.add(i)?;

        circuit.compute_signals();

        assert_eq!(circuit.get_signal("d"), Some(72));
        assert_eq!(circuit.get_signal("e"), Some(507));
        assert_eq!(circuit.get_signal("f"), Some(492));
        assert_eq!(circuit.get_signal("g"), Some(114));
        assert_eq!(circuit.get_signal("h"), Some(65412));
        assert_eq!(circuit.get_signal("i"), Some(65079));
        assert_eq!(circuit.get_signal("x"), Some(123));
        assert_eq!(circuit.get_signal("y"), Some(456));
        Ok(())
    }

    #[test]
    fn nanocorp_example_1_bis() -> Result<(), CircuitError> {
        let mut circuit = Circuit::new();
        circuit.add_wire_with_value("x", 123)?;
        circuit.add_wire_with_value("y", 456)?;
        circuit.add_gate_and("d", "x", "y")?;
        circuit.add_gate_or("e", "x", "y")?;
        circuit.add_gate_sll("f", "x", 2)?;
        circuit.add_gate_slr("g", "y", 2)?;
        circuit.add_gate_not("h", "x")?;
        circuit.add_gate_not("i", "y")?;
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
        Ok(())
    }

    #[test]
    fn try_from_nanocorp_example_1() -> Result<(), CircuitError> {
        let s = "x AND y -> d\n\
		 NOT x -> h\n\
		 NOT y -> i\n\
		 x OR y -> e\n\
		 y RSHIFT 2 -> g\n\
		 x LSHIFT 2 -> f\n\
		 123 -> x\n\
		 456 -> y";
        let c1 = Circuit::try_from(s).unwrap();

        let mut c2 = Circuit::new();
        c2.add_wire_with_value("x", 123)?;
        c2.add_wire_with_value("y", 456)?;
        c2.add_gate_and("d", "x", "y")?;
        c2.add_gate_or("e", "x", "y")?;
        c2.add_gate_sll("f", "x", 2)?;
        c2.add_gate_slr("g", "y", 2)?;
        c2.add_gate_not("h", "x")?;
        c2.add_gate_not("i", "y")?;

        assert_eq!(c1, c2);
        Ok(())
    }

    #[test]
    fn read_nanocorp_example_2() -> Result<(), ParseCircuitError> {
        let c = Circuit::read("circuits/nanocorp_2.txt")?;
        println!("{}", c);
        Ok(())
    }

    #[test]
    fn write_nanocorp_example_1() -> Result<(), CircuitError> {
        let mut c = Circuit::new();
        c.add_wire_with_value("x", 123)?;
        c.add_wire_with_value("y", 456)?;
        c.add_gate_and("d", "x", "y")?;
        c.add_gate_or("e", "x", "y")?;
        c.add_gate_sll("f", "x", 2)?;
        c.add_gate_slr("g", "y", 2)?;
        c.add_gate_not("h", "x")?;
        c.add_gate_not("i", "y")?;
        c.write("circuits/nanocorp_1.txt").unwrap();
        Ok(())
    }
}
