//! Gameplay functionality: entities, components, systems, ...
//!
//! This crate implements most functionality which is generic between
//! client and server, i.e., which does not involve interaction with the network.

use smartstring::{LazyCompact, SmartString};
use std::ops::Deref;

pub mod entity;

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

