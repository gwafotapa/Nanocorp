use std::{io, num::ParseIntError};

use thiserror::Error;

use crate::wire::WireId;

#[derive(Error, Debug)]
#[error("Wire id '{0}' is not ascii lowercase")]
pub struct WireIdError(pub WireId);

#[derive(Error, Debug)]
pub enum GateError {
    #[error(transparent)]
    InvalidId(#[from] WireIdError),

    #[error("Shift amount '{0}' exceeds 15")]
    TooLargeShift(u8),
}

#[derive(Error, Debug)]
pub enum WireError {
    #[error(transparent)]
    InvalidId(#[from] WireIdError),

    #[error(transparent)]
    InvalidGate(#[from] GateError),
}

#[derive(Error, Debug)]
pub enum CircuitError {
    #[error("Circuit already has a wire whose id is '{0}'")]
    ConflictingWireId(WireId),

    #[error(transparent)]
    InvalidGate(#[from] GateError),

    #[error(transparent)]
    InvalidWire(#[from] WireError),
}

#[derive(Error, Debug)]
pub enum ParseGateError {
    #[error("Cannot parse string '{0}' as a gate")]
    UnknownGate(String),

    #[error(transparent)]
    InvalidInput(#[from] GateError),

    #[error(transparent)]
    ParseShift(#[from] ParseIntError),
}

#[derive(Error, Debug)]
pub enum ParseWireError {
    #[error("String {0} has no arrow ' -> '")]
    MissingArrow(String),

    #[error(transparent)]
    ParseGate(#[from] ParseGateError),

    #[error(transparent)]
    InvalidWire(#[from] WireError),
}

#[derive(Error, Debug)]
pub enum ParseCircuitError {
    #[error(transparent)]
    InvalidPath(#[from] io::Error),

    #[error(transparent)]
    ParseWire(#[from] ParseWireError),

    #[error(transparent)]
    InvalidCircuit(#[from] CircuitError),
}
