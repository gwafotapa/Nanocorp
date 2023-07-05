use crate::{error::Error, wire::WireId};

#[derive(Debug, PartialEq)]
pub enum Gate {
    And { input1: WireId, input2: WireId },
    AndValue { input: WireId, value: u16 },
    Or { input1: WireId, input2: WireId },
    OrValue { input: WireId, value: u16 },
    SLL { input: WireId, shift: u8 },
    SLR { input: WireId, shift: u8 },
    Not { input: WireId },
}

impl Gate {
    pub fn and<S: Into<String>, T: Into<String>>(input1: S, input2: T) -> Result<Self, Error> {
        let input1 = input1.into();
        let input2 = input2.into();
        if !input1.bytes().all(|b| b.is_ascii_lowercase()) {
            Err(Error::WrongFormatId(input1))
        } else if !input2.bytes().all(|b| b.is_ascii_lowercase()) {
            Err(Error::WrongFormatId(input2))
        } else {
            Ok(Self::And { input1, input2 })
        }
    }

    pub fn and_value<S: Into<String>>(input: S, value: u16) -> Result<Self, Error> {
        let input = input.into();
        if input.bytes().all(|b| b.is_ascii_lowercase()) {
            Ok(Self::AndValue { input, value })
        } else {
            Err(Error::WrongFormatId(input))
        }
    }

    pub fn or<S: Into<String>, T: Into<String>>(input1: S, input2: T) -> Result<Self, Error> {
        let input1 = input1.into();
        let input2 = input2.into();
        if !input1.bytes().all(|b| b.is_ascii_lowercase()) {
            Err(Error::WrongFormatId(input1))
        } else if !input2.bytes().all(|b| b.is_ascii_lowercase()) {
            Err(Error::WrongFormatId(input2))
        } else {
            Ok(Self::Or { input1, input2 })
        }
    }

    pub fn or_value<S: Into<String>>(input: S, value: u16) -> Result<Self, Error> {
        let input = input.into();
        if input.bytes().all(|b| b.is_ascii_lowercase()) {
            Ok(Self::OrValue { input, value })
        } else {
            Err(Error::WrongFormatId(input))
        }
    }

    pub fn sll<S: Into<String>>(input: S, shift: u8) -> Result<Self, Error> {
        let input = input.into();
        if !input.bytes().all(|b| b.is_ascii_lowercase()) {
            Err(Error::WrongFormatId(input))
        } else if shift > 15 {
            Err(Error::ShiftTooLarge(shift))
        } else {
            Ok(Self::SLL { input, shift })
        }
    }

    pub fn slr<S: Into<String>>(input: S, shift: u8) -> Result<Self, Error> {
        let input = input.into();
        if !input.bytes().all(|b| b.is_ascii_lowercase()) {
            Err(Error::WrongFormatId(input))
        } else if shift > 15 {
            Err(Error::ShiftTooLarge(shift))
        } else {
            Ok(Self::SLR { input, shift })
        }
    }

    pub fn not<S: Into<String>>(input: S) -> Result<Self, Error> {
        let input = input.into();
        if input.bytes().all(|b| b.is_ascii_lowercase()) {
            Ok(Self::Not { input })
        } else {
            Err(Error::WrongFormatId(input))
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
