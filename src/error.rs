use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Wrong format id '{0}'")]
    WrongFormatId(String),
    #[error("Shift amount '{0}' too large")]
    ShiftTooLarge(u8),
    #[error("Id '{0}' already exists")]
    IdAlreadyExists(String),
}
