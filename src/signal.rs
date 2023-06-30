pub trait Signal {
    fn signal(&self) -> Option<u16>;
}
