use crate::signal::Signal;

#[derive(Clone, Copy)]
pub enum Source<'a> {
    Value(u16),
    Wire(&'a Wire<'a>),
}

#[derive(Clone)]
pub struct Wire<'a> {
    pub id: String,
    pub source: Option<Source<'a>>,
    pub signal: Option<u16>,
}

impl<'a> Wire<'a> {
    pub fn no_source(id: impl Into<String>) -> Option<Self> {
        let id = id.into();
        id.bytes().all(|b| b.is_ascii_lowercase()).then_some(Self {
            id,
            source: None,
            signal: None,
        })
    }

    pub fn new(
        id: impl Into<String>,
        source: Source<'a>,
        // signal: Option<u16>,
    ) -> Option<Self> {
        let id = id.into();
        id.bytes().all(|b| b.is_ascii_lowercase()).then_some(Self {
            id,
            source: Some(source),
            signal: None,
        })
    }

    pub fn source_value(id: impl Into<String>, value: u16) -> Option<Self> {
        let id = id.into();
        id.bytes().all(|b| b.is_ascii_lowercase()).then_some(Self {
            id,
            source: Some(Source::Value(value)),
            signal: None,
        })
    }

    pub fn source_wire(
        id: impl Into<String>,
        wire: &'a Wire<'a>,
        // signal: Option<u16>,
    ) -> Option<Self> {
        let id = id.into();
        id.bytes().all(|b| b.is_ascii_lowercase()).then_some(Self {
            id,
            source: Some(Source::Wire(wire)),
            signal: None,
        })
    }

    // pub fn source_gate(
    //     id: impl Into<String>,
    //     gate: Gate<'a>,
    //     // signal: Option<u16>,
    // ) -> Option<Self> {
    //     let id = id.into();
    //     id.bytes().all(|b| b.is_ascii_lowercase()).then_some(Self {
    //         id,
    //         source: Some(Source::Gate(gate)),
    //     })
    // }

    pub fn compute_signal(&self) -> Option<u16> {
        if let Some(source) = self.source {
            match source {
                Source::Value(value) => Some(value),
                Source::Wire(wire) => {
                    wire.compute_signal();
                    wire.signal
                }
            }
        } else {
            None
        }
    }

    pub fn set_signal(&mut self) {
        self.signal = self.compute_signal();
    }
}

impl<'a> Signal for Wire<'a> {
    fn signal(&self) -> Option<u16> {
        self.source
            .map(|s| match s {
                Source::Value(value) => Some(value),
                Source::Wire(wire) => wire.signal(),
            })
            .flatten()
        // match self.source {
        //     Source::Value(value) => Some(value),
        //     Source::Wire(wire) => wire.signal(),
        // }
    }
}

impl<'a> PartialEq for Wire<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids() {
        assert!(Wire::no_source("A").is_none());
        assert!(Wire::no_source("3").is_none());
        assert!(Wire::no_source("nano corp").is_none());
        assert!(Wire::no_source("nanocorp").is_some());
        assert!(Wire::no_source("wire!").is_none());
        assert!(Wire::no_source("z\n").is_none());
    }

    #[test]
    fn sources() {
        let w1 = Wire::new("a", Source::Value(1)).unwrap();
        assert_eq!(w1.signal(), Some(1));

        let w2 = Wire::new("b", Source::Wire(&w1)).unwrap();
        assert_eq!(w2.signal(), Some(1));
    }
}
