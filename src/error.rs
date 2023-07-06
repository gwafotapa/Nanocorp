use std::{io, num};

use thiserror;

use crate::wire::WireId;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Wire id '{0}' is not ascii lowercase")]
    InvalidWireId(WireId),

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

    #[error(transparent)]
    InvalidPath(#[from] io::Error),
}
