use crate::song::StructureToken;

pub struct LyricsParser;

impl LyricsParser {
  pub fn get_structure(lyrics: &str) -> Vec<StructureToken> {
    //some lyrics contain \n
    let lyrics = lyrics.replace("\\n", "\n");

    let mut result = vec![StructureToken::Start];

    for line in lyrics.lines() {
      if line.starts_with('[') {
        let parsed_line = line.replace(|c: char| c.is_ascii_punctuation(), " ");
        let split: Vec<String> = parsed_line
          .split(' ')
          .map(|str| str.to_lowercase())
          .collect();

        //order matters
        if split.contains(&"chorus".to_string()) {
          result.push(StructureToken::Chorus);
        } else if split.contains(&"verse".to_string()) {
          result.push(StructureToken::Verse);
        } else if split.contains(&"intro".to_string()) {
          result.push(StructureToken::Intro);
        } else if split.contains(&"outro".to_string()) {
          result.push(StructureToken::Outro);
        } else if split.contains(&"bridge".to_string()) {
          result.push(StructureToken::Bridge);
        }
      }
    }

    result.push(StructureToken::End);
    result
  }
}
