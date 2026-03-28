use std::collections::TryReserveError;

use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum BuildError {
  #[error("failed to perform auxiliary allocation")]
  Allocation { inner: AllocError, src: BuildAllocationKind },
}
