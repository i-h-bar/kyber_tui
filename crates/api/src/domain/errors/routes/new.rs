use thiserror::Error;

#[derive(Error, Debug)]
pub enum NewUserError {
    #[error("Deserialisation error")]
    DeserialisationError,
}