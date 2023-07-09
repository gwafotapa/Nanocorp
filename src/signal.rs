#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Signal {
    Uncomputed,
    Uncomputable,
    Value(u16),
}
