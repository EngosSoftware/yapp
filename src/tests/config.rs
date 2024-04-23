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

#[test]
fn config_single_quotation_mark_should_work() {
  let config = r#"

'  alfa  '
'  beta  '

  "#;

  let processor = load_config(config).unwrap();
  assert_eq!("│  beta  │ alfa │", processor.replace("│  alfa  │ alfa │"));
}

#[test]
fn config_single_quotation_mark_1_should_work() {
  let config = r#"

'  alfa  '
        beta

  "#;

  let processor = load_config(config).unwrap();
  assert_eq!("│beta│ alfa │", processor.replace("│  alfa  │ alfa │"));
}

#[test]
fn config_single_quotation_mark_2_should_work() {
  let config = r#"

  alfa
'     beta '

  "#;

  let processor = load_config(config).unwrap();
  assert_eq!("│     beta │      beta  │", processor.replace("│alfa│ alfa │"));
}

#[test]
fn config_single_quotation_mark_3_should_work() {
  let config = r#"

' alfa '
''beta''

  "#;

  let processor = load_config(config).unwrap();
  assert_eq!("│'beta'│'beta'│", processor.replace("│ alfa │ alfa │"));
}

#[test]
fn config_double_quotation_mark_should_work() {
  let config = r#"

"  alfa  "
"  beta  "

  "#;

  let processor = load_config(config).unwrap();
  assert_eq!("│  beta  │ alfa │", processor.replace("│  alfa  │ alfa │"));
}

#[test]
fn config_double_quotation_mark_1_should_work() {
  let config = r#"

"  alfa  "
        beta

  "#;

  let processor = load_config(config).unwrap();
  assert_eq!("│beta│ alfa │", processor.replace("│  alfa  │ alfa │"));
}

#[test]
fn config_double_quotation_mark_2_should_work() {
  let config = r#"

  alfa
"     beta "

  "#;

  let processor = load_config(config).unwrap();
  assert_eq!("│     beta │      beta  │", processor.replace("│alfa│ alfa │"));
}

#[test]
fn config_double_quotation_mark_3_should_work() {
  let config = r#"

" alfa "
""beta""

  "#;

  let processor = load_config(config).unwrap();
  assert_eq!(r#"│"beta"│"beta"│"#, processor.replace("│ alfa │ alfa │"));
}
