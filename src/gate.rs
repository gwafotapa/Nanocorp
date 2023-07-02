// use crate::{signal::Signal, wire::Wire};
// use crate::wire::Wire;

pub enum Gate {
    And {
        wire1: String,
        wire2: String,
        // signal: Option<u16>,
    },
    Or {
        wire1: String,
        wire2: String,
        // signal: Option<u16>,
    },
    SLL {
        wire: String,
        shift: u8,
        // signal: Option<u16>,
    },
    SLR {
        wire: String,
        shift: u8,
        // signal: Option<u16>,
    },
    Not {
        wire: String,
        // signal: Option<u16>,
    },
}

impl Gate {
    pub fn and(wire1: impl Into<String>, wire2: impl Into<String>) -> Option<Self> {
        let wire1 = wire1.into();
        let wire2 = wire2.into();
        (wire1.bytes().all(|b| b.is_ascii_lowercase())
            && wire2.bytes().all(|b| b.is_ascii_lowercase()))
        .then_some(Self::And {
            wire1,
            wire2,
            // signal: None,
        })
    }

    pub fn or(wire1: impl Into<String>, wire2: impl Into<String>) -> Option<Self> {
        let wire1 = wire1.into();
        let wire2 = wire2.into();
        (wire1.bytes().all(|b| b.is_ascii_lowercase())
            && wire2.bytes().all(|b| b.is_ascii_lowercase()))
        .then_some(Self::Or {
            wire1,
            wire2,
            // signal: None,
        })
    }

    pub fn sll(wire: impl Into<String>, shift: u8) -> Option<Self> {
        let wire = wire.into();
        (wire.bytes().all(|b| b.is_ascii_lowercase()) && shift < 16).then_some(Self::SLL {
            wire,
            shift,
            // signal: None,
        })
    }

    pub fn slr(wire: impl Into<String>, shift: u8) -> Option<Self> {
        let wire = wire.into();
        (wire.bytes().all(|b| b.is_ascii_lowercase()) && shift < 16).then_some(Self::SLR {
            wire,
            shift,
            // signal: None,
        })
    }

    pub fn not(wire: impl Into<String>) -> Option<Self> {
        let wire = wire.into();
        wire.bytes()
            .all(|b| b.is_ascii_lowercase())
            .then_some(Self::Not { wire }) //signal: None })
    }
}

// impl Signal for Gate {
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
