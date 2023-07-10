use super::{gate::Gate, wire_id::WireId};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum WireInput {
    Value(u16),
    Wire(WireId),
    Gate(Gate),
}
