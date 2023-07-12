/// The signal transmitted by a wire.
///
/// See [here](crate::Circuit::get_signal) for more details.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Signal {
    #[default]
    Uncomputed,
    Uncomputable,
    Value(u16),
}
