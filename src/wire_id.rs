use std::{fmt, ops};

use crate::error::Error;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct WireId(String);

// impl WireId {
// pub fn new<S: Into<String>>(id: S) -> Self {
//     Self(id.into())
// }

// pub fn is_valid(&self) -> bool {
//     !self.0.is_empty() && self.0.bytes().all(|b| b.is_ascii_lowercase())
// }
// }

impl TryFrom<String> for WireId {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Error> {
        if !s.is_empty() && s.bytes().all(|b| b.is_ascii_lowercase()) {
            Ok(Self(s))
        } else {
            Err(Error::InvalidWireId(s))
        }
    }
}

impl TryFrom<&str> for WireId {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Error> {
        Self::try_from(String::from(s))
    }
}

// impl From<&WireId> for WireId {
//     fn from(id: &Self) -> Self {
//         Self(id.0.to_string())
//     }
// }

// impl AsRef<WireId> for String {
//     fn as_ref(&self) -> &WireId {
//         &WireId(self)
//     }
// }

// impl AsRef<WireId> for str {
//     fn as_ref(&self) -> &WireId {
//         &WireId(self.to_string())
//     }
// }

impl ops::Deref for WireId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for WireId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from() {
        assert!(WireId::try_from("").is_err());
        assert!(WireId::try_from("w1r31d").is_err());
        assert!(WireId::try_from("Nanocorp").is_err());
        assert!(WireId::try_from("nanocorp!").is_err());
        assert!(WireId::try_from("nanocorp\n").is_err());
        assert!(WireId::try_from("nano corp").is_err());

        assert!(WireId::try_from("w").is_ok());
        assert!(WireId::try_from("nanocorp").is_ok());
    }
}
