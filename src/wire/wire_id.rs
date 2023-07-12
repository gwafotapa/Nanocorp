use std::fmt::{self, Display, Formatter};

use crate::error::{Error, Result};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct WireId(String);

impl WireId {
    pub fn new<S: Into<String>>(id: S) -> Result<Self> {
        Self::try_from(id.into())
    }

    fn is_valid(id: &str) -> bool {
        !id.is_empty() && id.bytes().all(|b| b.is_ascii_lowercase())
    }
}

impl TryFrom<&str> for WireId {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        Self::try_from(String::from(s))
    }
}

impl TryFrom<String> for WireId {
    type Error = Error;

    fn try_from(s: String) -> Result<Self> {
        if WireId::is_valid(&s) {
            Ok(Self(s))
        } else {
            Err(Error::InvalidWireId(s))
        }
    }
}

impl From<WireId> for String {
    fn from(w: WireId) -> Self {
        w.to_string()
    }
}

impl Display for WireId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_ids() {
        assert!(WireId::new("").is_err());
        assert!(WireId::new("w1r31d").is_err());
        assert!(WireId::new("Nanocorp").is_err());
        assert!(WireId::new("nanocorp!").is_err());
        assert!(WireId::new("nanocorp\n").is_err());
        assert!(WireId::new("nano corp").is_err());

        assert!(WireId::new("w").is_ok());
        assert!(WireId::new("nanocorp").is_ok());
    }
}
