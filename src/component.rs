use std::fmt;

use crate::error::Error;

pub type ComponentId = String;

pub struct Component {
    pub id: ComponentId,
    pub kind: ComponentKind,
    pub signal: Option<u16>,
}

pub enum ComponentKind {
    Wire {
        source: WireSource,
    },
    GateAnd {
        source1: ComponentId,
        source2: ComponentId,
    },
    GateOr {
        source1: ComponentId,
        source2: ComponentId,
    },
    GateNot {
        source: ComponentId,
    },
    GateSLL {
        // TODO: GateLShift ?
        source: ComponentId,
        shift: u8,
    },
    GateSLR {
        source: ComponentId,
        shift: u8,
    },
}

pub enum WireSource {
    Value(u16),
    Id(ComponentId),
}

impl Component {
    pub fn new_wire_with_value(id: impl Into<String>, value: u16) -> Result<Self, Error> {
        Self::new_wire(id, WireSource::Value(value))
    }

    pub fn new_wire_from_component(
        id: impl Into<String>,
        source_id: impl Into<String>,
    ) -> Result<Self, Error> {
        Self::new_wire(id, WireSource::Id(source_id.into()))
    }

    pub fn new_wire(id: impl Into<String>, source: WireSource) -> Result<Self, Error> {
        let id = id.into();
        if id.bytes().all(|b| b.is_ascii_lowercase()) {
            Ok(Self {
                id: id.into(),
                kind: ComponentKind::Wire { source },
                signal: None,
            })
        } else {
            Err(Error::WrongFormatId(id))
        }
    }

    pub fn new_gate_and(
        id: impl Into<String>,
        source1: impl Into<String>,
        source2: impl Into<String>,
    ) -> Result<Self, Error> {
        let id = id.into();
        if id.bytes().all(|b| b.is_ascii_uppercase()) {
            Ok(Self {
                id,
                kind: ComponentKind::GateAnd {
                    source1: source1.into(),
                    source2: source2.into(),
                },
                signal: None,
            })
        } else {
            Err(Error::WrongFormatId(id))
        }
    }

    pub fn new_gate_or(
        id: impl Into<String>,
        source1: impl Into<String>,
        source2: impl Into<String>,
    ) -> Result<Self, Error> {
        let id = id.into();
        if id.bytes().all(|b| b.is_ascii_uppercase()) {
            Ok(Self {
                id,
                kind: ComponentKind::GateOr {
                    source1: source1.into(),
                    source2: source2.into(),
                },
                signal: None,
            })
        } else {
            Err(Error::WrongFormatId(id))
        }
    }

    pub fn new_gate_sll(
        id: impl Into<String>,
        source: impl Into<String>,
        shift: u8,
    ) -> Result<Self, Error> {
        let id = id.into();
        if !id.bytes().all(|b| b.is_ascii_uppercase()) {
            Err(Error::WrongFormatId(id))
        } else if shift > 15 {
            Err(Error::ShiftTooLarge(shift))
        } else {
            Ok(Self {
                id,
                kind: ComponentKind::GateSLL {
                    source: source.into(),
                    shift,
                },
                signal: None,
            })
        }
    }

    pub fn new_gate_slr(
        id: impl Into<String>,
        source: impl Into<String>,
        shift: u8,
    ) -> Result<Self, Error> {
        let id = id.into();
        if !id.bytes().all(|b| b.is_ascii_uppercase()) {
            Err(Error::WrongFormatId(id))
        } else if shift > 15 {
            Err(Error::ShiftTooLarge(shift))
        } else {
            Ok(Self {
                id,
                kind: ComponentKind::GateSLR {
                    source: source.into(),
                    shift,
                },
                signal: None,
            })
        }
    }

    pub fn new_gate_not(id: impl Into<String>, source: impl Into<String>) -> Result<Self, Error> {
        let id = id.into();
        if id.bytes().all(|b| b.is_ascii_uppercase()) {
            Ok(Self {
                id,
                kind: ComponentKind::GateNot {
                    source: source.into(),
                },
                signal: None,
            })
        } else {
            Err(Error::WrongFormatId(id))
        }
    }
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ComponentKind::Wire { source } => {
                write!(f, "{} -> {}", source, self.id)
            }
            ComponentKind::GateAnd { source1, source2 } => {}
            ComponentKind::GateOr { source1, source2 } => {}
            ComponentKind::GateSLL { source, shift } => {}
            ComponentKind::GateSLR { source, shift } => {}
            ComponentKind::GateNot { source } => {}
        }
    }
}

impl fmt::Display for WireSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WireSource::Value(value) => {
                write!(f, "{}", value)
            }
            WireSource::Id(id) => {
                write!(f, "{}", id)
            }
        }
    }
}
