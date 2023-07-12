use std::fmt::{self, Display, Formatter};

use super::{signal::Signal, wire_id::WireId};
use crate::error::{Error, Result};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum Gate {
    And { input1: WireId, input2: WireId },
    AndValue { input: WireId, value: u16 },
    Or { input1: WireId, input2: WireId },
    OrValue { input: WireId, value: u16 },
    LShift { input: WireId, shift: u8 },
    RShift { input: WireId, shift: u8 },
    Not { input: WireId },
}

impl Gate {
    pub fn and<S: Into<String>, T: Into<String>>(input1: S, input2: T) -> Result<Self> {
        Ok(Self::And {
            input1: WireId::new(input1)?,
            input2: WireId::new(input2)?,
        })
    }

    pub fn and_value<S: Into<String>>(input: S, value: u16) -> Result<Self> {
        Ok(Self::AndValue {
            input: WireId::new(input)?,
            value,
        })
    }

    pub fn or<S: Into<String>, T: Into<String>>(input1: S, input2: T) -> Result<Self> {
        Ok(Self::Or {
            input1: WireId::new(input1)?,
            input2: WireId::new(input2)?,
        })
    }

    pub fn or_value<S: Into<String>>(input: S, value: u16) -> Result<Self> {
        Ok(Self::OrValue {
            input: WireId::new(input)?,
            value,
        })
    }

    pub fn lshift<S: Into<String>>(input: S, shift: u8) -> Result<Self> {
        let input = WireId::new(input)?;
        if shift < 16 {
            Ok(Self::LShift { input, shift })
        } else {
            Err(Error::TooLargeShift(shift))
        }
    }

    pub fn rshift<S: Into<String>>(input: S, shift: u8) -> Result<Self> {
        let input = WireId::new(input)?;
        if shift < 16 {
            Ok(Self::RShift { input, shift })
        } else {
            Err(Error::TooLargeShift(shift))
        }
    }

    pub fn not<S: Into<String>>(input: S) -> Result<Self> {
        Ok(Self::Not {
            input: WireId::new(input)?,
        })
    }

    pub fn has_input(&self, id: &WireId) -> bool {
        match self {
            Gate::And { input1, input2 } => id == input1 || id == input2,
            Gate::Or { input1, input2 } => id == input1 || id == input2,
            Gate::AndValue { input, .. } => id == input,
            Gate::OrValue { input, .. } => id == input,
            Gate::LShift { input, .. } => id == input,
            Gate::RShift { input, .. } => id == input,
            Gate::Not { input } => id == input,
        }
    }

    pub fn signal(&self, input1: u16, input2: Option<u16>) -> Signal {
        match self {
            Gate::And { .. } => Signal::Value(input1 & input2.unwrap()),
            Gate::Or { .. } => Signal::Value(input1 | input2.unwrap()),
            Gate::AndValue { value, .. } => Signal::Value(input1 & value),
            Gate::OrValue { value, .. } => Signal::Value(input1 | value),
            Gate::LShift { shift, .. } => Signal::Value(input1 << shift),
            Gate::RShift { shift, .. } => Signal::Value(input1 >> shift),
            Gate::Not { .. } => Signal::Value(!input1),
        }
    }
}

impl TryFrom<&str> for Gate {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        let elements: Vec<&str> = s.split(' ').collect();
        match elements.len() {
            2 => {
                if elements[0] == "NOT" {
                    Ok(Gate::not(elements[1])?)
                } else {
                    Err(Error::ParseGate(s.to_string()))
                }
            }
            3 => match elements[1] {
                "AND" => {
                    if let Ok(value) = elements[0].parse::<u16>() {
                        Gate::and_value(elements[2], value)
                    } else if let Ok(value) = elements[2].parse::<u16>() {
                        Gate::and_value(elements[0], value)
                    } else {
                        Gate::and(elements[0], elements[2])
                    }
                }
                "OR" => {
                    if let Ok(value) = elements[0].parse::<u16>() {
                        Gate::or_value(elements[2], value)
                    } else if let Ok(value) = elements[2].parse::<u16>() {
                        Gate::or_value(elements[0], value)
                    } else {
                        Gate::or(elements[0], elements[2])
                    }
                }
                "LSHIFT" => Gate::lshift(elements[0], elements[2].parse::<u8>()?),
                "RSHIFT" => Gate::rshift(elements[0], elements[2].parse::<u8>()?),
                _ => Err(Error::ParseGate(s.to_string())),
            },
            _ => Err(Error::ParseGate(s.to_string())),
        }
    }
}

