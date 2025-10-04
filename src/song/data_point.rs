use crate::song::StructureToken;

#[derive(PartialEq, Eq, Hash, Clone, Default)]
pub struct DataPoint {
  pub structure: Vec<StructureToken>,
  pub intros: Vec<Vec<String>>,
  pub outros: Vec<Vec<String>>,
  pub bridges: Vec<Vec<String>>,
  pub verses: Vec<Vec<String>>,
  pub chorus: Vec<String>,
  pub unmarked: Vec<Vec<String>>,
}

impl DataPoint {
  pub fn add_block(&mut self, block: &str, flow: StructureToken) {
    use StructureToken::*;
    let block_words: Vec<String> = block
      .split(' ')
      .map(|str| str.to_string())
      .collect();

    if block_words.len() == 0 {
      return;
    }

    match flow {
      Intro(_) => self.intros.push(block_words),
      Outro(_) => self.outros.push(block_words),
      Bridge(_) => self.bridges.push(block_words),
      Verse(_) => self.verses.push(block_words),
      Chorus(_) => {
        if self.chorus.is_empty() {
          self.chorus = block_words
        }
      }
      _ => self.unmarked.push(block_words),
    }
  }
}
