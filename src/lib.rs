// TODO: check for pub(crate) -> pub (dangerous) and private -> pub(crate) (do i use it ?)
// TODO: Bottom line up front: sort content of impl blocks by importance, ctor/getters/setters and public/private

// Dependency reexports
pub use thiserror;

pub mod circuit;
// pub mod circuit_builder;
pub mod error;
// pub mod gate;
// pub mod signal;
// pub mod wire;
// pub mod wire_id;
