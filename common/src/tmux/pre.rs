use serde::{Deserialize, Deserializer, Serialize};
use std::slice::Iter;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Pre(pub Vec<String>);

impl Pre {
    pub fn iter(&self) -> Iter<'_, String> {
        self.0.iter()
    }
}

/// Represents a wrapper type Pre for holding a vector of strings,
/// with custom deserialization logic supporting both single strings
/// and lists of strings.
///
/// # Deserialization
///
/// `Pre` can be deserialized from either:
/// - a single string (becoming a single-element vector; empty string becomes an empty vector)
/// - a sequence of strings (vector of strings)
///
/// This is useful for configuration formats (like YAML or JSON) where a field may
/// be specified as a single string or as a list for convenience.
///
/// # Example
///
/// ```yaml
/// # Both forms are valid:
/// pre: "echo hi"
/// pre: ["echo hi", "echo again"]
/// ```
/// Either variant will deserialize into a `Pre` struct for ergonomic downstream handling.
impl<'de> Deserialize<'de> for Pre {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PreVisitor;

        impl<'de> serde::de::Visitor<'de> for PreVisitor {
            type Value = Pre;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string or a list of strings")
            }

            fn visit_str<E>(self, value: &str) -> Result<Pre, E>
            where
                E: serde::de::Error,
            {
                if value.is_empty() {
                    Ok(Pre(vec![]))
                } else {
                    Ok(Pre(vec![value.to_string()]))
                }
            }

            fn visit_seq<S>(self, mut seq: S) -> Result<Pre, S::Error>
            where
                S: serde::de::SeqAccess<'de>,
            {
                let mut vec = Vec::new();
                while let Some(s) = seq.next_element::<String>()? {
                    vec.push(s);
                }
                Ok(Pre(vec))
            }
        }

        deserializer.deserialize_any(PreVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_single_string() {
        let yaml = "hello";
        let p: Pre = serde_saphyr::from_str(yaml).unwrap();
        assert_eq!(p, Pre(vec!["hello".to_string()]));
    }

    #[test]
    fn test_deserialize_list_of_strings() {
        let yaml = "- foo\n- bar\n";
        let p: Pre = serde_saphyr::from_str(yaml).unwrap();
        assert_eq!(p, Pre(vec!["foo".to_string(), "bar".to_string()]));
    }

    #[test]
    fn test_deserialize_empty_array() {
        let yaml = "[]";
        let p: Pre = serde_saphyr::from_str(yaml).unwrap();
        assert_eq!(p, Pre(vec![]));
    }

    #[test]
    fn test_deserialize_empty_string() {
        let yaml = "\"\"";
        let p: Pre = serde_saphyr::from_str(yaml).unwrap();
        assert_eq!(p, Pre(vec![]));
    }
}
