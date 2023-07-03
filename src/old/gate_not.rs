use crate::{signal::Signal, wire::Wire};

pub struct GateNot<'a> {
    wire: &'a Wire<'a>,
    // signal: Option<u16>,
}

impl<'a> GateNot<'a> {
    pub fn new(wire: &'a Wire) -> Self {
        Self {
            wire,
            // signal: None,
        }
    }
}

impl Signal for GateNot<'_> {
    fn signal(&self) -> Option<u16> {
        self.wire.signal().map(|s| !s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_or() {
        let w = Wire::source_value("a", 0x3).unwrap();
        let not_w = GateNot::new(&w);
        assert_eq!(not_w.signal(), Some(0xfffc));
    }
}