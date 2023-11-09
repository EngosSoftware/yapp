//!

///
pub struct Replacement {
  from: String,
  to: String,
}

impl Replacement {
  ///
  pub fn new(from: String, to: String) -> Self {
    Self { from, to }
  }

  ///
  pub fn replace(&self, content: &str) -> String {
    content.replace(&self.from, &self.to)
  }
}

#[derive(Default)]
pub struct Replacements(Vec<Replacement>);

impl Replacements {
  ///
  pub fn new(replacements: Vec<Replacement>) -> Self {
    Self(replacements)
  }

  ///
  pub fn replace(&self, content: &str) -> String {
    let mut modified = content.to_string();
    for replacement in &self.0 {
      modified = replacement.replace(&modified);
    }
    modified
  }
}
