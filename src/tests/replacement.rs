//! # Unit tests for replacement functionality

use crate::replacement::Replacement;

#[test]
fn creating_replacement_should_work() {
  let replacement = Replacement::new("alfa".to_string(), "beta".to_string());
  assert_eq!("a beta b", replacement.replace("a alfa b"))
}
