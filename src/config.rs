//! # Configuration loader

use crate::replacement::{Replacement, Replacements};
use std::fs;

/// Configuration file prefix.
const FILE_NAME_PREFIX: &str = "yapp";

/// Configuration loading error message.
const ERROR_LOAD: &str = "[ERROR][Yapp] loading configuration file failed with reason:";

/// Configuration file not found warning message.
const WARNING_NOT_FOUND: &str = "[WARNING][Yapp] configuration file not found, in current directory expected a file with the name starting with prefix:";

/// Loads configuration from file.
pub fn load_config_from_file() -> Option<Replacements> {
  match fs::read_dir("./") {
    Ok(paths) => {
      for path in paths {
        match path {
          Ok(entry) => match entry.file_type() {
            Ok(file_type) => {
              if file_type.is_file() && entry.file_name().to_string_lossy().to_lowercase().starts_with(FILE_NAME_PREFIX) {
                return match fs::read_to_string(entry.file_name()) {
                  Ok(content) => load_config(&content),
                  Err(reason) => {
                    eprintln!("{ERROR_LOAD} {}", reason);
                    None
                  }
                };
              }
            }
            Err(reason) => {
              eprintln!("{ERROR_LOAD} {}", reason);
              return None;
            }
          },
          Err(reason) => {
            eprintln!("{ERROR_LOAD} {}", reason);
            return None;
          }
        }
      }
    }
    Err(reason) => {
      eprintln!("{ERROR_LOAD} {}", reason);
      return None;
    }
  }
  eprintln!("{WARNING_NOT_FOUND} {FILE_NAME_PREFIX}");
  None
}

/// Loads configuration from provided text.
pub fn load_config(content: &str) -> Option<Replacements> {
  let mut replacements = vec![];
  let lines: Vec<String> = content.lines().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
  for chunk in lines.chunks(2) {
    replacements.push(Replacement::new(trim_quotation_marks(&chunk[0]), trim_quotation_marks(&chunk[1])));
  }
  Some(Replacements::new(replacements))
}

fn trim_quotation_marks(input: &str) -> String {
  if (input.starts_with('"') && input.ends_with('"')) || (input.starts_with('\'') && input.ends_with('\'')) {
    input[1..input.len() - 1].to_string()
  } else {
    input.to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn trimming_single_quotation_should_work() {
    assert_eq!(r#"alfa"#, trim_quotation_marks(r#"'alfa'"#));
    assert_eq!(r#"'alfa'"#, trim_quotation_marks(r#"''alfa''"#));
    assert_eq!(r#"'alfa"#, trim_quotation_marks(r#"'alfa"#));
    assert_eq!(r#"alfa'"#, trim_quotation_marks(r#"alfa'"#));
    assert_eq!(r#"'alfa""#, trim_quotation_marks(r#"'alfa""#));
  }

  #[test]
  fn trimming_double_quotation_should_work() {
    assert_eq!(r#"alfa"#, trim_quotation_marks(r#""alfa""#));
    assert_eq!(r#""alfa""#, trim_quotation_marks(r#"""alfa"""#));
    assert_eq!(r#""alfa"#, trim_quotation_marks(r#""alfa"#));
    assert_eq!(r#"alfa""#, trim_quotation_marks(r#"alfa""#));
    assert_eq!(r#"'alfa""#, trim_quotation_marks(r#"'alfa""#));
  }
}
