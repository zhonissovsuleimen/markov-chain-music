#[derive(Debug, serde::Deserialize)]
pub struct Record {
  pub title: String,
  pub tag: String,
  pub artist: String,
  pub year: i64,
  pub views: i64,
  pub features: String,
  pub lyrics: String,
  pub id: i64,
  pub language_cld3: String,
  pub language_ft: String,
  pub language: String
}

impl Record {
  pub fn certain_language(&self) -> Option<String> {
    if self.language_cld3 == self.language_ft && self.language_cld3 == self.language {
      Some(self.language_cld3.clone())
    } else {
      None
    }
  }

  pub fn probable_language(&self) -> Option<String> {
    if self.language_cld3 == self.language_ft || self.language_cld3 == self.language {
      Some(self.language_cld3.clone())
    } else if self.language_ft == self.language {
      Some(self.language_ft.clone())
    } else {
      None
    }
  }
}