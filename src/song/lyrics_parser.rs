use crate::song::{DataPoint, StructureToken};

pub struct LyricsParser;

impl LyricsParser {
  pub fn parse(lyrics: &str) -> DataPoint {
    use StructureToken::*;

    let lyrics = clean(lyrics);

    let mut output = DataPoint::default();
    output.structure.push(StructureToken::Start);

    let mut block = String::new();
    for line in lyrics.lines() {
      if line.starts_with('[') {
        let i = output.structure.len();
        let check = line.to_lowercase();

        //order matters
        if check.contains(&"chorus".to_string()) {
          output.structure.push(Chorus(i));
        } else if check.contains(&"verse".to_string()) {
          output.structure.push(Verse(i));
        } else if check.contains(&"intro".to_string()) {
          output.structure.push(Intro(i));
        } else if check.contains(&"outro".to_string()) {
          output.structure.push(Outro(i));
        } else if check.contains(&"bridge".to_string()) {
          output.structure.push(Bridge(i));
        } else {
          continue;
        }

        if !block.is_empty() {
          match output.structure.get(output.structure.len() - 2) {
            Some(token) => output.add_block(&block, token.clone()),
            None => output.add_block(&block, Start),
          }
        }
      } else if !line.trim().is_empty() {
        block.push_str(&line);
        block.push_str("\n");
      }
    }

    if !block.is_empty() {
      match output.structure.get(output.structure.len() - 2) {
        Some(token) => output.add_block(&block, token.clone()),
        None => output.add_block(&block, Start),
      }
    }
    output.structure.push(StructureToken::End);

    output
  }
}

fn clean(lyrics: &str) -> String {
  let mut chars = lyrics.chars().collect::<Vec<char>>();

  let mut i = 1;
  while i < chars.len() {
    if chars[i].is_uppercase() && chars[i - 1].is_lowercase() {
      chars.insert(i, '\n');
      i += 2;
    }
    i += 1;
  }

  let mut lyrics: String = chars.iter().collect();

  lyrics = lyrics
    .replace("\\n", "\n")
    .lines()
    .filter(|line| !line.is_empty() && line.contains(char::is_alphanumeric))
    .collect::<Vec<&str>>()
    .join("\n");

  let allowed_punctuation = ['\'', '[', ']'];
  let allowed_whitespace = ['\n', ' '];
  lyrics = lyrics
    .chars()
    .filter(|&c| {
      c.is_alphanumeric() || allowed_punctuation.contains(&c) || allowed_whitespace.contains(&c)
    })
    .collect();

  lyrics
}
