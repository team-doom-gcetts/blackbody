


#[derive(thiserror::Error,Debug)]
pub enum BlackBoxError {
  #[error("device not found or inaccessible")]
  DeviceNotFound,
  #[error("verification failed")]
  VerificationFailed,
  #[error("i/o error: {0}")]
  IoError(#[from] std::io::Error),
  #[error("operation aborted")]
  OperationAborted,
  #[error("forensic extraction failed: {0}")]
  ExtractionFailed(String),
}








