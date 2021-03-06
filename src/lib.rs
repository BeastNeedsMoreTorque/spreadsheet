pub mod parser;

pub mod util;

pub mod parser_util;

pub mod eval_stack;

pub mod eval;

pub mod definitions;

pub mod workbook;

pub mod compute;

//use im::HashMap;

// fn foo() -> HashMap<String, String> {
//     HashMap::from(vec![
//         ("foo".to_string(), "bar".to_string()),
//         ("moose".to_string(), "goose".to_string()),
//     ])
// }

// #[test]
// fn test_foo() {
//     let mut hm = foo();
//     assert_eq!(hm.get("foo"), Some(&"bar".to_string()));
//     assert_eq!(hm.get("moose"), Some(&"goose".to_string()));
//     let hm2 = hm.clone();
//     hm = hm.update("foo".to_string(), "sloth".to_string());
//     hm = hm.update("dog".to_string(), "woof".to_string());
//     assert_eq!(hm.get("foo"), Some(&"sloth".to_string()));
//     let baz = foo() + hm ;
//     assert_eq!(baz.get("foo"), Some(&"bar".to_string()));
//     assert_eq!(baz.get("dog"), Some(&"woof".to_string()));
//     assert_eq!(hm2.get("foo"), Some(&"bar".to_string()));
//     assert_eq!(hm2.get("dog"), None);
// }
