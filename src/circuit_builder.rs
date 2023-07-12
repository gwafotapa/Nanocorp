use std::{collections::HashMap, mem};

use super::{
    wire::{wire_id::WireId, Wire},
    Circuit,
};
use crate::error::{Error, Result};

/// A builder for [`Circuit`]
///
/// [`CircuitBuilder`] has methods named after those of [`Circuit`] for adding wires.
///
/// # Example
///
/// The circuit below tests if x is greater than 32767
/// (returning 1 if it is true and 0 if it is not).  
/// In this example we test number 32768.
/// ```
/// # use circuitry::{CircuitBuilder, Signal, Error};
/// # fn main() -> Result<(), Error> {
/// let mut is_greater_than_32767 = CircuitBuilder::new()
///     .add_wire_with_value("x", 0x8000)?  // Adds wire x emitting 32768
///     .add_gate_rshift("y", "x", 15)?     // Adds wire y emitting x >> 15
///     .add_gate_and_value("res", "y", 1)? // Adds wire res emitting y & 1
///     .build();
///
/// is_greater_than_32767.compute_signals()?;
/// assert_eq!(is_greater_than_32767.signal("res"), Signal::Value(1));
/// # Ok(())
/// # }
/// ```

/// You can also use method [`add_wire()`](Self::add_wire)
/// with string representation if you prefer.
/// See [example](Circuit#example-1) for usage.
#[derive(Clone, Debug, Default)]
pub struct CircuitBuilder {
    wires: HashMap<WireId, Wire>,
}

impl CircuitBuilder {
    /// Creates an empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Final call method building the circuit from the builder.
    pub fn build(&mut self) -> Circuit {
        let mut circuit = Circuit::new();
        circuit.set_wires(mem::take(&mut self.wires));
        circuit.set_uncomputed(circuit.get_wires().keys().cloned().collect());
        circuit
    }

    fn add(&mut self, wire: Wire) -> Result<&mut CircuitBuilder> {
        if self.wires.contains_key(wire.id()) {
            Err(Error::WireIdAlreadyExists(wire.id().to_string()))
        } else {
            self.wires.insert(wire.id().to_owned(), wire);
            Ok(self)
        }
    }

    /// Adds a wire whose string representation is `s`.
    /// See [example](Circuit#example-1) for usage.
    pub fn add_wire(&mut self, s: &str) -> Result<&mut CircuitBuilder> {
        self.add(Wire::try_from(s)?)
    }

    /// Equivalent of [`Circuit::add_wire_with_value`].
    pub fn add_wire_with_value<S: Into<String>>(
        &mut self,
        id: S,
        value: u16,
    ) -> Result<&mut CircuitBuilder> {
        self.add(Wire::with_value(id, value)?)
    }

    /// Equivalent of [`Circuit::add_wire_from_wire`].
    pub fn add_wire_from_wire<S: Into<String>, T: Into<String>>(
        &mut self,
        id: S,
        input_id: T,
    ) -> Result<&mut CircuitBuilder> {
        self.add(Wire::from_wire(id, input_id)?)
    }

    /// Equivalent of [`Circuit::add_gate_and`].
    pub fn add_gate_and<S: Into<String>, T: Into<String>, U: Into<String>>(
        &mut self,
        output: S,
        input1: T,
        input2: U,
    ) -> Result<&mut CircuitBuilder> {
        self.add(Wire::from_gate_and(output, input1, input2)?)
    }

    /// Equivalent of [`Circuit::add_gate_and_value`].
    pub fn add_gate_and_value<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        value: u16,
    ) -> Result<&mut CircuitBuilder> {
        self.add(Wire::from_gate_and_value(output, input, value)?)
    }

    /// Equivalent of [`Circuit::add_gate_or`].
    pub fn add_gate_or<S: Into<String>, T: Into<String>, U: Into<String>>(
        &mut self,
        output: S,
        input1: T,
        input2: U,
    ) -> Result<&mut CircuitBuilder> {
        self.add(Wire::from_gate_or(output, input1, input2)?)
    }

    /// Equivalent of [`Circuit::add_gate_or_value`].
    pub fn add_gate_or_value<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        value: u16,
    ) -> Result<&mut CircuitBuilder> {
        self.add(Wire::from_gate_or_value(output, input, value)?)
    }

    /// Equivalent of [`Circuit::add_gate_lshift`].
    pub fn add_gate_lshift<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        shift: u8,
    ) -> Result<&mut CircuitBuilder> {
        self.add(Wire::from_gate_lshift(output, input, shift)?)
    }

    /// Equivalent of [`Circuit::add_gate_rshift`].
    pub fn add_gate_rshift<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
        shift: u8,
    ) -> Result<&mut CircuitBuilder> {
        self.add(Wire::from_gate_rshift(output, input, shift)?)
    }

    /// Equivalent of [`Circuit::add_gate_not`].
    pub fn add_gate_not<S: Into<String>, T: Into<String>>(
        &mut self,
        output: S,
        input: T,
    ) -> Result<&mut CircuitBuilder> {
        self.add(Wire::from_gate_not(output, input)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_liner() -> Result<()> {
        let c1 = CircuitBuilder::new()
            .add_wire_with_value("x", 123)?
            .add_wire_with_value("y", 456)?
            .add_gate_and("d", "x", "y")?
            .add_gate_or("e", "x", "y")?
            .add_gate_lshift("f", "x", 2)?
            .add_gate_rshift("g", "y", 2)?
            .add_gate_not("h", "x")?
            .add_gate_not("i", "y")?
            .build();

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
    fn complex_configuration() -> Result<()> {
        let mut builder = CircuitBuilder::new();
        builder.add_wire_with_value("x", 123)?;
        builder.add_wire_with_value("y", 456)?;
        builder.add_gate_and("d", "x", "y")?;
        builder.add_gate_or("e", "x", "y")?;
        builder.add_gate_lshift("f", "x", 2)?;
        builder.add_gate_rshift("g", "y", 2)?;
        builder.add_gate_not("h", "x")?;
        builder.add_gate_not("i", "y")?;
        let c1 = builder.build();

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
}
