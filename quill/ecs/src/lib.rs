#![allow(unused)] // TEMP (remove before merge)
#![allow(unstable_name_collisions)]

mod borrow;
mod bundle;
mod component;
mod entity;
mod entity_builder;
mod layout_ext;
mod query;
mod storage;
mod world;

pub use component::Component;
pub use entity::EntityId;
pub use entity_builder::EntityBuilder;
pub use query::{QueryDriver, QueryItem};
pub use storage::{SparseSetRef, SparseSetStorage};
pub use world::{ComponentError, Components, EntityDead, World};
