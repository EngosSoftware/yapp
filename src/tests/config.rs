//!

use crate::config::load_config;

#[test]
fn loading_config_should_work() {
  let config = r#"
  
  alfa
  beta

  @version 
  0.1.3

  "#;

  let processor = load_config(config).unwrap();
  assert_eq!("a beta b 0.1.3 c", processor.replace("a alfa b @version c"));
}
