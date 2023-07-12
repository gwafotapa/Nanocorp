use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    fs::{self, File},
    io::Write,
    mem,
    path::Path,
};

use super::wire::{gate::Gate, signal::Signal, wire_id::WireId, wire_input::WireInput, Wire};
use crate::error::{Error, Result};

/// A circuit is a set of connected wires and gates
///
/// A circuit is built by adding wires one at a time.
/// Each wire has a unique id which is an ascii lowercase string.  
/// A wire can have three kinds of input:
/// - a value ([u16])
/// - the output of another wire
/// - a gate combining outputs of other wires
///
/// When first added, a wire's signal is [`Signal::Uncomputed`].
/// Calling [`compute_signals()`](Self::compute_signals) will compute signals
/// for all wires in the circuit.
/// You can then retrieve a signal by calling [`signal()`](Self::signal)
/// with the id of the wire you're interested in.
///
/// # Example
///
/// The following circuit determines if a number is a multiple of 4.  
/// Wire x takes the number as input, here 100.
/// Wire res emits signal 1 if x is a multiple of 4 and 0 otherwise.
/// ```
/// # use circuitry::{Circuit, Signal, Error};
/// # fn main() -> Result<(), Error> {
/// let mut is_multiple_of_4 = Circuit::new();
/// is_multiple_of_4.add_wire_with_value("x", 100)?;       // Adds wire x emitting 100
/// is_multiple_of_4.add_gate_and_value("y", "x", 1)?;     // Adds wire y emitting x & 1
/// is_multiple_of_4.add_gate_and_value("z", "x", 2)?;     // Adds wire z emitting x & 2
/// is_multiple_of_4.add_gate_or("yz", "y", "z")?;         // Adds wire yz emitting y | z
/// is_multiple_of_4.add_gate_not("nyz", "yz")?;           // Adds wire nyz emitting !yz
/// is_multiple_of_4.add_gate_and_value("res", "nyz", 1)?; // Adds wire res emitting nyz & 1
/// assert_eq!(is_multiple_of_4.signal("x"), Signal::Uncomputed);
/// assert_eq!(is_multiple_of_4.signal("res"), Signal::Uncomputed);
///
/// is_multiple_of_4.compute_signals()?;
/// assert_eq!(is_multiple_of_4.signal("x"), Signal::Value(100));
/// assert_eq!(is_multiple_of_4.signal("res"), Signal::Value(1));
/// # Ok(())
/// # }
/// ```
/// A [`CircuitBuilder`](super::CircuitBuilder) is provided
/// to avoid retyping the circuit's name with each addition of a wire.  
/// Methods of [`CircuitBuilder`](super::CircuitBuilder) for adding wires
/// have names identical to those of [`Circuit`].
///
/// Wires also have string representations if you prefer.
/// In that case, use repeated calls to [`add_wire()`](Self::add_wire).
///
/// Here's an example using [`CircuitBuilder`](super::CircuitBuilder) with string representation.
///
/// # Example
/// The circuit below computes the logical XOR between its wires x and y.
///
/// ```
/// # use circuitry::{CircuitBuilder, Signal, Error};
/// # fn main() -> Result<(), Error> {
/// let mut circuit = CircuitBuilder::new()
///     .add_wire("2536 -> x")?        // Adds wire x emitting signal 2536
///     .add_wire("9711 -> y")?
///     .add_wire("x OR y -> o")?      // Adds wire o output of a gate OR with inputs x and y
///     .add_wire("x AND y -> a")?
///     .add_wire("NOT a -> na")?
///     .add_wire("o AND na -> xor")?
///     .build();
///
/// circuit.compute_signals()?;
/// assert_eq!(circuit.signal("xor"), Signal::Value(2536 ^ 9711));
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Default)]
pub struct Circuit {
    wires: HashMap<WireId, Wire>,
    uncomputed: Vec<WireId>,
    uncomputable: Vec<WireId>,
}

