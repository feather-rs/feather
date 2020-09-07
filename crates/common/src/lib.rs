//! Gameplay functionality: entities, components, systems, ...
//!
//! This crate implements most functionality which is generic between
//! client and server, i.e., which does not involve interaction with the network.

use base::Setup;

pub mod entity;

pub fn setup(_setup: &mut Setup) {}
