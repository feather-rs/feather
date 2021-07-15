#![allow(unused)] // TEMP (remove before merge)
#![allow(unstable_name_collisions)]

mod borrow;
mod bundle;
mod component;
mod entity;
mod entity_builder;
mod event;
mod layout_ext;
mod query;
mod resources;
mod storage;
mod system;
mod world;

pub use borrow::{BorrowError, BorrowFlag, Ref, RefMut};
pub use component::Component;
pub use entity::EntityId;
pub use entity_builder::EntityBuilder;
pub use query::{QueryDriver, QueryItem};
pub use resources::{ResourceError, Resources};
pub use storage::{SparseSetRef, SparseSetStorage};
pub use system::{HasResources, HasWorld, SysResult, SystemExecutor};
pub use world::{ComponentError, Components, EntityDead, World};
