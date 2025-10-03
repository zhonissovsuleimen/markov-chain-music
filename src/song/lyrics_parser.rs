use crate::song::{DataPoint, StructureToken};

pub struct LyricsParser;

impl LyricsParser {
  pub fn parse(lyrics: &str) -> DataPoint {
    let mut lyrics = lyrics.replace("\\n", "\n");
    let mut chars = lyrics.chars().collect::<Vec<char>>();

    let mut i = 1;
    while i < chars.len() {
      if chars[i].is_uppercase() && chars[i - 1].is_lowercase() {
        chars.insert(i, ' ');
      }
      i += 1;
    }
    lyrics = chars.iter().collect::<String>();

    let mut output = DataPoint::default();
    output.structure.push(StructureToken::Start);

    let mut block = String::new();
    let mut flow = StructureToken::Start;
    for line in lyrics.lines() {
      if line.starts_with('[') {
        let parsed_line = line.replace(|c: char| c.is_ascii_punctuation(), " ");
        let split: Vec<String> = parsed_line
          .split(' ')
          .map(|str| str.to_lowercase())
          .collect();

        output.add_block(&block, flow.clone());

        //order matters
        let i = output.structure.len();
        if split.contains(&"chorus".to_string()) {
          flow = StructureToken::Chorus(i);
        } else if split.contains(&"verse".to_string()) {
          flow = StructureToken::Verse(i);
        } else if split.contains(&"intro".to_string()) {
          flow = StructureToken::Intro(i);
        } else if split.contains(&"outro".to_string()) {
          flow = StructureToken::Outro(i);
        } else if split.contains(&"bridge".to_string()) {
          flow = StructureToken::Bridge(i);
        }

        output.structure.push(flow.clone());
      } else {
        block.push_str(line);
      }
    }

    output.structure.push(StructureToken::End);
    output
  }
}
