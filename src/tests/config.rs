//! # Unit tests for configuration

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
  assert_eq!("│ beta │ 0.1.3 │", processor.replace("│ alfa │ @version │"));
}
