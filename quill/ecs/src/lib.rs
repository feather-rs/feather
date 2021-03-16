#![allow(unused)] // TEMP (remove before merge)
#![allow(unstable_name_collisions)]
#![feature(generic_associated_types)] // TEMP: need to figure out how to remove this before merge (needed for queries)
#![allow(incomplete_features)]

mod borrow;
mod bundle;
mod component;
mod ecs;
mod entity;
mod entity_builder;
mod layout_ext;
mod query;
mod storage;

pub use component::Component;
pub use ecs::{ComponentError, Components, Ecs, EntityDead};
pub use entity::EntityId;
pub use entity_builder::EntityBuilder;
pub use query::{QueryDriver, QueryItem, QueryParameter, QueryTuple};
pub use storage::{SparseSetRef, SparseSetStorage};
