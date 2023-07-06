use std::fmt;

use crate::{
    error::{GateError, ParseGateError, WireIdError},
    wire::WireId,
};

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
    pub fn and<S: Into<String>, T: Into<String>>(input1: S, input2: T) -> Result<Self, GateError> {
        let input1 = input1.into();
        let input2 = input2.into();
        if !input1.bytes().all(|b| b.is_ascii_lowercase()) {
            Err(WireIdError(input1).into())
        } else if !input2.bytes().all(|b| b.is_ascii_lowercase()) {
            Err(WireIdError(input2).into())
        } else {
            Ok(Self::And { input1, input2 })
        }
    }

    pub fn and_value<S: Into<String>>(input: S, value: u16) -> Result<Self, GateError> {
        let input = input.into();
        if input.bytes().all(|b| b.is_ascii_lowercase()) {
            Ok(Self::AndValue { input, value })
        } else {
            Err(WireIdError(input).into())
        }
    }

    pub fn or<S: Into<String>, T: Into<String>>(input1: S, input2: T) -> Result<Self, GateError> {
        let input1 = input1.into();
        let input2 = input2.into();
        if !input1.bytes().all(|b| b.is_ascii_lowercase()) {
            Err(WireIdError(input1).into())
        } else if !input2.bytes().all(|b| b.is_ascii_lowercase()) {
            Err(WireIdError(input2).into())
        } else {
            Ok(Self::Or { input1, input2 })
        }
    }

    pub fn or_value<S: Into<String>>(input: S, value: u16) -> Result<Self, GateError> {
        let input = input.into();
        if input.bytes().all(|b| b.is_ascii_lowercase()) {
            Ok(Self::OrValue { input, value })
        } else {
            Err(WireIdError(input).into())
        }
    }

    pub fn sll<S: Into<String>>(input: S, shift: u8) -> Result<Self, GateError> {
        let input = input.into();
        if !input.bytes().all(|b| b.is_ascii_lowercase()) {
            Err(WireIdError(input).into())
        } else if shift > 15 {
            Err(GateError::TooLargeShift(shift))
        } else {
            Ok(Self::SLL { input, shift })
        }
    }

    pub fn slr<S: Into<String>>(input: S, shift: u8) -> Result<Self, GateError> {
        let input = input.into();
        if !input.bytes().all(|b| b.is_ascii_lowercase()) {
            Err(WireIdError(input).into())
        } else if shift > 15 {
            Err(GateError::TooLargeShift(shift))
        } else {
            Ok(Self::SLR { input, shift })
        }
    }

    pub fn not<S: Into<String>>(input: S) -> Result<Self, GateError> {
        let input = input.into();
        if input.bytes().all(|b| b.is_ascii_lowercase()) {
            Ok(Self::Not { input })
        } else {
            Err(WireIdError(input).into())
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
            Gate::SLL { input, shift } => {
                write!(f, "{} LSHIFT {}", input, shift)
            }
            Gate::SLR { input, shift } => {
                write!(f, "{} RSHIFT {}", input, shift)
            }
            Gate::Not { input } => {
                write!(f, "NOT {}", input)
            }
        }
    }
}

impl TryFrom<&str> for Gate {
    type Error = ParseGateError;

    fn try_from(s: &str) -> Result<Self, ParseGateError> {
        let elements: Vec<&str> = s.split(' ').collect();
        match elements.len() {
            2 => {
                if elements[0] == "NOT" {
                    Ok(Gate::not(elements[1])?)
                } else {
                    Err(ParseGateError::UnknownGate(s.to_string()))
                }
            }
            3 => match elements[1] {
                "AND" => {
                    if let Ok(value) = elements[0].parse::<u16>() {
                        Ok(Gate::and_value(elements[2], value)?)
                    } else if let Ok(value) = elements[2].parse::<u16>() {
                        Ok(Gate::and_value(elements[0], value)?)
                    } else {
                        Ok(Gate::and(elements[0], elements[2])?)
                    }
                }
                "OR" => {
                    if let Ok(value) = elements[0].parse::<u16>() {
                        Ok(Gate::or_value(elements[2], value)?)
                    } else if let Ok(value) = elements[2].parse::<u16>() {
                        Ok(Gate::or_value(elements[0], value)?)
                    } else {
                        Ok(Gate::or(elements[0], elements[2])?)
                    }
                }
                "LSHIFT" => Ok(Gate::sll(elements[0], elements[2].parse::<u8>()?)?),
                "RSHIFT" => Ok(Gate::slr(elements[0], elements[2].parse::<u8>()?)?),
                _ => Err(ParseGateError::UnknownGate(s.to_string())),
            },
            _ => Err(ParseGateError::UnknownGate(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_ids() {
        assert!(Gate::not("A").is_err());
        assert!(Gate::not("#hashtag").is_err());
        assert!(Gate::and("input1", "input 2").is_err());
    }

    #[test]
    fn shifts() {
        assert!(Gate::slr("sh", 15).is_ok());
        assert!(Gate::slr("sh", 16).is_err());
    }
}
