use std::{io, num::ParseIntError, result};

use thiserror;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// This string does not qualify as a wire id because it is not ascii lowercase
    #[error("Wire id '{0}' is not ascii lowercase")]
    InvalidWireId(String),

    /// This value is too large to be represented by type [u16]
    #[error("Type u16 cannot represent value '{0}'")]
    TooLargeValue(u64),

    /// A wire cannot be an input to itself
    #[error("Identical input and output ids '{0}'")]
    InputMatchesOutput(String),

    /// This shift amount exceeds 15
    /// (which is the maximum since a signal is represented by type [u16])
    #[error("Shift amount '{0}' exceeds 15")]
    TooLargeShift(u8),

    /// The circuit already has a wire with this id
    #[error("Circuit already has a wire whose id is '{0}'")]
    WireIdAlreadyExists(String),

    /// The circuit has no wire with this id
    #[error("Circuit has no wire '{0}'")]
    UnknownWireId(String),

    /// The circuit has a loop
    #[error("Circuit has a loop")]
    CircuitLoop,

    /// This string cannot be parsed as a gate
    #[error("Cannot parse string '{0}' as a gate")]
    ParseGate(String),

    /// The gate shift could not be parsed
    #[error("Cannot parse gate shift from string '{0}'")]
    ParseShift(#[from] ParseIntError),

    /// This string representation of a wire is missing the arrow symbol ' -> '
    #[error("String {0} has no arrow ' -> '")]
    ParseArrow(String),

    /// [std::io::Error]
    #[error(transparent)]
    IOError(#[from] io::Error),
}
