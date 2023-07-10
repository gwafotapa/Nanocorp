use std::fmt;

use crate::error::{Error, Result};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct WireId(String);

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

    fn try_from(s: String) -> Result<Self> {
        if !s.is_empty() && s.bytes().all(|b| b.is_ascii_lowercase()) {
            Ok(Self(s))
        } else {
            Err(Error::InvalidWireId(s))
        }
    }
}

impl TryFrom<&str> for WireId {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        Self::try_from(String::from(s))
    }
}

impl From<WireId> for String {
    fn from(w: WireId) -> Self {
        w.to_string()
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

impl fmt::Display for WireId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// impl Ord for WireId {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         cmp(self.0, other.0)
//     }
// }

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
