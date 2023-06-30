pub struct Wire {
    pub id: String,
    pub signal: Option<u16>,
}

impl Wire {
    pub fn new(id: impl Into<String>, signal: Option<u16>) -> Option<Self> {
        let id = id.into();
        id.bytes()
            .all(|b| b.is_ascii_lowercase())
            .then_some(Self { id, signal })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids() {
        assert!(Wire::new("A", Some(0)).is_none());
        assert!(Wire::new("3", Some(1)).is_none());
        assert!(Wire::new("nano corp", Some(0)).is_none());
        assert!(Wire::new("nanocorp", None).is_some());
        assert!(Wire::new("wire!", Some(0)).is_none());
        assert!(Wire::new("z\n", Some(0)).is_none());
    }
}
