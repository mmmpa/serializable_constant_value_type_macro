[![CircleCI](https://circleci.com/gh/mmmpa/serializable_constant_value_type_macro.svg?style=shield)](https://circleci.com/gh/mmmpa/serializable_constant_value_type_macro)

# serializable_constant_value_type_macro

Define types witch are serialized into constant string anytime.

```rust
use serde::{Serialize, Serializer};

define_constant_value!(
    MarkdownValue => "mrkdwn",
    FiveFiveFiveTo => 666,
);

#[derive(Default, Serialize)]
pub struct MarkdownText {
    pub type_name: MarkdownValue,
    pub value: String,
}

#[test]
fn test() {
    let j = serde_json::to_string(&MarkdownText::default()).unwrap();
    assert_eq!(r#"{"type_name":"mrkdwn","value":""}"#, j);

    let j = serde_json::to_string(&FiveFiveFiveTo).unwrap();
    assert_eq!("666", j);
}
```
