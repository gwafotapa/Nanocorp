use std::{
    collections::HashMap,
    fmt,
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use crate::{
    error::Error,
    gate::Gate,
    wire::{Wire, WireInput},
    wire_id::WireId,
};

// TODO: check for types implementing clone, copy, ...
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

    pub fn remove(&mut self, id: &WireId) {
        self.wires.remove(id);
    }

    pub fn add(&mut self, wire: Wire) -> Result<(), Error> {
        if self.wires.contains_key(&wire.id) {
            Err(Error::WireIdAlreadyExists(wire.id))
        } else {
            self.wires.insert(wire.id.to_owned(), wire);
            Ok(())
        }
    }

    pub fn add_wire_with_input<S: Into<String>>(
        &mut self,
        id: S,
        input: WireInput,
    ) -> Result<(), Error> {
        let wire = Wire::new(id, input)?;
        self.add(wire)
    }

    pub fn add_wire_with_value<S: Into<String>>(&mut self, id: S, value: u16) -> Result<(), Error> {
        let wire = Wire::with_value(id, value)?;
        self.add(wire)
    }

    pub fn add_wire_from_wire<S: Into<String>, T: Into<String>>(
        &mut self,
        id: S,
        input_id: T,
    ) -> Result<(), Error> {
        let wire = Wire::from_wire(id, input_id)?;
        self.add(wire)
    }

    pub fn add_wire_from_gate<S: Into<String>>(&mut self, id: S, gate: Gate) -> Result<(), Error> {
        let wire = Wire::from_gate(id, gate)?;
        self.add(wire)
    }

    pub fn add_gate_and<S: Into<String>, T: Into<String>, U: Into<String>>(
        &mut self,
        output: S,
        input1: T,
        input2: U,
    ) -> Result<(), Error> {
        let gate = Gate::and(input1, input2)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)
    }

    pub fn add_gate_and_value<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        value: u16,
    ) -> Result<(), Error> {
        let gate = Gate::and_value(input, value)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)
    }

    pub fn add_gate_or<S: Into<String>, T: Into<String>, U: Into<String>>(
        &mut self,
        output: S,
        input1: T,
        input2: U,
    ) -> Result<(), Error> {
        let gate = Gate::or(input1, input2)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)
    }

    pub fn add_gate_or_value<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        value: u16,
    ) -> Result<(), Error> {
        let gate = Gate::or_value(input, value)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)
    }

    pub fn add_gate_sll<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        shift: u8,
    ) -> Result<(), Error> {
        let gate = Gate::sll(input, shift)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)
    }

    pub fn add_gate_slr<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        shift: u8,
    ) -> Result<(), Error> {
        let gate = Gate::slr(input, shift)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)
    }

    pub fn add_gate_not<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
    ) -> Result<(), Error> {
        let gate = Gate::not(input)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)
    }

    // TODO: rework
    // TODO: check for loops
    // TODO: add result return type
    pub fn compute_signals(&mut self) -> bool {
        let mut ids: Vec<WireId> = self.wires.keys().map(|id| id.to_owned()).collect();
        while let Some(id) = ids.last() {
            if let Some(wire) = self.wires.get(id) {
                match &wire.input {
                    WireInput::Value(value) => {
                        self.set_signal_of(id, Some(*value));
                        ids.pop();
                    }
                    WireInput::Wire(input_id) => {
                        if let Ok(input_wire) = self.get_wire(input_id) {
                            if let Some(signal) = input_wire.signal {
                                self.set_signal_of(id, Some(signal));
                                ids.pop();
                            } else {
                                ids.push(input_id.to_owned());
                            }
                        } else {
                            // Unknown wire
                            ids.pop();
                        }
                    }
                    WireInput::Gate(gate) => match gate {
                        Gate::And { input1, input2 } => {
                            if let (Ok(wire1), Ok(wire2)) =
                                (self.get_wire(input1), self.get_wire(input2))
                            {
                                if let Some(signal1) = wire1.signal {
                                    if let Some(signal2) = wire2.signal {
                                        self.set_signal_of(id, Some(signal1 & signal2));
                                        ids.pop();
                                    } else {
                                        ids.push(input2.to_owned());
                                    }
                                } else {
                                    ids.push(input1.to_owned());
                                    if wire2.signal.is_none() {
                                        ids.push(input2.to_owned());
                                    }
                                }
                            } else {
                                ids.pop();
                            }
                        }
                        Gate::AndValue { input, value } => {
                            if let Ok(input_wire) = self.get_wire(input) {
                                if let Some(signal) = input_wire.signal {
                                    self.set_signal_of(id, Some(signal & value));
                                    ids.pop();
                                } else {
                                    ids.push(input.to_owned());
                                }
                            } else {
                                ids.pop();
                            }
                        }
                        Gate::Or { input1, input2 } => {
                            if let (Ok(wire1), Ok(wire2)) =
                                (self.get_wire(input1), self.get_wire(input2))
                            {
                                if let Some(signal1) = wire1.signal {
                                    if let Some(signal2) = wire2.signal {
                                        self.set_signal_of(id, Some(signal1 | signal2));
                                        ids.pop();
                                    } else {
                                        ids.push(input2.to_owned());
                                    }
                                } else {
                                    ids.push(input1.to_owned());
                                    if wire2.signal.is_none() {
                                        ids.push(input2.to_owned());
                                    }
                                }
                            } else {
                                ids.pop();
                            }
                        }
                        Gate::OrValue { input, value } => {
                            if let Ok(input_wire) = self.get_wire(input) {
                                if let Some(signal) = input_wire.signal {
                                    self.set_signal_of(id, Some(signal | value));
                                    ids.pop();
                                } else {
                                    ids.push(input.to_owned());
                                }
                            } else {
                                ids.pop();
                            }
                        }
                        Gate::SLL { input, shift } => {
                            if let Ok(input_wire) = self.get_wire(input) {
                                if let Some(signal) = input_wire.signal {
                                    self.set_signal_of(id, Some(signal << shift));
                                    ids.pop();
                                } else {
                                    ids.push(input.to_owned());
                                }
                            } else {
                                ids.pop();
                            }
                        }
                        Gate::SLR { input, shift } => {
                            if let Ok(input_wire) = self.get_wire(input) {
                                if let Some(signal) = input_wire.signal {
                                    self.set_signal_of(id, Some(signal >> shift));
                                    ids.pop();
                                } else {
                                    ids.push(input.to_owned());
                                }
                            } else {
                                ids.pop();
                            }
                        }
                        Gate::Not { input } => {
                            if let Ok(input_wire) = self.get_wire(input) {
                                if let Some(signal) = input_wire.signal {
                                    self.set_signal_of(id, Some(!signal));
                                    ids.pop();
                                } else {
                                    ids.push(input.to_owned());
                                }
                            } else {
                                ids.pop();
                            }
                        }
                    },
                }
            } else {
                // Unkwown wire id
                ids.pop();
            }
        }

        true
    }

    fn get_wire(&self, id: &WireId) -> Result<&Wire, Error> {
        // TODO: check for use of clone instead of to_owned
        self.wires
            .get(id)
            .ok_or(Error::UnknownWireId(id.to_owned()))
    }

    fn wire(&self, id: &WireId) -> &Wire {
        self.get_wire(id).unwrap()
    }

    fn get_signal_of(&self, id: &WireId) -> Result<Option<u16>, Error> {
        self.get_wire(id).map(|w| w.signal)
    }

    fn signal_of(&self, id: &WireId) -> Option<u16> {
        self.get_signal_of(id).unwrap()
    }

    pub fn get_signal_from<S: AsRef<str>>(&self, id: S) -> Result<Option<u16>, Error> {
        let id = WireId::try_from(id.as_ref())?;
        self.get_signal_of(&id)
    }

    pub fn signal_from<S: AsRef<str>>(&self, id: S) -> Option<u16> {
        self.get_signal_from(id).unwrap()
    }

    // TODO: Wire public or private ? wire.set_signal_of() and wire.get_signal() ?
    fn set_signal_of(&mut self, id: &WireId, signal: Option<u16>) -> bool {
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

    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
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
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Error> {
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
        assert!(circuit.get_signal_from("z").is_err()); // TODO: should return unknown wire error
                                                        // TODO: remove matches! and use signal_from instead ?
        assert!(matches!(circuit.get_signal_from("a"), Ok(None)));
        assert!(matches!(circuit.get_signal_from("b"), Ok(None)));

        circuit.compute_signals();
        assert!(matches!(circuit.get_signal_from("a"), Ok(Some(1))));
        assert!(matches!(circuit.get_signal_from("b"), Ok(Some(1))));

        let g = Gate::not("b").unwrap();
        let c = Wire::from_gate("c", g).unwrap();
        circuit.add(c).unwrap();
        assert!(matches!(circuit.get_signal_from("c"), Ok(None)));

        circuit.compute_signals();
        assert!(matches!(circuit.get_signal_from("c"), Ok(Some(0xfffe))));
        println!("{}", circuit);
    }

    #[test]
    fn nanocorp_example_1() -> Result<(), Error> {
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

        assert!(matches!(circuit.get_signal_from("d"), Ok(Some(72))));
        assert!(matches!(circuit.get_signal_from("e"), Ok(Some(507))));
        assert!(matches!(circuit.get_signal_from("f"), Ok(Some(492))));
        assert!(matches!(circuit.get_signal_from("g"), Ok(Some(114))));
        assert!(matches!(circuit.get_signal_from("h"), Ok(Some(65412))));
        assert!(matches!(circuit.get_signal_from("i"), Ok(Some(65079))));
        assert!(matches!(circuit.get_signal_from("x"), Ok(Some(123))));
        assert!(matches!(circuit.get_signal_from("y"), Ok(Some(456))));
        Ok(())
    }

    #[test]
    fn nanocorp_example_1_bis() -> Result<(), Error> {
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

        assert!(matches!(circuit.get_signal_from("d"), Ok(Some(72))));
        assert!(matches!(circuit.get_signal_from("e"), Ok(Some(507))));
        assert!(matches!(circuit.get_signal_from("f"), Ok(Some(492))));
        assert!(matches!(circuit.get_signal_from("g"), Ok(Some(114))));
        assert!(matches!(circuit.get_signal_from("h"), Ok(Some(65412))));
        assert!(matches!(circuit.get_signal_from("i"), Ok(Some(65079))));
        assert!(matches!(circuit.get_signal_from("x"), Ok(Some(123))));
        assert!(matches!(circuit.get_signal_from("y"), Ok(Some(456))));
        Ok(())
    }

    #[test]
    fn try_from_nanocorp_example_1() -> Result<(), Error> {
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
    fn read_nanocorp_example_2() -> Result<(), Error> {
        let c = Circuit::read("circuits/nanocorp_2.txt")?;
        println!("{}", c);
        Ok(())
    }

    #[test]
    fn write_nanocorp_example_1() -> Result<(), Error> {
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

    #[test]
    fn non_connected_wires() {
        let mut c = Circuit::new();
        c.add_wire_with_value("x", 0xfff0).unwrap();
        c.add_wire_with_value("y", 0x0fff).unwrap();
        c.add_gate_or("xoy", "x", "y").unwrap();
        c.add_gate_and("xoyau", "xoy", "unknown").unwrap();
        c.add_gate_sll("u", "unknown", 2).unwrap();
        c.add_gate_slr("v", "unknown", 2).unwrap();
        c.add_gate_not("nxoy", "xoy").unwrap();
        c.add_gate_not("w", "unknown").unwrap();

        c.compute_signals();

        assert_eq!(c.signal_from("x"), Some(0xfff0));
        assert_eq!(c.signal_from("y"), Some(0x0fff));
        assert_eq!(c.signal_from("xoy"), Some(0xffff));
        assert_eq!(c.signal_from("nxoy"), Some(0x0));
        assert_eq!(c.signal_from("u"), None);
        assert_eq!(c.signal_from("v"), None);
        assert_eq!(c.signal_from("w"), None);
        assert_eq!(c.signal_from("xoyau"), None);
        assert!(matches!(
            c.get_signal_from("unknown"),
            Err(Error::UnknownWireId(_))
        ));
    }

    #[test]
    fn identical_gate_inputs() {
        let x = 0xa35c;
        let mut c = Circuit::new();
        c.add_wire_with_value("x", x).unwrap();
        c.add_gate_or("xox", "x", "x").unwrap();
        c.add_gate_and("xax", "x", "x").unwrap();

        c.compute_signals();

        assert_eq!(c.signal_from("x"), Some(x));
        assert_eq!(c.signal_from("xox"), Some(x));
        assert_eq!(c.signal_from("xax"), Some(x));
    }
}
