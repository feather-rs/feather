use smartstring::{LazyCompact, SmartString};
use std::{fmt::Display, ops::Deref};

/// Component storing an entity's username. (Usually
/// only players have this component.)
#[derive(Debug, PartialEq, Eq)]
pub struct Name(pub SmartString<LazyCompact>);

impl Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Component storing an entity's UUID.
pub use uuid::Uuid;
