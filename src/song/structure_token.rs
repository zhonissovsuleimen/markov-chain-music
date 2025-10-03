#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum StructureToken {
  Start,
  End,
  Intro(usize),
  Outro(usize),
  Bridge(usize),
  Chorus(usize),
  Verse(usize),
}