use crate::{signal::Signal, wire::Wire};

pub struct GateOr<'a> {
    wire1: &'a Wire<'a>,
    wire2: &'a Wire<'a>,
    // signal: Option<u16>,
}

impl<'a> GateOr<'a> {
    pub fn new(wire1: &'a Wire, wire2: &'a Wire) -> Self {
        Self {
            wire1,
            wire2,
            // signal: None,
        }
    }
}

impl Signal for GateOr<'_> {
    fn signal(&self) -> Option<u16> {
        if let (Some(signal1), Some(signal2)) = (self.wire1.signal(), self.wire2.signal()) {
            Some(signal1 | signal2)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_or() {
        let w1 = Wire::source_value("a", 0x7).unwrap();
        let w2 = Wire::source_value("b", 0xe).unwrap();
        let w1_a_w2 = GateOr::new(&w1, &w2);
        assert_eq!(w1_a_w2.signal(), Some(0xf));
    }
}
