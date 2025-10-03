#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum StructureToken {
  Start,
  End,
  Intro,
  Outro,
  Bridge,
  Chorus,
  Verse,
}