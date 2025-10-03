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

        let i = result.len();
        //order matters
        if split.contains(&"chorus".to_string()) {
          result.push(StructureToken::Chorus(i));
        } else if split.contains(&"verse".to_string()) {
          result.push(StructureToken::Verse(i));
        } else if split.contains(&"intro".to_string()) {
          result.push(StructureToken::Intro(i));
        } else if split.contains(&"outro".to_string()) {
          result.push(StructureToken::Outro(i));
        } else if split.contains(&"bridge".to_string()) {
          result.push(StructureToken::Bridge(i));
        }
      }
    }

    result.push(StructureToken::End);
    result
  }
}
