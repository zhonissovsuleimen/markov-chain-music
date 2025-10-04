mod dataset_parser;
mod song;

use crate::{
  dataset_parser::{DatasetParser, ParserOptions},
  song::{LyricsParser, Song},
};
use tracing_subscriber::fmt::format::FmtSpan;

fn main() {
  tracing_subscriber::fmt()
    .compact()
    .with_span_events(FmtSpan::CLOSE)
    .init();

  let chains_dir = std::path::PathBuf::from("chains");
  std::fs::create_dir_all(&chains_dir).expect("Failed to create chains directory");

  //something wrong with "country" and "misc", look into it later
  let tags = vec!["pop", "rock", "rap", "rb"];

  for tag in tags {
    let mut records = DatasetParser::parse(ParserOptions {
      language_preference: Some(String::from("en")),
      tag_preference: Some(String::from(tag)),
      ..Default::default()
    })
    .unwrap();

    //remove some really really long songs(performances / collections?)
    //that contain multiple songs in a row
    //thus skewing the results by decreasing the chances of End token appearing
    records.retain(|r| r.lyrics.len() <= 3000);

    let mut dataset = vec![];
    for record in records {
      let datapoint = LyricsParser::parse(&record.lyrics);

      //removing songs with overly detailed structure / or structure that contain a lot of pre-chorus post-chorus etc..
      if datapoint.structure.len() == 2 || datapoint.structure.len() > 8 {
        continue;
      }
      dataset.push(datapoint);
    }

    let mut song = Song::new();
    song.train(dataset);

    let path = chains_dir.join(format!("{}-{}.yaml", "en", tag));
    let yaml = serde_yaml::to_string(&song).expect("Failed to serialize song");
    std::fs::write(&path, yaml).expect("Failed to write to file");
  }
}
