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
    pub fn new_wire_with_value(id: impl Into<String>, value: u16) -> Self {
        Self {
            id: id.into(),
            kind: ComponentKind::Wire {
                source: WireSource::Value(value),
            },
            signal: None,
        }
    }

    pub fn new_wire_from_component(id: impl Into<String>, input: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            kind: ComponentKind::Wire {
                source: WireSource::Id(input.into()),
            },
            signal: None,
        }
    }

    pub fn new_wire(id: impl Into<String>, source: WireSource) -> Self {
        Self {
            id: id.into(),
            kind: ComponentKind::Wire { source },
            signal: None,
        }
    }

    pub fn new_gate_and(
        id: impl Into<String>,
        source1: impl Into<String>,
        source2: impl Into<String>,
    ) -> Component {
        Self {
            id: id.into(),
            kind: ComponentKind::GateAnd {
                source1: source1.into(),
                source2: source2.into(),
            },
            signal: None,
        }
    }

    pub fn new_gate_or(
        id: impl Into<String>,
        source1: impl Into<String>,
        source2: impl Into<String>,
    ) -> Component {
        Self {
            id: id.into(),
            kind: ComponentKind::GateOr {
                source1: source1.into(),
                source2: source2.into(),
            },
            signal: None,
        }
    }

    pub fn new_gate_sll(id: impl Into<String>, source: impl Into<String>, shift: u8) -> Component {
        Self {
            id: id.into(),
            kind: ComponentKind::GateSLL {
                source: source.into(),
                shift,
            },
            signal: None,
        }
    }

    pub fn new_gate_slr(id: impl Into<String>, source: impl Into<String>, shift: u8) -> Component {
        Self {
            id: id.into(),
            kind: ComponentKind::GateSLR {
                source: source.into(),
                shift,
            },
            signal: None,
        }
    }

    pub fn new_gate_not(id: impl Into<String>, source: impl Into<String>) -> Component {
        Self {
            id: id.into(),
            kind: ComponentKind::GateNot {
                source: source.into(),
            },
            signal: None,
        }
    }
}
