use crate::{error::Error, wire::WireId};

pub enum Gate {
    And { wire1: WireId, wire2: WireId },
    Or { wire1: WireId, wire2: WireId },
    SLL { wire: WireId, shift: u8 },
    SLR { wire: WireId, shift: u8 },
    Not { wire: WireId },
}

impl Gate {
    pub fn and(wire1: impl Into<String>, wire2: impl Into<String>) -> Result<Self, Error> {
        let wire1 = wire1.into();
        let wire2 = wire2.into();
        if !wire1.bytes().all(|b| b.is_ascii_lowercase()) {
            Err(Error::WrongFormatId(wire1))
        } else if !wire2.bytes().all(|b| b.is_ascii_lowercase()) {
            Err(Error::WrongFormatId(wire2))
        } else {
            Ok(Self::And { wire1, wire2 })
        }
    }

    pub fn or(wire1: impl Into<String>, wire2: impl Into<String>) -> Result<Self, Error> {
        let wire1 = wire1.into();
        let wire2 = wire2.into();
        if !wire1.bytes().all(|b| b.is_ascii_lowercase()) {
            Err(Error::WrongFormatId(wire1))
        } else if !wire2.bytes().all(|b| b.is_ascii_lowercase()) {
            Err(Error::WrongFormatId(wire2))
        } else {
            Ok(Self::Or { wire1, wire2 })
        }
    }

    pub fn sll(wire: impl Into<String>, shift: u8) -> Result<Self, Error> {
        let wire = wire.into();
        if !wire.bytes().all(|b| b.is_ascii_lowercase()) {
            Err(Error::WrongFormatId(wire))
        } else if shift > 15 {
            Err(Error::ShiftTooLarge(shift))
        } else {
            Ok(Self::SLL { wire, shift })
        }
    }

    pub fn slr(wire: impl Into<String>, shift: u8) -> Result<Self, Error> {
        let wire = wire.into();
        if !wire.bytes().all(|b| b.is_ascii_lowercase()) {
            Err(Error::WrongFormatId(wire))
        } else if shift > 15 {
            Err(Error::ShiftTooLarge(shift))
        } else {
            Ok(Self::SLR { wire, shift })
        }
    }

    pub fn not(wire: impl Into<String>) -> Result<Self, Error> {
        let wire = wire.into();
        if wire.bytes().all(|b| b.is_ascii_lowercase()) {
            Ok(Self::Not { wire })
        } else {
            Err(Error::WrongFormatId(wire))
        }
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
