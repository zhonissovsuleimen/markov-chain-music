mod dataset_parser;
mod song;

use crate::{
  dataset_parser::{Parser, ParserOptions},
  song::{LyricsParser, StructureToken},
};
use markov::Chain;
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;

fn main() {
  tracing_subscriber::fmt()
    .compact()
    .with_span_events(FmtSpan::CLOSE)
    .init();

  let mut records = Parser::parse(ParserOptions {
    language_preference: Some(String::from("en")),
    ..Default::default()
  })
  .unwrap();

  //remove some really really long songs(performances / collections?)
  //that contain multiple songs in a row
  //thus skewing the results by decreasing the chances of End token appearing
  records.retain(|r| r.lyrics.len() <= 3000);

  let mut chain = Chain::<StructureToken>::of_order(2);
  for record in records {
    let structure = LyricsParser::get_structure(&record.lyrics);
    if structure.len() == 2 {
      continue;
    }

    if structure.len() > 100 {
      println!("{:?}", record);
    }

    chain.feed(structure);
  }

  for (i, structure) in chain.iter_for(10).enumerate() {
    info!("CHAIN {}: {:?}", i + 1, structure);
  }
}
