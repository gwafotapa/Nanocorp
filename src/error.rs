use std::{io, num, result};

use thiserror;

use crate::wire_id::WireId;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Wire id '{0}' is not ascii lowercase")]
    InvalidWireId(String),

    #[error("Shift amount '{0}' exceeds 15")]
    TooLargeShift(u8),

    #[error("Cannot parse string '{0}' as a gate")]
    ParseGate(String),

    #[error("Cannot parse gate shift from string '{0}'")]
    ParseShift(#[from] num::ParseIntError),

    #[error("String {0} has no arrow ' -> '")]
    ParseArrow(String),

    #[error("Circuit already has a wire whose id is '{0}'")]
    WireIdAlreadyExists(WireId),

    #[error("Circuit has no wire '{0}'")]
    UnknownWireId(WireId),

    #[error("Identical input and output ids '{0}'")]
    InputMatchesOutput(WireId),

    #[error(transparent)]
    InvalidPath(#[from] io::Error),

    #[error("Circuit has a loop")]
    CircuitLoop,
}
