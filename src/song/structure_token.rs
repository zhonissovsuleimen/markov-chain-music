use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
pub enum StructureToken {
  Start,
  End,
  Intro(usize),
  Outro(usize),
  Bridge(usize),
  Chorus(usize),
  Verse(usize),
}