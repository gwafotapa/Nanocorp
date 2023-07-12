// #![warn(missing_docs)]

//! Abstraction of a circuit of wires connected by logical gates
//!
//! Provides an abstraction over a circuit of wires transmitting signals.
//! Each wire emits a signal coming from a source which can be:
//! - a value
//! - another wire
//! - a gate combining the signals of other wires
//!
//! # Example
//!
//! Here's a simple circuit composed of 3 wires: "a", "b" and "ab".
//! Wires "a" and "b" are sourced using values whereas the signal of "ab"
//! is the logical AND of the signals emitted by "a" and "b".
//!
//! ```
//! # use circuitry::{CircuitBuilder, Signal, Error};
//! # fn main() -> Result<(), Error> {
//! let mut circuit = CircuitBuilder::new()
//!     .add_wire_with_value("a", 0x03ff)?
//!     .add_wire_with_value("b", 0xff50)?
//!     .add_gate_and("ab", "a", "b")?
//!     .build();
//!
//! circuit.compute_signals();
//!
//! assert_eq!(circuit.signal("a"), Signal::Value(0x03ff));
//! assert_eq!(circuit.signal("b"), Signal::Value(0xff50));
//! assert_eq!(circuit.signal("ab"), Signal::Value(0x0350));
//! # Ok(())
//! # }
//! ```

// Dependency reexports
pub use thiserror;

pub use circuit::Circuit;
pub use circuit_builder::CircuitBuilder;
pub use error::Error;
pub use wire::signal::Signal;

#[doc(hidden)]
pub mod circuit;
#[doc(hidden)]
pub mod circuit_builder;
pub mod error;
mod wire;