impl Circuit {
    /// Creates an empty circuit.
    pub fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, wire: Wire) -> Result<()> {
        if self.wires.contains_key(wire.id()) {
            Err(Error::WireIdAlreadyExists(wire.id().to_string()))
        } else {
            self.uncomputed.push(wire.id().to_owned());
            self.wires.insert(wire.id().to_owned(), wire);
            Ok(())
        }
    }

    /// Adds a wire using string representation.
    /// See [example](Circuit#example-1) for usage.

    pub fn add_wire(&mut self, s: &str) -> Result<()> {
        self.add(Wire::try_from(s)?)
    }

    /// Adds a wire `id` whose input is a value.
    /// Returns an error if `id` is not ascii lowercase.
    pub fn add_wire_with_value<S: Into<String>>(&mut self, id: S, value: u16) -> Result<()> {
        self.add(Wire::with_value(id, value)?)
    }

    /// Adds a wire `id` whose input is another wire `input_id`.  
    /// Returns an error if `id` or `input_id` is not ascii lowercase
    /// or if `id` and `input_id` match.
    pub fn add_wire_from_wire<S: Into<String>, T: Into<String>>(
        &mut self,
        id: S,
        input_id: T,
    ) -> Result<()> {
        self.add(Wire::from_wire(id, input_id)?)
    }

    /// Adds a wire `output` fed by a logical AND gate between wires `input1` and `input2`.  
    /// Returns an error if any id is not ascii lowercase or if `output` matches an input.
    pub fn add_gate_and<S: Into<String>, T: Into<String>, U: Into<String>>(
        &mut self,
        output: S,
        input1: T,
        input2: U,
    ) -> Result<()> {
        self.add(Wire::from_gate_and(output, input1, input2)?)
    }

    /// Adds a wire `output` fed by a logical AND gate between wire `input` and value.  
    /// Returns an error if `output` or `input` is not ascii lowercase
    /// or if `output` matches `input`.
    pub fn add_gate_and_value<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        value: u16,
    ) -> Result<()> {
        self.add(Wire::from_gate_and_value(output, input, value)?)
    }

    /// Adds a wire `output` fed by a logical OR gate between wires `input1` and `input2`.  
    /// Returns an error if any id is not ascii lowercase or if `output` matches an input.
    pub fn add_gate_or<S: Into<String>, T: Into<String>, U: Into<String>>(
        &mut self,
        output: S,
        input1: T,
        input2: U,
    ) -> Result<()> {
        self.add(Wire::from_gate_or(output, input1, input2)?)
    }

    /// Adds a wire `output` fed by a logical OR gate between wire `input` and value.  
    /// Returns an error if `output` or `input` is not ascii lowercase
    /// or if `output` matches `input`.
    pub fn add_gate_or_value<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        value: u16,
    ) -> Result<()> {
        self.add(Wire::from_gate_or_value(output, input, value)?)
    }

    /// Adds a wire `output` fed by a logical LEFT SHIFT gate of wire `input` by amount `shift`.  
    /// Returns an error if `output` or `input` is not ascii lowercase
    /// or if `output` matches `input`.
    pub fn add_gate_lshift<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        shift: u8,
    ) -> Result<()> {
        self.add(Wire::from_gate_lshift(output, input, shift)?)
    }

    /// Adds a wire `output` fed by a logical RIGHT SHIFT gate of wire `input` by amount `shift`.  
    /// Returns an error if `output` or `input` is not ascii lowercase
    /// or if `output` matches `input`.
    pub fn add_gate_rshift<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        shift: u8,
    ) -> Result<()> {
        self.add(Wire::from_gate_rshift(output, input, shift)?)
    }

    /// Adds a wire `output` fed by a logical NOT gate of wire `input`.  
    /// Returns an error if `output` or `input` is not ascii lowercase
    /// or if `output` matches `input`.
    pub fn add_gate_not<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
    ) -> Result<()> {
        self.add(Wire::from_gate_not(output, input)?)
    }

    pub(super) fn get_wires(&self) -> &HashMap<WireId, Wire> {
        &self.wires
    }

    #[allow(dead_code)]
    fn get_wire<S: Into<String>>(&self, id: S) -> Result<&Wire> {
        self.get_wire_of(&WireId::new(id)?)
    }

    fn get_wire_of(&self, id: &WireId) -> Result<&Wire> {
        self.wires
            .get(id)
            .ok_or(Error::UnknownWireId(id.to_string()))
    }

    #[allow(dead_code)]
    fn wire_of(&self, id: &WireId) -> &Wire {
        self.get_wire_of(id).unwrap()
    }

    /// Retrieves signal of wire `id`.  
    /// If you get the result [`Signal::Uncomputed`], you forgot to call
    /// [`compute_signals()`](Self::compute_signals).  
    /// If you get the result [`Signal::Uncomputable`], somewhere up the chain of inputs
    /// leading to your wire,  
    /// an input is unknown to the circuit, thus leading to a chain of uncomputable signals.  
    /// Returns an error if `id` is not ascii lowercase or if circuit has no such wire.
    pub fn get_signal<S: Into<String>>(&self, id: S) -> Result<Signal> {
        self.get_signal_of(&WireId::new(id)?)
    }

    /// Infallible version of the previous function.
    pub fn signal<S: Into<String>>(&self, id: S) -> Signal {
        self.get_signal(id).unwrap()
    }

    fn get_signal_of(&self, id: &WireId) -> Result<Signal> {
        self.get_wire_of(id).map(|w| *w.signal())
    }

    #[allow(dead_code)]
    fn signal_of(&self, id: &WireId) -> Signal {
        self.get_signal_of(id).unwrap()
    }

    fn set_signal_of(&mut self, id: &WireId, signal: Signal) -> Result<()> {
        self.wires
            .get_mut(id)
            .ok_or(Error::UnknownWireId(id.to_string()))
            .map(|w| {
                w.set_signal(signal);
            })
    }

    /// Computes signals of all wires in the circuit.  
    /// If you add wires after calling this function, you need to call it again to compute
    /// the signals of the new wires  
    /// (and potentially previously uncomputable signals).  
    /// Returns error if the circuit has a loop.
    pub fn compute_signals(&mut self) -> Result<()> {
        let mut to_be_computed = mem::take(&mut self.uncomputable);
        for id in &mut to_be_computed {
            self.set_signal_of(id, Signal::Uncomputed).unwrap();
        }
        to_be_computed.append(&mut self.uncomputed);
        self.compute_signals_of(to_be_computed)?;
        self.uncomputable.sort();
        self.uncomputable.dedup();
        Ok(())
    }

    /// Computes the signal of wire `id`.  
    /// Returns an error if `id` is not ascii lowercase or if the circuit has no such wire.
    pub fn compute_signal<S: Into<String>>(&mut self, id: S) -> Result<Signal> {
        let id = WireId::new(id)?;
        self.compute_signals_of(vec![id.clone()])?;
        self.get_signal_of(&id)
    }

    // Computes the signals of all the wires from ids
    //
    // (Note: For clarity, wires are identified with their ids in this comment)
    //
    // Proceeds by iteration starting from the end of the vector until it is empty
    // 1) Select the last id of the vector and mark it as root if there is none
    // 2a) If that id has an uncomputable signal or does not exist,
    //     mark all the ids from root until the end of the vector as uncomputable,
    //     pop them from the vector and go back to 1)
    // 2b) If that id has a signal, pop it from the vector and go back to 1)
    // 3) Otherwise that id has an uncomputed signal. Look for its inputs
    // 3a) If an input has an uncomputed signal:
    // 3a1) If that input is already in [root..], the circuit has a loop. Error.
    // 3a2) Otherwise add it to the vector and go back to 1)
    // 3b) If an input has an uncomputable signal or does not exist,
    //     mark all the ids from root until the end of the vector as uncomputable,
    //     pop them from the vector and go back to 1) (same as 2a)
    // 3c) Otherwise all inputs have signals.
    //     Deduce the signal of id, pop it from the vector and go back to 1)
    fn compute_signals_of(&mut self, mut ids: Vec<WireId>) -> Result<()> {
        let mut root_index = if ids.is_empty() { 0 } else { ids.len() - 1 };
        while let Some(id) = ids.last() {
            if root_index > ids.len() - 1 {
                root_index = ids.len() - 1;
            }
            if let Some(wire) = self.wires.get(id) {
                match wire.signal() {
                    Signal::Value(_) => {
                        ids.pop();
                    }
                    Signal::Uncomputable => {
                        ids = self.set_uncomputable_from_index(ids, root_index);
                    }
                    Signal::Uncomputed => match wire.input() {
                        WireInput::Value(value) => {
                            self.set_signal_of(id, Signal::Value(*value)).unwrap();
                            ids.pop();
                        }
                        WireInput::Wire(input_id) => {
                            if let Ok(input_wire) = self.get_wire_of(input_id) {
                                match input_wire.signal() {
                                    Signal::Value(signal) => {
                                        self.set_signal_of(id, Signal::Value(*signal)).unwrap();
                                        ids.pop();
                                    }
                                    Signal::Uncomputable => {
                                        ids = self.set_uncomputable_from_index(ids, root_index);
                                    }
                                    Signal::Uncomputed => {
                                        if ids[root_index..].contains(input_id) {
                                            return Err(Error::CircuitLoop);
                                        }
                                        ids.push(input_id.to_owned());
                                    }
                                }
                            } else {
                                ids = self.set_uncomputable_from_index(ids, root_index);
                            }
                        }
                        WireInput::Gate(gate) => match gate {
                            Gate::And { input1, input2 } | Gate::Or { input1, input2 } => {
                                if let (Ok(wire1), Ok(wire2)) =
                                    (self.get_wire_of(input1), self.get_wire_of(input2))
                                {
                                    match (wire1.signal(), wire2.signal()) {
                                        (Signal::Value(signal1), Signal::Value(signal2)) => {
                                            self.set_signal_of(
                                                id,
                                                gate.signal(*signal1, Some(*signal2)),
                                            )
                                            .unwrap();
                                            ids.pop();
                                        }
                                        (Signal::Uncomputable, _) | (_, Signal::Uncomputable) => {
                                            ids = self.set_uncomputable_from_index(ids, root_index);
                                        }
                                        (Signal::Uncomputed, _) => {
                                            if ids[root_index..].contains(input1) {
                                                return Err(Error::CircuitLoop);
                                            }
                                            ids.push(input1.to_owned());
                                        }
                                        (_, Signal::Uncomputed) => {
                                            if ids[root_index..].contains(input2) {
                                                return Err(Error::CircuitLoop);
                                            }
                                            ids.push(input2.to_owned());
                                        }
                                    }
                                } else {
                                    ids = self.set_uncomputable_from_index(ids, root_index);
                                }
                            }
                            Gate::AndValue { input, .. }
                            | Gate::OrValue { input, .. }
                            | Gate::LShift { input, .. }
                            | Gate::RShift { input, .. }
                            | Gate::Not { input } => {
                                if let Ok(input_wire) = self.get_wire_of(input) {
                                    match input_wire.signal() {
                                        Signal::Value(signal) => {
                                            self.set_signal_of(id, gate.signal(*signal, None))
                                                .unwrap();
                                            ids.pop();
                                        }
                                        Signal::Uncomputable => {
                                            ids = self.set_uncomputable_from_index(ids, root_index);
                                        }
                                        Signal::Uncomputed => {
                                            if ids[root_index..].contains(input) {
                                                return Err(Error::CircuitLoop);
                                            }
                                            ids.push(input.to_owned());
                                        }
                                    }
                                } else {
                                    ids = self.set_uncomputable_from_index(ids, root_index);
                                }
                            }
                        },
                    },
                }
            } else {
                ids = self.set_uncomputable_from_index(ids, root_index);
            }
        }
        Ok(())
    }

    // Helper function of compute_signals_of()
    fn set_uncomputable_from_index(
        &mut self,
        mut ids: Vec<WireId>,
        root_index: usize,
    ) -> Vec<WireId> {
        for id in &ids[root_index..] {
            self.set_signal_of(id, Signal::Uncomputable).unwrap();
            self.uncomputable.push(id.to_owned());
        }
        ids.truncate(root_index);
        ids
    }

    /// Prints all signals.  
    /// The implementation of [`Circuit`] uses a [`HashMap`](std::collections::HashMap).
    /// For that reason, the ordering is random.
    pub fn print_signals(&self) {
        for wire in self.wires.values() {
            println!("{}: {:?}", wire.id(), wire.signal());
        }
    }

    /// Reads circuit from a file assuming a wire per line.  
    /// See [example](Circuit#example-1) for how to represent a wire with a string
    /// or use the next function to get clues!
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let s = fs::read_to_string(path)?;
        Self::try_from(s.as_str())
    }

    /// Writes circuit to a file.
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let data = self.to_string();
        let mut f = File::create(path)?;
        Ok(f.write_all(data.as_bytes())?)
    }

    /// Remove wire `id` from circuit then reset all signals (to [`Signal::Uncomputed`]).  
    /// Returns an error if `id` is not ascii lowercase or if circuit has not such wire.
    /// If an error occurs, signals are not reset.
    pub fn remove_wire_then_reset_signals<S: Into<String>>(&mut self, id: S) -> Result<()> {
        let id = WireId::new(id)?;
        self.wires
            .remove(&id)
            .ok_or(Error::UnknownWireId(id.to_string()))
            .map(|_| {
                self.reset_signals();
            })
    }

    #[allow(dead_code)]
    fn set_wire_then_reset_signals(&mut self, wire: Wire) -> Result<()> {
        if let Some(w) = self.wires.get_mut(wire.id()) {
            *w = wire;
            self.reset_signals();
            Ok(())
        } else {
            Err(Error::UnknownWireId(wire.id().to_string()))
        }
    }

    fn reset_signals(&mut self) {
        self.wires
            .values_mut()
            .for_each(|w| w.set_signal(Signal::Uncomputed));
        self.uncomputable = vec![];
        self.uncomputed = self.wires.keys().cloned().collect();
    }

    pub(super) fn set_wires(&mut self, wires: HashMap<WireId, Wire>) {
        self.wires = wires;
    }

    pub(super) fn set_uncomputed(&mut self, uncomputed: Vec<WireId>) {
        self.uncomputed = uncomputed;
    }

    // Tests if both circuits have the same wires (ids, inputs and signals)
    #[allow(dead_code)]
    pub(super) fn equals(&self, other: &Self) -> bool {
        if self.wires.len() != other.wires.len() {
            return false;
        }
        for (id1, wire1) in &self.wires {
            if let Ok(wire2) = other.get_wire_of(id1) {
                if wire1.id() != wire2.id()
                    || wire1.input() != wire2.input()
                    || wire1.signal() != wire2.signal()
                {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

impl TryFrom<&str> for Circuit {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        let mut circuit = Circuit::new();
        for wire in s.trim_end().split('\n') {
            circuit.add(wire.try_into()?)?
        }
        Ok(circuit)
    }
}

impl Display for Circuit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for wire in self.wires.values() {
            writeln!(f, "{}", wire)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_circuit() -> Result<()> {
        let mut circuit = Circuit::new();
        circuit.compute_signals()?;
        Ok(())
    }

    #[test]
    fn conflicting_wires() -> Result<()> {
        let mut circuit = Circuit::new();
        circuit.add_wire_with_value("w", 0)?;
        assert!(matches!(
            circuit.add_wire_with_value("w", 1),
            Err(Error::WireIdAlreadyExists(_))
        ));
        Ok(())
    }

    #[test]
    fn simple_circuit() -> Result<()> {
        let mut circuit = Circuit::new();
        circuit.add_wire_with_value("a", 1)?;
        circuit.add_wire_from_wire("b", "a")?;
        assert!(matches!(
            circuit.get_signal("z"),
            Err(Error::UnknownWireId(_))
        ));
        assert_eq!(circuit.signal("a"), Signal::Uncomputed);
        assert_eq!(circuit.signal("b"), Signal::Uncomputed);

        circuit.compute_signals()?;
        assert_eq!(circuit.signal("a"), Signal::Value(1));
        assert_eq!(circuit.signal("b"), Signal::Value(1));
        Ok(())
    }

    #[test]
    fn nanocorp_1() -> Result<()> {
        let mut circuit = Circuit::new();
        circuit.add_wire_with_value("x", 123)?;
        circuit.add_wire_with_value("y", 456)?;
        circuit.add_gate_and("d", "x", "y")?;
        circuit.add_gate_or("e", "x", "y")?;
        circuit.add_gate_lshift("f", "x", 2)?;
        circuit.add_gate_rshift("g", "y", 2)?;
        circuit.add_gate_not("h", "x")?;
        circuit.add_gate_not("i", "y")?;
        circuit.compute_signals()?;
        println!("{}", circuit);
        circuit.print_signals();

        assert_eq!(circuit.signal("d"), Signal::Value(72));
        assert_eq!(circuit.signal("e"), Signal::Value(507));
        assert_eq!(circuit.signal("f"), Signal::Value(492));
        assert_eq!(circuit.signal("g"), Signal::Value(114));
        assert_eq!(circuit.signal("h"), Signal::Value(65412));
        assert_eq!(circuit.signal("i"), Signal::Value(65079));
        assert_eq!(circuit.signal("x"), Signal::Value(123));
        assert_eq!(circuit.signal("y"), Signal::Value(456));
        Ok(())
    }

    #[test]
    fn nanocorp_1_from_string() -> Result<()> {
        let s = "x AND y -> d\n\
		 NOT x -> h\n\
		 NOT y -> i\n\
		 x OR y -> e\n\
		 y RSHIFT 2 -> g\n\
		 x LSHIFT 2 -> f\n\
		 123 -> x\n\
		 456 -> y";
        let c1 = Circuit::try_from(s)?;

        let mut c2 = Circuit::new();
        c2.add_wire_with_value("x", 123)?;
        c2.add_wire_with_value("y", 456)?;
        c2.add_gate_and("d", "x", "y")?;
        c2.add_gate_or("e", "x", "y")?;
        c2.add_gate_lshift("f", "x", 2)?;
        c2.add_gate_rshift("g", "y", 2)?;
        c2.add_gate_not("h", "x")?;
        c2.add_gate_not("i", "y")?;

        assert!(c1.equals(&c2));
        Ok(())
    }

    #[test]
    fn read_nanocorp_2() -> Result<()> {
        Circuit::read("circuits/nanocorp_2.txt")?;
        Ok(())
    }

    #[test]
    fn write_read_nanocorp_1() -> Result<()> {
        let mut c1 = Circuit::new();
        c1.add_wire_with_value("x", 123)?;
        c1.add_wire_with_value("y", 456)?;
        c1.add_gate_and("d", "x", "y")?;
        c1.add_gate_or("e", "x", "y")?;
        c1.add_gate_lshift("f", "x", 2)?;
        c1.add_gate_rshift("g", "y", 2)?;
        c1.add_gate_not("h", "x")?;
        c1.add_gate_not("i", "y")?;
        c1.write("circuits/nanocorp_1.txt")?;

        let c2 = Circuit::read("circuits/nanocorp_1.txt")?;
        assert!(c1.equals(&c2));
        Ok(())
    }

    #[test]
    fn read_write_read_nanocorp_2() -> Result<()> {
        let c1 = Circuit::read("circuits/nanocorp_2.txt")?;
        c1.write("circuits/nanocorp_2_copy.txt")?;
        let c2 = Circuit::read("circuits/nanocorp_2_copy.txt")?;
        assert!(c1.equals(&c2));
        Ok(())
    }

    #[test]
    fn non_connected_wires() -> Result<()> {
        let mut c = Circuit::new();
        c.add_wire_with_value("x", 0xfff0)?;
        c.add_wire_with_value("y", 0x0fff)?;
        c.add_gate_or("xoy", "x", "y")?;
        c.add_gate_and("xoyau", "xoy", "unknown")?;
        c.add_gate_lshift("u", "unknown", 2)?;
        c.add_gate_rshift("v", "unknown", 2)?;
        c.add_gate_not("nxoy", "xoy")?;
        c.add_gate_not("w", "unknown")?;
        c.compute_signals()?;

        assert_eq!(c.signal("x"), Signal::Value(0xfff0));
        assert_eq!(c.signal("y"), Signal::Value(0x0fff));
        assert_eq!(c.signal("xoy"), Signal::Value(0xffff));
        assert_eq!(c.signal("nxoy"), Signal::Value(0x0));
        assert_eq!(c.signal("u"), Signal::Uncomputable);
        assert_eq!(c.signal("v"), Signal::Uncomputable);
        assert_eq!(c.signal("w"), Signal::Uncomputable);
        assert_eq!(c.signal("xoyau"), Signal::Uncomputable);
        assert!(matches!(
            c.get_signal("unknown"),
            Err(Error::UnknownWireId(_))
        ));
        Ok(())
    }

    #[test]
    fn identical_gate_inputs() -> Result<()> {
        let x = 0xa35c;
        let mut c = Circuit::new();
        c.add_wire_with_value("x", x)?;
        c.add_gate_or("xox", "x", "x")?;
        c.add_gate_and("xax", "x", "x")?;
        c.compute_signals()?;

        assert_eq!(c.signal("x"), Signal::Value(x));
        assert_eq!(c.signal("xox"), Signal::Value(x));
        assert_eq!(c.signal("xax"), Signal::Value(x));
        Ok(())
    }

    #[test]
    fn loop_2_wires() -> Result<()> {
        let mut c = Circuit::new();
        c.add_wire_from_wire("a", "b")?;
        c.add_wire_from_wire("b", "a")?;
        assert!(c.compute_signals().is_err());
        Ok(())
    }

    #[test]
    fn loop_3_wires() -> Result<()> {
        let mut c = Circuit::new();
        c.add_wire_from_wire("a", "b")?;
        c.add_gate_and("b", "c", "d")?;
        c.add_gate_or("c", "e", "f")?;
        c.add_gate_not("f", "b")?;
        c.add_wire_with_value("d", 19)?;
        c.add_wire_with_value("e", 7)?;
        assert!(c.compute_signals().is_err());
        Ok(())
    }

    #[test]
    fn compute_signals_then_add_wire() -> Result<()> {
        let mut c = Circuit::new();
        c.add_wire_with_value("b", 0x10)?;
        c.add_wire_with_value("c", 0x100)?;
        c.add_gate_or("aob", "a", "b")?;
        c.add_gate_or("boc", "b", "c")?;
        c.add_gate_or("cod", "c", "d")?;
        c.add_gate_and("x", "aob", "boc")?;
        c.add_gate_and("y", "boc", "cod")?;
        c.add_gate_or("z", "x", "y")?;
        c.add_gate_not("nz", "z")?;
        c.compute_signals()?;

        assert!(matches!(c.get_signal("a"), Err(Error::UnknownWireId(_))));
        assert!(matches!(c.get_signal("d"), Err(Error::UnknownWireId(_))));

        assert_eq!(c.signal("b"), Signal::Value(0x10));
        assert_eq!(c.signal("c"), Signal::Value(0x100));
        assert_eq!(c.signal("boc"), Signal::Value(0x110));

        assert_eq!(c.signal("aob"), Signal::Uncomputable);
        assert_eq!(c.signal("cod"), Signal::Uncomputable);
        assert_eq!(c.signal("x"), Signal::Uncomputable);
        assert_eq!(c.signal("y"), Signal::Uncomputable);
        assert_eq!(c.signal("z"), Signal::Uncomputable);
        assert_eq!(c.signal("nz"), Signal::Uncomputable);
        assert_eq!(c.uncomputable.len(), 6);

        c.add_wire_with_value("a", 0x1)?;
        c.add_wire_with_value("d", 0x1000)?;
        c.compute_signals()?;

        assert_eq!(c.signal("a"), Signal::Value(0x1));
        assert_eq!(c.signal("b"), Signal::Value(0x10));
        assert_eq!(c.signal("c"), Signal::Value(0x100));
        assert_eq!(c.signal("d"), Signal::Value(0x1000));
        assert_eq!(c.signal("aob"), Signal::Value(0x11));
        assert_eq!(c.signal("boc"), Signal::Value(0x110));
        assert_eq!(c.signal("cod"), Signal::Value(0x1100));
        assert_eq!(c.signal("x"), Signal::Value(0x10));
        assert_eq!(c.signal("y"), Signal::Value(0x100));
        assert_eq!(c.signal("z"), Signal::Value(0x110));
        assert_eq!(c.signal("nz"), Signal::Value(0xfeef));
        Ok(())
    }
}
