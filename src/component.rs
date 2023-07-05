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
        input: ComponentInput,
    },
    GateAnd {
        input1: ComponentInput,
        input2: ComponentInput,
    },
    GateOr {
        input1: ComponentInput,
        input2: ComponentInput,
    },
    GateNot {
        input: ComponentInput,
    },
    GateSLL {
        // TODO: GateLShift ?
        input: ComponentInput,
        shift: u8,
    },
    GateSLR {
        input: ComponentInput,
        shift: u8,
    },
}

pub enum ComponentInput {
    Value(u16),
    Id(ComponentId),
}

impl Component {
    // pub fn new(id: S, kind: ComponentKind) -> Result<Self, Error> {
    // 	let id = id.into();
    // }

    pub fn wire_with_value(id: S, value: u16) -> Result<Self, Error> {
        Self::wire(id, ComponentInput::Value(value))
    }

    pub fn wire_from_component(
        id: S,
        input_id: S,
    ) -> Result<Self, Error> {
        Self::wire(id, ComponentInput::Id(input_id.into()))
    }

    pub fn wire(id: S, input: ComponentInput) -> Result<Self, Error> {
        let id = id.into();
        if id.bytes().all(|b| b.is_ascii_lowercase()) {
            Ok(Self {
                id: id.into(),
                kind: ComponentKind::Wire { input },
                signal: None,
            })
        } else {
            Err(Error::WrongFormatId(id))
        }
    }

    pub fn gate_and(
        id: S,
        input1: S,
        input2: S,
    ) -> Result<Self, Error> {
        let id = id.into();
        if id.bytes().all(|b| b.is_ascii_uppercase()) {
            Ok(Self {
                id,
                kind: ComponentKind::GateAnd {
                    input1: input1.into(),
                    input2: input2.into(),
                },
                signal: None,
            })
        } else {
            Err(Error::WrongFormatId(id))
        }
    }

    pub fn gate_or(
        id: S,
        input1: S,
        input2: S,
    ) -> Result<Self, Error> {
        let id = id.into();
        if id.bytes().all(|b| b.is_ascii_uppercase()) {
            Ok(Self {
                id,
                kind: ComponentKind::GateOr {
                    input1: input1.into(),
                    input2: input2.into(),
                },
                signal: None,
            })
        } else {
            Err(Error::WrongFormatId(id))
        }
    }

    pub fn gate_sll(
        id: S,
        input: S,
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
                    input: input.into(),
                    shift,
                },
                signal: None,
            })
        }
    }

    pub fn gate_slr(
        id: S,
        input: S,
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
                    input: input.into(),
                    shift,
                },
                signal: None,
            })
        }
    }

    pub fn gate_not(id: S, input: S) -> Result<Self, Error> {
        let id = id.into();
        if id.bytes().all(|b| b.is_ascii_uppercase()) {
            Ok(Self {
                id,
                kind: ComponentKind::GateNot {
                    input: input.into(),
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
            ComponentKind::Wire { input } => {
                write!(f, "{} -> {}", input, self.id)
            }
            ComponentKind::GateAnd { input1, input2 } => {}
            ComponentKind::GateOr { input1, input2 } => {}
            ComponentKind::GateSLL { input, shift } => {}
            ComponentKind::GateSLR { input, shift } => {}
            ComponentKind::GateNot { input } => {}
        }
    }
}

impl fmt::Display for ComponentInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComponentInput::Value(value) => {
                write!(f, "{}", value)
            }
            ComponentInput::Id(id) => {
                write!(f, "{}", id)
            }
        }
    }
}
