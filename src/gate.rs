use std::fmt;

use crate::{
    error::{Error, Result},
    signal::Signal,
    wire_id::WireId,
};

// TODO: derive Eq, Hash, Clone ?
#[derive(Debug, PartialEq)]
pub enum Gate {
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
        let input1 = WireId::try_from(input1.into())?;
        let input2 = WireId::try_from(input2.into())?;
        Ok(Self::And { input1, input2 })
    }

    pub fn and_value<S: Into<String>>(input: S, value: u16) -> Result<Self> {
        let input = WireId::try_from(input.into())?;
        Ok(Self::AndValue { input, value })
    }

    pub fn or<S: Into<String>, T: Into<String>>(input1: S, input2: T) -> Result<Self> {
        let input1 = WireId::try_from(input1.into())?;
        let input2 = WireId::try_from(input2.into())?;
        Ok(Self::Or { input1, input2 })
    }

    pub fn or_value<S: Into<String>>(input: S, value: u16) -> Result<Self> {
        let input = WireId::try_from(input.into())?;
        Ok(Self::OrValue { input, value })
    }

    pub fn lshift<S: Into<String>>(input: S, shift: u8) -> Result<Self> {
        let input = WireId::try_from(input.into())?;
        if shift < 16 {
            Ok(Self::LShift { input, shift })
        } else {
            Err(Error::TooLargeShift(shift))
        }
    }

    pub fn rshift<S: Into<String>>(input: S, shift: u8) -> Result<Self> {
        let input = WireId::try_from(input.into())?;
        if shift < 16 {
            Ok(Self::RShift { input, shift })
        } else {
            Err(Error::TooLargeShift(shift))
        }
    }

    pub fn not<S: Into<String>>(input: S) -> Result<Self> {
        let input = WireId::try_from(input.into())?;
        Ok(Self::Not { input })
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

    // TODO: ? Add nested types to enum Gate to implement a signal function for each gate variant
    pub fn signal(&self, input1: Option<u16>, input2: Option<u16>) -> Signal {
        match self {
            Gate::And { .. } => Signal::Value(input1.unwrap() & input2.unwrap()),
            Gate::Or { .. } => Signal::Value(input1.unwrap() | input2.unwrap()),
            Gate::AndValue { value, .. } => Signal::Value(input1.unwrap() & value),
            Gate::OrValue { value, .. } => Signal::Value(input1.unwrap() | value),
            Gate::LShift { shift, .. } => Signal::Value(input1.unwrap() << shift),
            Gate::RShift { shift, .. } => Signal::Value(input1.unwrap() >> shift),
            Gate::Not { .. } => Signal::Value(!input1.unwrap()),
        }
    }
}

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wire_id() {
        assert!(Gate::not("").is_err());
        assert!(Gate::not("A").is_err());
        assert!(Gate::not("#hashtag").is_err());
        assert!(Gate::and("input1", "input 2").is_err());
    }

    #[test]
    fn shift_amount() {
        assert!(Gate::lshift("sh", 0).is_ok());
        assert!(Gate::rshift("sh", 15).is_ok());
        assert!(matches!(
            Gate::rshift("sh", 16),
            Err(Error::TooLargeShift(16))
        ));
    }

    // TODO: matches!
    #[test]
    fn parse_gate() {
        assert!(Gate::try_from("").is_err());
        assert!(Gate::try_from("a").is_err());
        assert!(Gate::try_from("NOT").is_err());
        assert!(Gate::try_from("a AND NOT b").is_err());
        assert!(Gate::try_from("a OR").is_err());
        assert!(Gate::try_from("a NOT b").is_err());
    }

    #[test]
    fn parse_shift() {
        assert!(Gate::try_from("a LSHIFT 16").is_err());
        assert!(Gate::try_from("a RSHIFT a").is_err());
    }
}
