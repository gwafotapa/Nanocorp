// #![warn(missing_docs)]

//! Abstraction of a circuit of wires and gates
//!
//! Provides an abstraction over a circuit of wires transmitting signals.
//! Each wire emits a signal coming from a source which can be:
//! - a value
//! - another wire
//! - a gate combining other wires
//!
//! # Example
//!
//! Here's a simple circuit composed of three wires: "a", "b" and "ab".
//! Inputs of wires "a" and "b" are values.
//! A logical AND gate connects the three wires, taking wires "a" and "b" as inputs
//! and outputting to wire "ab".
//!
//! ```
//! # use circuitry::{CircuitBuilder, Signal, Error};
//! # fn main() -> Result<(), Error> {
//! let mut circuit = CircuitBuilder::new()
//!     .add_wire_with_value("a", 1729)?
//!     .add_wire_with_value("b", 4936)?
//!     .add_gate_and("ab", "a", "b")?
//!     .build();
//!
//! circuit.compute_signals();
//!
//! assert_eq!(circuit.signal("a"), Signal::Value(1729));
//! assert_eq!(circuit.signal("b"), Signal::Value(4936));
//! assert_eq!(circuit.signal("ab"), Signal::Value(1729 & 4936));
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
