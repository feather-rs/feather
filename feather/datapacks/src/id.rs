use crate::DEFAULT_NAMESPACE;
use serde::{de, Deserialize, Serialize};
use smartstring::{LazyCompact, SmartString};
use std::{
    fmt::{self, Display},
    str::FromStr,
};

/// A namespaced identifier, also known as a "resource location"
/// in Forge. See <https://minecraft.gamepedia.com/Namespaced_ID>.
///
/// Namespaced IDs can be parsed using the `FromStr` implementation,
/// and they can be formatted using the `Display` impl or by calling `to_string()`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NamespacedId {
    // Smart (inlineable) strings are used to reduce heap
    // fragmentation and memory usage.
    namespace: SmartString<LazyCompact>,
    name: SmartString<LazyCompact>,
}

impl NamespacedId {
    /// Returns the namespace for this ID.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the name for this ID.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Error returned when a namespaced ID was formatted incorrectly.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ParseError {
    #[error("'{0}' is not a valid character for namespaces")]
    InvalidNamespaceChar(char),
    #[error("'{0}' is not a valid character for namespaced ID names")]
    InvalidNameChar(char),
}

impl FromStr for NamespacedId {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Determine the namespace and name components.
        let mut parts = s.split(':');
        let part1 = parts.next().unwrap_or("");
        let part2 = parts.next();

        let (namespace, name) = if let Some(part2) = part2 {
            (part1, part2)
        } else {
            (DEFAULT_NAMESPACE, part1)
        };

        // Ensure that the namespace and name are legal.
        validate_namespace(namespace)?;
        validate_name(name)?;

        Ok(NamespacedId {
            namespace: SmartString::from(namespace),
            name: SmartString::from(name),
        })
    }
}

fn validate_namespace(namespace: &str) -> Result<(), ParseError> {
    for c in namespace.chars() {
        if !is_valid_namespace_char(c) {
            return Err(ParseError::InvalidNamespaceChar(c));
        }
    }
    Ok(())
}

fn validate_name(name: &str) -> Result<(), ParseError> {
    for c in name.chars() {
        if !is_valid_name_char(c) {
            return Err(ParseError::InvalidNameChar(c));
        }
    }
    Ok(())
}

fn is_valid_namespace_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-'
}

fn is_valid_name_char(c: char) -> bool {
    // Names can have some extra characters in addition
    // to those allowed for namespaces.
    is_valid_namespace_char(c) || c == '/' || c == '.'
}

impl<'de> Deserialize<'de> for NamespacedId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = SmartString::<LazyCompact>::deserialize(deserializer)?;

        NamespacedId::from_str(&string).map_err(|e| de::Error::custom(e.to_string()))
    }
}

impl Display for NamespacedId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.name)
    }
}

impl Serialize for NamespacedId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legal_namespace_chars() {
        for c in ('a'..='z').chain('A'..='Z').chain('0'..='9') {
            assert!(is_valid_name_char(c));
            assert!(is_valid_namespace_char(c));
        }
    }

    #[test]
    fn legal_name_chars() {
        for &c in &['/', '.'] {
            assert!(is_valid_name_char(c));
            assert!(!is_valid_namespace_char(c));
        }
    }

    #[test]
    fn illegal_namespace_chars() {
        for &c in &['/', '.', '\\', '\n', '@', '%', '?'] {
            assert!(!is_valid_namespace_char(c));
        }
    }

    #[test]
    fn illegal_name_chars() {
        for &c in &['\\', '\n', '@', '%', '?'] {
            assert!(!is_valid_namespace_char(c));
        }
    }

    #[test]
    fn parse_id_with_namespace() {
        let id = NamespacedId::from_str("namespace-caelunshun_66:folder/file.ext").unwrap();
        assert_eq!(&id.name, "folder/file.ext");
        assert_eq!(&id.namespace, "namespace-caelunshun_66");
    }

    #[test]
    fn parse_id_with_default_namespace() {
        let id = NamespacedId::from_str("acacia_leaves-2").unwrap();
        assert_eq!(&id.name, "acacia_leaves-2");
        assert_eq!(&id.namespace, DEFAULT_NAMESPACE);
    }

    #[test]
    fn parse_empty_id() {
        let id = NamespacedId::from_str("").unwrap();
        assert_eq!(id.name(), "");
        assert_eq!(id.namespace(), DEFAULT_NAMESPACE);
    }

    #[test]
    fn parse_id_with_invalid_namespace() {
        assert_eq!(
            NamespacedId::from_str("ewhi@iho:name"),
            Err(ParseError::InvalidNamespaceChar('@'))
        );
        assert_eq!(
            NamespacedId::from_str("dir1/dir2:file"),
            Err(ParseError::InvalidNamespaceChar('/'))
        );
    }

    #[test]
    fn parse_id_with_invalid_name() {
        assert_eq!(
            NamespacedId::from_str("name^"),
            Err(ParseError::InvalidNameChar('^'))
        );
        assert_eq!(
            NamespacedId::from_str("spaces galore"),
            Err(ParseError::InvalidNameChar(' '))
        );
    }

    #[test]
    fn formatting() {
        let id = NamespacedId::from_str("namespace:name").unwrap();
        assert_eq!(id.to_string(), "namespace:name");
    }

    #[test]
    fn formatting_default_namespace() {
        let id = NamespacedId::from_str("name").unwrap();
        assert_eq!(id.to_string(), "minecraft:name");
    }
}
