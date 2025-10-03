use crate::song::{DataPoint, StructureToken};
use markov::Chain;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Song {
  structure: Chain<StructureToken>,
  intro: Chain<String>,
  outro: Chain<String>,
  bridge: Chain<String>,
  chorus: Chain<String>,
  verse: Chain<String>,
}

impl Song {
  pub fn new() -> Song {
    Song::of_order(1)
  }
  
  pub fn of_order(order: usize) -> Song {
    Song {
      structure: Chain::of_order(order),
      intro: Chain::of_order(order),
      outro: Chain::of_order(order),
      bridge: Chain::of_order(order),
      chorus: Chain::of_order(order),
      verse: Chain::of_order(order),
    }
  }


  pub fn train(&mut self, dataset: Vec<DataPoint>) {
    for data in dataset {
      self.structure.feed(data.structure);
      self.chorus.feed(data.chorus);

      for intro in data.intros {
        self.intro.feed(intro);
      }
      for outro in data.outros {
        self.outro.feed(outro);
      }
      for bridge in data.bridges {
        self.bridge.feed(bridge);
      }
      for verse in data.verses {
        self.verse.feed(verse);
      }
    }
  }

  pub fn generate(&self) -> String {
    let mut song = String::new();
    let structure = self.structure.generate();
    let chorus = self.chorus.generate().join(" ");

    for token in structure {

      let block;
      match token {
        StructureToken::Intro(_) => {
          block = self.intro.generate().join(" ");
        },
        StructureToken::Outro(_) => {
          block = self.outro.generate().join(" ");
        },
        StructureToken::Bridge(_) => {
          block = self.bridge.generate().join(" ");
        },
        StructureToken::Chorus(_) => {
          block = chorus.clone();
        },
        StructureToken::Verse(_) => {
          block = self.verse.generate().join(" ");
        },
        _ => continue,
      }

      song.push_str(&block);
      song.push_str("\n");
    }

    song
  }
}
