use csv::ReaderBuilder;
use tracing::instrument;
use crate::dataset_parser::{parser_error::ParserError, Record};

//Parser for dataset:
//https://huggingface.co/datasets/sebastiandizon/genius-song-lyrics


#[derive(Debug)]
pub struct ParserOptions {
  pub path: String,
  pub language_preference: Option<String>,
  pub tag_preference: Option<String>,
}

impl Default for ParserOptions {
  fn default() -> Self {
    ParserOptions {
      path: String::from(".\\dataset\\song_lyrics 2.csv"),
      language_preference: None,
      tag_preference: None,
    }
  }
}

pub struct Parser;

impl Parser {
  #[instrument(skip(options))]
  pub fn parse(options: ParserOptions) -> Result<Vec<Record>, ParserError> {
    use std::fs::File;
    let file = File::open(options.path)?;
    let mut csv_reader = ReaderBuilder::new().from_reader(file);

    let mut result = Vec::new();
    for parsed_record in csv_reader.deserialize::<Record>() {
      let record = parsed_record?;

      if (options.language_preference.is_some() && options.language_preference != record.certain_language()) ||
        (options.tag_preference.is_some() && options.tag_preference != Some(record.tag.clone())){
        continue;
      }

      result.push(record);
    }

    Ok(result)
  }
}