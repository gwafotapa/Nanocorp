// use crate::{signal::Signal, wire::Wire};
use crate::wire::Wire;

#[derive(Clone, Copy)]
pub enum Gate<'a> {
    And {
        wire1: &'a Wire<'a>,
        wire2: &'a Wire<'a>,
        signal: Option<u16>,
    },
    Or {
        wire1: &'a Wire<'a>,
        wire2: &'a Wire<'a>,
        signal: Option<u16>,
    },
    SLL {
        wire: &'a Wire<'a>,
        shift: u8,
        signal: Option<u16>,
    },
    SLR {
        wire: &'a Wire<'a>,
        shift: u8,
        signal: Option<u16>,
    },
    Not {
        wire: &'a Wire<'a>,
        signal: Option<u16>,
    },
}

impl<'a> Gate<'a> {
    pub fn and(wire1: &'a Wire, wire2: &'a Wire) -> Self {
        Self::And {
            wire1,
            wire2,
            signal: None,
        }
    }

    pub fn or(wire1: &'a Wire, wire2: &'a Wire) -> Self {
        Self::Or {
            wire1,
            wire2,
            signal: None,
        }
    }

    pub fn sll(wire: &'a Wire, shift: u8) -> Option<Self> {
        if shift < 16 {
            Some(Self::SLL {
                wire,
                shift,
                signal: None,
            })
        } else {
            None
        }
    }

    pub fn slr(wire: &'a Wire, shift: u8) -> Option<Self> {
        if shift < 16 {
            Some(Self::SLR {
                wire,
                shift,
                signal: None,
            })
        } else {
            None
        }
    }

    pub fn not(wire: &'a Wire) -> Self {
        Self::Not { wire, signal: None }
    }
}

// impl<'a> Signal for Gate<'a> {
//     fn signal(&self) -> Option<u16> {
//         match self {
//             Self::And(w1, w2) => {
//                 if let (Some(signal1), Some(signal2)) = (w1.signal(), w2.signal()) {
//                     Some(signal1 & signal2)
//                 } else {
//                     None
//                 }
//             }
//             Self::Or(w1, w2) => {
//                 if let (Some(signal1), Some(signal2)) = (w1.signal(), w2.signal()) {
//                     Some(signal1 | signal2)
//                 } else {
//                     None
//                 }
//             }
//             Self::SLL(w, shift) => w.signal().map(|s| s << shift),
//             Self::SLR(w, shift) => w.signal().map(|s| s >> shift),
//             Self::Not(w) => w.signal().map(|s| !s),
//         }
//     }
// }
