use crate::{signal::Signal, wire::Wire};

struct GateSLL<'a> {
    wire: &'a Wire,
    shift: u8,
    // signal: Option<u16>,
}

impl<'a> GateSLL<'a> {
    fn new(wire: &'a Wire, shift: u8) -> Option<Self> {
        if shift < 16 {
            Some(Self {
                wire,
                shift,
                // signal: None,
            })
        } else {
            None
        }
    }
}

impl Signal for GateSLL<'_> {
    fn signal(&self) -> Option<u16> {
        self.wire.signal.map(|s| s << self.shift)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_and() {
        let w = Wire::new("a", Some(0x7)).unwrap();
        let sll_w = GateSLL::new(&w, 5).unwrap();
        assert_eq!(sll_w.signal(), Some(0xe0));
    }
}
