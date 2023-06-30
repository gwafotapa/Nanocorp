struct Wire {
    label: String,
    signal: u16,
}

impl Wire {
    fn new(label: impl Into<String>, signal: u16) -> Option<Self> {
        let label = label.into();
        label
            .bytes()
            .all(|b| b.is_ascii_lowercase())
            .then_some(Self { label, signal })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels() {
        assert!(Wire::new("A", 0).is_none());
        assert!(Wire::new("3", 0).is_none());
        assert!(Wire::new("nano corp", 0).is_none());
        assert!(Wire::new("nanocorp", 0).is_some());
        assert!(Wire::new("wire!", 0).is_none());
        assert!(Wire::new("z\n", 0).is_none());
    }
}
