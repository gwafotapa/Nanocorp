// TODO: Check implems of debug, clone, default, partialeq and send/sync
// https://www.youtube.com/watch?v=Nzclc6MswaI
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Signal {
    Uncomputed,
    Uncomputable,
    Value(u16),
}