impl Display for Gate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Gate::And { input1, input2 } => {
                write!(f, "{} AND {}", input1, input2)
            }
            Gate::AndValue { input, value } => {
                write!(f, "{} AND {}", input, value)
            }
            Gate::Or { input1, input2 } => {
                write!(f, "{} OR {}", input1, input2)
            }
            Gate::OrValue { input, value } => {
                write!(f, "{} OR {}", input, value)
            }
            Gate::LShift { input, shift } => {
                write!(f, "{} LSHIFT {}", input, shift)
            }
            Gate::RShift { input, shift } => {
                write!(f, "{} RSHIFT {}", input, shift)
            }
            Gate::Not { input } => {
                write!(f, "NOT {}", input)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wire_id() {
        assert!(Gate::or_value("valid", 3).is_ok());
        assert!(Gate::and("x", "x").is_ok());

        assert!(matches!(Gate::not(""), Err(Error::InvalidWireId(_))));
        assert!(matches!(Gate::not("A"), Err(Error::InvalidWireId(_))));
        assert!(matches!(
            Gate::not("#hashtag"),
            Err(Error::InvalidWireId(_))
        ));
        assert!(matches!(
            Gate::and("input1", "input 2"),
            Err(Error::InvalidWireId(_))
        ));
    }

    #[test]
    fn shift_amount() {
        assert!(Gate::lshift("sh", 0).is_ok());
        assert!(Gate::rshift("sh", 15).is_ok());
        assert!(matches!(
            Gate::rshift("sh", 16),
            Err(Error::TooLargeShift(16))
        ));
        assert!(matches!(
            Gate::try_from("a LSHIFT 31"),
            Err(Error::TooLargeShift(31))
        ));
    }

    #[test]
    fn parse_gate() {
        assert!(matches!(Gate::try_from(""), Err(Error::ParseGate(_))));
        assert!(matches!(Gate::try_from("a"), Err(Error::ParseGate(_))));
        assert!(matches!(Gate::try_from("NOT"), Err(Error::ParseGate(_))));
        assert!(matches!(
            Gate::try_from("a AND NOT b"),
            Err(Error::ParseGate(_))
        ));
        assert!(matches!(Gate::try_from("a OR"), Err(Error::ParseGate(_))));
        assert!(matches!(
            Gate::try_from("a NOT b"),
            Err(Error::ParseGate(_))
        ));
    }

    #[test]
    fn parse_shift() {
        assert!(matches!(
            Gate::try_from("a RSHIFT a"),
            Err(Error::ParseShift(_))
        ));
    }

    #[test]
    fn has_input() -> Result<()> {
        assert!(Gate::not("x")?.has_input(&WireId::new("x")?));
        assert!(Gate::and("x", "y")?.has_input(&WireId::new("x")?));
        assert!(Gate::and("x", "y")?.has_input(&WireId::new("y")?));

        assert!(!Gate::not("xx")?.has_input(&WireId::new("x")?));
        assert!(!Gate::not("x")?.has_input(&WireId::new("xx")?));
        assert!(!Gate::and("x", "y")?.has_input(&WireId::new("xy")?));
        Ok(())
    }

    #[test]
    fn signal() -> Result<()> {
        assert_eq!(
            Gate::and("x", "y")?.signal(353, Some(57)),
            Signal::Value(353 & 57)
        );
        assert_eq!(
            Gate::or("x", "y")?.signal(119, Some(3222)),
            Signal::Value(119 | 3222)
        );
        assert_eq!(
            Gate::and_value("x", 226)?.signal(27, None),
            Signal::Value(27 & 226)
        );
        assert_eq!(
            Gate::or_value("x", 913)?.signal(172, None),
            Signal::Value(172 | 913)
        );
        assert_eq!(
            Gate::lshift("x", 7)?.signal(34, None),
            Signal::Value(34 << 7)
        );
        assert_eq!(
            Gate::rshift("x", 3)?.signal(1925, None),
            Signal::Value(1925 >> 3)
        );
        assert_eq!(Gate::not("x")?.signal(0xa56e, None), Signal::Value(!0xa56e));
        Ok(())
    }
}
