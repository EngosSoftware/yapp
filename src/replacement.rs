//! # Replacements

/// Single replacement, stores the pattern to be searched and the replacement text.
pub struct Replacement {
  /// Searched pattern.
  from: String,
  /// Replacement text
  to: String,
}

impl Replacement {
  /// Creates new replacement from search phrase and replacement text.
  pub fn new(from: String, to: String) -> Self {
    Self { from, to }
  }

  /// Replaces a pattern with replacement text in provided content.
  /// Returns the changed content.
  pub fn replace(&self, content: &str) -> String {
    content.replace(&self.from, &self.to)
  }
}

/// Collection of replacements.
#[derive(Default)]
pub struct Replacements(Vec<Replacement>);

impl Replacements {
  /// Creates a new collection of replacements.
  pub fn new(replacements: Vec<Replacement>) -> Self {
    Self(replacements)
  }

  /// Replaces all configured replacements in provided content.
  /// Returns the changed content.
  pub fn replace(&self, content: &str) -> String {
    let mut modified = content.to_string();
    for replacement in &self.0 {
      modified = replacement.replace(&modified);
    }
    modified
  }
}
