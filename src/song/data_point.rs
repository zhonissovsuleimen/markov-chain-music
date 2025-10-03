use crate::song::StructureToken;

#[derive(PartialEq, Eq, Hash, Clone, Default)]
pub struct DataPoint {
  pub structure: Vec<StructureToken>,
  pub intros: Vec<Vec<String>>,
  pub outros: Vec<Vec<String>>,
  pub bridges: Vec<Vec<String>>,
  pub verses: Vec<Vec<String>>,
  pub chorus: Vec<String>,
}

impl DataPoint {
  pub fn add_block(&mut self, block: &str, flow: StructureToken) {
    use StructureToken::*;
    match flow {
      Intro(_) => self.intros.push(block.split(' ').map(|str| str.to_string()).collect()),
      Outro(_) => self.outros.push(block.split(' ').map(|str| str.to_string()).collect()),
      Bridge(_) => self.bridges.push(block.split(' ').map(|str| str.to_string()).collect()),
      Verse(_) => self.verses.push(block.split(' ').map(|str| str.to_string()).collect()),
      Chorus(_) => {
        self.chorus = block.split(' ').map(|str| str.to_string()).collect();
      }
      _ => {}
    }
  }
}
