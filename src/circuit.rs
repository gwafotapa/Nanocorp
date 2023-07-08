use std::{
    collections::{HashMap, HashSet},
    fmt,
    fs::{self, File},
    hash::Hash,
    io::{self, Write},
    path::Path,
};

use crate::{
    error::Error,
    gate::Gate,
    signal::Signal,
    wire::{Wire, WireInput},
    wire_id::WireId,
};

// TODO: check for types implementing clone, copy, ...
#[derive(Debug, Default, PartialEq)]
pub struct Circuit {
    pub wires: HashMap<WireId, Wire>,
}

impl Circuit {
    //
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

    // TODO: define type Signal = Option<u16>
    // TODO: rework
    // TODO: Need reset_signals() in case we add wire after compute_signals
    pub fn compute_signals(&mut self) -> Result<(), Error> {
        let mut ids: Vec<WireId> = self.wires.keys().map(|id| id.to_owned()).collect();
        // Index of id the computation originated from
        let mut root_index: Option<usize> = None;
        while let Some(id) = ids.last() {
            if root_index.is_none() {
                root_index = Some(ids.len() - 1);
            }
            if let Some(wire) = self.wires.get(id) {
                match wire.signal {
                    Signal::Value(_) => {
                        ids.pop();
                    }
                    Signal::Uncomputable => {
                        for id in &ids[root_index.unwrap()..] {
                            self.set_signal_of(id, Signal::Uncomputable);
                        }
                        ids.truncate(root_index.unwrap());
                    }
                    Signal::Uncomputed => {
                        match &wire.input {
                            WireInput::Value(value) => {
                                self.set_signal_of(id, Signal::Value(*value));
                                ids.pop();
                            }
                            WireInput::Wire(input_id) => {
                                if let Ok(input_wire) = self.get_wire(input_id) {
                                    match input_wire.signal {
                                        Signal::Value(signal) => {
                                            self.set_signal_of(id, Signal::Value(signal));
                                            ids.pop();
                                        }
                                        Signal::Uncomputable => {
                                            self.set_signal_of(id, Signal::Uncomputable);
                                            for id in &ids[root_index.unwrap()..] {
                                                self.set_signal_of(id, Signal::Uncomputable);
                                            }
                                            ids.truncate(root_index.unwrap());
                                        }
                                        Signal::Uncomputed => {
                                            if ids[root_index.unwrap()..].contains(input_id) {
                                                return Err(Error::CircuitLoop);
                                            }
                                            ids.push(input_id.to_owned());
                                        }
                                    }
                                } else {
                                    // Unknown wire id
                                    for id in &ids[root_index.unwrap()..] {
                                        self.set_signal_of(id, Signal::Uncomputable);
                                    }
                                    ids.truncate(root_index.unwrap());
                                }
                            }
                            WireInput::Gate(gate) => match gate {
                                Gate::And { input1, input2 } => {
                                    if let (Ok(wire1), Ok(wire2)) =
                                        (self.get_wire(input1), self.get_wire(input2))
                                    {
                                        match (wire1.signal, wire2.signal) {
                                            (Signal::Value(signal1), Signal::Value(signal2)) => {
                                                self.set_signal_of(
                                                    id,
                                                    Signal::Value(signal1 & signal2),
                                                );
                                                ids.pop();
                                            }
                                            (Signal::Uncomputable, _)
                                            | (_, Signal::Uncomputable) => {
                                                // self.set_signal_of(id, Signal::Uncomputable);
                                                for id in &ids[root_index.unwrap()..] {
                                                    self.set_signal_of(id, Signal::Uncomputable);
                                                }
                                                ids.truncate(root_index.unwrap());
                                            }
                                            (Signal::Uncomputed, _) => {
                                                if ids[root_index.unwrap()..].contains(input1) {
                                                    return Err(Error::CircuitLoop);
                                                }
                                                ids.push(input1.to_owned());
                                            }
                                            (_, Signal::Uncomputed) => {
                                                if ids[root_index.unwrap()..].contains(input2) {
                                                    return Err(Error::CircuitLoop);
                                                }
                                                ids.push(input2.to_owned());
                                            }
                                        }
                                    } else {
                                        for id in &ids[root_index.unwrap()..] {
                                            self.set_signal_of(id, Signal::Uncomputable);
                                        }
                                        ids.truncate(root_index.unwrap());
                                    }
                                }
                                Gate::AndValue { input, value } => {
                                    if let Ok(input_wire) = self.get_wire(input) {
                                        match input_wire.signal {
                                            Signal::Value(signal) => {
                                                self.set_signal_of(
                                                    id,
                                                    Signal::Value(signal & value),
                                                );
                                                ids.pop();
                                            }
                                            Signal::Uncomputable => {
                                                for id in &ids[root_index.unwrap()..] {
                                                    self.set_signal_of(id, Signal::Uncomputable);
                                                }
                                                ids.truncate(root_index.unwrap());
                                            }
                                            Signal::Uncomputed => {
                                                if ids[root_index.unwrap()..].contains(input) {
                                                    return Err(Error::CircuitLoop);
                                                }
                                                ids.push(input.to_owned());
                                            }
                                        }
                                    } else {
                                        for id in &ids[root_index.unwrap()..] {
                                            self.set_signal_of(id, Signal::Uncomputable);
                                        }
                                        ids.truncate(root_index.unwrap());
                                    }
                                }
                                Gate::Or { input1, input2 } => {
                                    if let (Ok(wire1), Ok(wire2)) =
                                        (self.get_wire(input1), self.get_wire(input2))
                                    {
                                        match (wire1.signal, wire2.signal) {
                                            (Signal::Value(signal1), Signal::Value(signal2)) => {
                                                self.set_signal_of(
                                                    id,
                                                    Signal::Value(signal1 | signal2),
                                                );
                                                ids.pop();
                                            }
                                            (Signal::Uncomputable, _)
                                            | (_, Signal::Uncomputable) => {
                                                // self.set_signal_of(id, Signal::Uncomputable);
                                                for id in &ids[root_index.unwrap()..] {
                                                    self.set_signal_of(id, Signal::Uncomputable);
                                                }
                                                ids.truncate(root_index.unwrap());
                                            }
                                            (Signal::Uncomputed, _) => {
                                                if ids[root_index.unwrap()..].contains(input1) {
                                                    return Err(Error::CircuitLoop);
                                                }
                                                ids.push(input1.to_owned());
                                            }
                                            (_, Signal::Uncomputed) => {
                                                if ids[root_index.unwrap()..].contains(input2) {
                                                    return Err(Error::CircuitLoop);
                                                }
                                                ids.push(input2.to_owned());
                                            }
                                        }
                                    } else {
                                        for id in &ids[root_index.unwrap()..] {
                                            self.set_signal_of(id, Signal::Uncomputable);
                                        }
                                        ids.truncate(root_index.unwrap());
                                    }
                                }
                                Gate::OrValue { input, value } => {
                                    if let Ok(input_wire) = self.get_wire(input) {
                                        match input_wire.signal {
                                            Signal::Value(signal) => {
                                                self.set_signal_of(
                                                    id,
                                                    Signal::Value(signal | value),
                                                );
                                                ids.pop();
                                            }
                                            Signal::Uncomputable => {
                                                for id in &ids[root_index.unwrap()..] {
                                                    self.set_signal_of(id, Signal::Uncomputable);
                                                }
                                                ids.truncate(root_index.unwrap());
                                            }
                                            Signal::Uncomputed => {
                                                if ids[root_index.unwrap()..].contains(input) {
                                                    return Err(Error::CircuitLoop);
                                                }
                                                ids.push(input.to_owned());
                                            }
                                        }
                                    } else {
                                        for id in &ids[root_index.unwrap()..] {
                                            self.set_signal_of(id, Signal::Uncomputable);
                                        }
                                        ids.truncate(root_index.unwrap());
                                    }
                                }
                                Gate::SLL { input, shift } => {
                                    if let Ok(input_wire) = self.get_wire(input) {
                                        match input_wire.signal {
                                            Signal::Value(signal) => {
                                                self.set_signal_of(
                                                    id,
                                                    Signal::Value(signal << shift),
                                                );
                                                ids.pop();
                                            }
                                            Signal::Uncomputable => {
                                                for id in &ids[root_index.unwrap()..] {
                                                    self.set_signal_of(id, Signal::Uncomputable);
                                                }
                                                ids.truncate(root_index.unwrap());
                                            }
                                            Signal::Uncomputed => {
                                                if ids[root_index.unwrap()..].contains(input) {
                                                    return Err(Error::CircuitLoop);
                                                }
                                                ids.push(input.to_owned());
                                            }
                                        }
                                    } else {
                                        for id in &ids[root_index.unwrap()..] {
                                            self.set_signal_of(id, Signal::Uncomputable);
                                        }
                                        ids.truncate(root_index.unwrap());
                                    }
                                }
                                Gate::SLR { input, shift } => {
                                    if let Ok(input_wire) = self.get_wire(input) {
                                        match input_wire.signal {
                                            Signal::Value(signal) => {
                                                self.set_signal_of(
                                                    id,
                                                    Signal::Value(signal >> shift),
                                                );
                                                ids.pop();
                                            }
                                            Signal::Uncomputable => {
                                                for id in &ids[root_index.unwrap()..] {
                                                    self.set_signal_of(id, Signal::Uncomputable);
                                                }
                                                ids.truncate(root_index.unwrap());
                                            }
                                            Signal::Uncomputed => {
                                                if ids[root_index.unwrap()..].contains(input) {
                                                    return Err(Error::CircuitLoop);
                                                }
                                                ids.push(input.to_owned());
                                            }
                                        }
                                    } else {
                                        for id in &ids[root_index.unwrap()..] {
                                            self.set_signal_of(id, Signal::Uncomputable);
                                        }
                                        ids.truncate(root_index.unwrap());
                                    }
                                }
                                Gate::Not { input } => {
                                    if let Ok(input_wire) = self.get_wire(input) {
                                        match input_wire.signal {
                                            Signal::Value(signal) => {
                                                self.set_signal_of(id, Signal::Value(!signal));
                                                ids.pop();
                                            }
                                            Signal::Uncomputable => {
                                                for id in &ids[root_index.unwrap()..] {
                                                    self.set_signal_of(id, Signal::Uncomputable);
                                                }
                                                ids.truncate(root_index.unwrap());
                                            }
                                            Signal::Uncomputed => {
                                                if ids[root_index.unwrap()..].contains(input) {
                                                    return Err(Error::CircuitLoop);
                                                }
                                                ids.push(input.to_owned());
                                            }
                                        }
                                    } else {
                                        for id in &ids[root_index.unwrap()..] {
                                            self.set_signal_of(id, Signal::Uncomputable);
                                        }
                                        ids.truncate(root_index.unwrap());
                                    }
                                }
                            },
                        }
                    }
                }
            } else {
                // Unkwown wire id
                for id in &ids[root_index.unwrap()..] {
                    self.set_signal_of(id, Signal::Uncomputable);
                }
                ids.truncate(root_index.unwrap());
            }
            if root_index.unwrap() >= ids.len() {
                root_index = None
            }
        }
        Ok(())
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

    fn get_signal_of(&self, id: &WireId) -> Result<Signal, Error> {
        self.get_wire(id).map(|w| w.signal)
    }

    pub fn signal_of(&self, id: &WireId) -> Signal {
        self.get_signal_of(id).unwrap()
    }

    pub fn get_signal_from<S: AsRef<str>>(&self, id: S) -> Result<Signal, Error> {
        let id = WireId::try_from(id.as_ref())?;
        self.get_signal_of(&id)
    }

    pub fn signal_from<S: AsRef<str>>(&self, id: S) -> Signal {
        self.get_signal_from(id).unwrap()
    }

    // TODO: Wire public or private ? wire.set_signal_of() and wire.get_signal() ?
    fn set_signal_of(&mut self, id: &WireId, signal: Signal) -> bool {
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

// impl Default for Circuit {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl fmt::Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for wire in self.wires.values() {
            writeln!(f, "{}", wire)?
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

// https://stackoverflow.com/questions/46766560/how-to-check-if-there-are-duplicates-in-a-slice
fn has_duplicate_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().any(move |x| !uniq.insert(x))
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
        assert!(matches!(
            circuit.get_signal_from("a"),
            Ok(Signal::Uncomputed)
        ));
        assert!(matches!(
            circuit.get_signal_from("b"),
            Ok(Signal::Uncomputed)
        ));

        assert!(circuit.compute_signals().is_ok());
        assert!(matches!(circuit.get_signal_from("a"), Ok(Signal::Value(1))));
        assert!(matches!(circuit.get_signal_from("b"), Ok(Signal::Value(1))));

        // let g = Gate::not("b").unwrap();
        // let c = Wire::from_gate("c", g).unwrap();
        // circuit.add(c).unwrap();
        // assert!(matches!(circuit.get_signal_from("c"), Ok(None)));

        // assert!(circuit.compute_signals().is_ok());
        // assert!(matches!(circuit.get_signal_from("c"), Ok(Some(0xfffe))));
        // println!("{}", circuit);
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

        assert!(circuit.compute_signals().is_ok());

        assert!(matches!(
            circuit.get_signal_from("d"),
            Ok(Signal::Value(72))
        ));
        assert!(matches!(
            circuit.get_signal_from("e"),
            Ok(Signal::Value(507))
        ));
        assert!(matches!(
            circuit.get_signal_from("f"),
            Ok(Signal::Value(492))
        ));
        assert!(matches!(
            circuit.get_signal_from("g"),
            Ok(Signal::Value(114))
        ));
        assert!(matches!(
            circuit.get_signal_from("h"),
            Ok(Signal::Value(65412))
        ));
        assert!(matches!(
            circuit.get_signal_from("i"),
            Ok(Signal::Value(65079))
        ));
        assert!(matches!(
            circuit.get_signal_from("x"),
            Ok(Signal::Value(123))
        ));
        assert!(matches!(
            circuit.get_signal_from("y"),
            Ok(Signal::Value(456))
        ));
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
        assert!(circuit.compute_signals().is_ok());

        println!("{}", circuit);

        assert!(matches!(
            circuit.get_signal_from("d"),
            Ok(Signal::Value(72))
        ));
        assert!(matches!(
            circuit.get_signal_from("e"),
            Ok(Signal::Value(507))
        ));
        assert!(matches!(
            circuit.get_signal_from("f"),
            Ok(Signal::Value(492))
        ));
        assert!(matches!(
            circuit.get_signal_from("g"),
            Ok(Signal::Value(114))
        ));
        assert!(matches!(
            circuit.get_signal_from("h"),
            Ok(Signal::Value(65412))
        ));
        assert!(matches!(
            circuit.get_signal_from("i"),
            Ok(Signal::Value(65079))
        ));
        assert!(matches!(
            circuit.get_signal_from("x"),
            Ok(Signal::Value(123))
        ));
        assert!(matches!(
            circuit.get_signal_from("y"),
            Ok(Signal::Value(456))
        ));
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
        // c.add_wire_with_value("x", 0xfff0).unwrap();
        // c.add_wire_with_value("y", 0x0fff).unwrap();
        // c.add_gate_or("xoy", "x", "y").unwrap();
        // c.add_gate_and("xoyau", "xoy", "unknown").unwrap();
        c.add_gate_sll("u", "unknown", 2).unwrap();
        // c.add_gate_slr("v", "unknown", 2).unwrap();
        // c.add_gate_not("nxoy", "xoy").unwrap();
        // c.add_gate_not("w", "unknown").unwrap();

        assert!(c.compute_signals().is_ok());

        // assert_eq!(c.signal_from("x"), Signal::Value(0xfff0));
        // assert_eq!(c.signal_from("y"), Signal::Value(0x0fff));
        // assert_eq!(c.signal_from("xoy"), Signal::Value(0xffff));
        // assert_eq!(c.signal_from("nxoy"), Signal::Value(0x0));
        // assert_eq!(c.signal_from("u"), Signal::Uncomputable);
        // assert_eq!(c.signal_from("v"), Signal::Uncomputable);
        // assert_eq!(c.signal_from("w"), Signal::Uncomputable);
        // assert_eq!(c.signal_from("xoyau"), Signal::Uncomputable);
        // assert!(matches!(
        //     c.get_signal_from("unknown"),
        //     Err(Error::UnknownWireId(_))
        // ));
    }

    #[test]
    fn identical_gate_inputs() {
        let x = 0xa35c;
        let mut c = Circuit::new();
        c.add_wire_with_value("x", x).unwrap();
        c.add_gate_or("xox", "x", "x").unwrap();
        c.add_gate_and("xax", "x", "x").unwrap();

        assert!(c.compute_signals().is_ok());

        assert_eq!(c.signal_from("x"), Signal::Value(x));
        assert_eq!(c.signal_from("xox"), Signal::Value(x));
        assert_eq!(c.signal_from("xax"), Signal::Value(x));
    }

    #[test]
    fn loop_2_wires() {
        let mut c = Circuit::new();
        c.add_wire_from_wire("a", "b").unwrap();
        c.add_wire_from_wire("b", "a").unwrap();
        assert!(c.compute_signals().is_err());
    }

    #[test]
    fn loop_3_wires() {
        let mut c = Circuit::new();
        c.add_wire_from_wire("a", "b").unwrap();
        c.add_gate_and("b", "c", "d").unwrap();
        c.add_gate_or("c", "e", "f").unwrap();
        c.add_gate_not("f", "b").unwrap();
        c.add_wire_with_value("d", 19).unwrap();
        c.add_wire_with_value("e", 7).unwrap();
        assert!(c.compute_signals().is_err());
    }

    #[test]
    fn compute_signals_then_add_wire() {
        let mut c = Circuit::new();
        c.add_wire_with_value("b", 0x10).unwrap();
        c.add_wire_with_value("c", 0x100).unwrap();
        c.add_gate_or("aob", "a", "b").unwrap();
        c.add_gate_or("boc", "b", "c").unwrap();
        c.add_gate_or("cod", "c", "d").unwrap();
        c.add_gate_and("x", "aob", "boc").unwrap();
        c.add_gate_and("y", "boc", "cod").unwrap();
        c.add_gate_or("z", "x", "y").unwrap();
        c.add_gate_not("nz", "z").unwrap();
        assert!(c.compute_signals().is_ok());
        assert!(c.get_signal_from("a").is_err());
        assert_eq!(c.signal_from("b"), Signal::Value(0x10));
        assert_eq!(c.signal_from("c"), Signal::Value(0x100));
        assert!(c.get_signal_from("d").is_err());
        assert_eq!(c.signal_from("aob"), Signal::Uncomputable);
        assert_eq!(c.signal_from("boc"), Signal::Value(0x110));
        assert_eq!(c.signal_from("cod"), Signal::Uncomputable);
        assert_eq!(c.signal_from("x"), Signal::Uncomputable);
        assert_eq!(c.signal_from("y"), Signal::Uncomputable);
        assert_eq!(c.signal_from("z"), Signal::Uncomputable);
        assert_eq!(c.signal_from("nz"), Signal::Uncomputable);

        c.add_wire_with_value("a", 0x1).unwrap();
        c.add_wire_with_value("d", 0x1000).unwrap();
        assert!(c.compute_signals().is_ok());
        assert_eq!(c.signal_from("a"), Signal::Value(0x1));
        assert_eq!(c.signal_from("b"), Signal::Value(0x10));
        assert_eq!(c.signal_from("c"), Signal::Value(0x100));
        assert_eq!(c.signal_from("d"), Signal::Value(0x1000));
        assert_eq!(c.signal_from("aob"), Signal::Value(0x11));
        assert_eq!(c.signal_from("boc"), Signal::Value(0x110));
        assert_eq!(c.signal_from("cod"), Signal::Value(0x1100));
        assert_eq!(c.signal_from("x"), Signal::Value(0x10));
        assert_eq!(c.signal_from("y"), Signal::Value(0x100));
        assert_eq!(c.signal_from("z"), Signal::Value(0x110));
        assert_eq!(c.signal_from("nz"), Signal::Value(0xfeef));
    }
}
