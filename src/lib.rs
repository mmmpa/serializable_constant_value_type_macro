#[macro_export]
macro_rules! define_serializable_constant_value_type {
    ( $($name:tt => $value:tt,)* ) => { $(
        #[derive(Debug, Default)]
        pub struct $name;

        impl Serialize for $name {
            fn serialize<S>(&self, s: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
            where
                S: Serializer,
            {
                $value.serialize(s)
            }
        }
    )* };
    ( $($name:tt => $value:tt),* ) => { $(
        define_constant_value!($name => $value,);
    )* };
}

#[cfg(test)]
pub mod tests {
    use serde::{Serialize, Serializer};

    define_serializable_constant_value_type!(
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
}
