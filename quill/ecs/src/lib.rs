#![allow(unused)] // TEMP (remove before merge)
#![allow(unstable_name_collisions)]

mod bundle;
mod component;
mod ecs;
mod entity;
mod entity_builder;
mod layout_ext;
mod space;
mod storage;

pub use component::{Component, ComponentTypeId};
pub use ecs::{ComponentError, Ecs, EntityDead};
pub use entity::EntityId;
pub use entity_builder::EntityBuilder;
pub use space::MemorySpace;
