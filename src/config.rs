//!

use crate::replacement::{Replacement, Replacements};
use std::fs;

///
const FILE_NAME_PREFIX: &str = "yapp";

///
const ERR_LOAD: &str = "[ERROR][Yapp] loading configuration file failed with reason:";

///
const ERR_NOT_FOUND: &str = "[WARNING][Yapp] configuration file not found, in current directory expected a file with the name starting with prefix:";

///
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
                    eprintln!("{ERR_LOAD} {}", reason);
                    None
                  }
                };
              }
            }
            Err(reason) => {
              eprintln!("{ERR_LOAD} {}", reason);
              return None;
            }
          },
          Err(reason) => {
            eprintln!("{ERR_LOAD} {}", reason);
            return None;
          }
        }
      }
    }
    Err(reason) => {
      eprintln!("{ERR_LOAD} {}", reason);
      return None;
    }
  }
  eprintln!("{ERR_NOT_FOUND} {FILE_NAME_PREFIX}");
  None
}

///
pub fn load_config(content: &str) -> Option<Replacements> {
  let mut replacements = vec![];
  let lines: Vec<String> = content.lines().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
  for window in lines.chunks(2) {
    replacements.push(Replacement::new(window[0].clone(), window[1].clone()));
  }
  Some(Replacements::new(replacements))
}
