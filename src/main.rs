mod dataset_parser;
use tracing_subscriber::fmt::format::FmtSpan;

use crate::dataset_parser::{Parser, ParserOptions};

fn main() {
  tracing_subscriber::fmt()
    .compact()
    .with_span_events(FmtSpan::CLOSE)
    .init();

  let _ = Parser::parse(ParserOptions::default());
}
