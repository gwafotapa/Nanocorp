#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Signal {
    #[default]
    Uncomputed,
    Uncomputable,
    Value(u16),
}
