use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildError {
  #[error("failed to perform auxiliary allocation")]
  AuxiliaryAlloc,
}
