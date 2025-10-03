use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
  #[error("IO Error: {0}")]
  IO(#[from] std::io::Error),
  #[error("CSV parse error: {0}")]
  Csv(#[from] csv::Error),
}