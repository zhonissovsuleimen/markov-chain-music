mod dataset_parser;
mod song;

use crate::{
  dataset_parser::{Parser, ParserOptions},
  song::{LyricsParser, Song},
};
use markov::Chain;
use serde::Serialize;
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

  let mut dataset = vec![];
  for record in records {
    let data = LyricsParser::parse(&record.lyrics);

    //removing songs with overly detailed structure / or structure that contain a lot of pre-chorus post-chorus etc..
    if data.structure.len() == 2 || data.structure.len() > 8 {
      continue;
    }

    dataset.push(data);
  }

  let mut song = Song::new();
  song.train(dataset);

  println!("{}", song.generate());

  // let save = serde_yaml::to_string(&song).expect("LOL");
  // std::fs::write("test.yaml", save).expect("lul");
}
