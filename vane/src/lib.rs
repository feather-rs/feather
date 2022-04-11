#![allow(unused)] // TEMP (remove before merge)
#![allow(unstable_name_collisions)]

mod borrow;
mod bundle;
mod bus;
mod component;
mod entity;
mod entity_builder;
mod entity_ref;
mod event;
mod layout_ext;
mod query;
mod resources;
mod storage;
mod system;
mod world;

pub use borrow::{BorrowError, BorrowFlag, Ref, RefMut};
pub use bus::Bus;
pub use component::Component;
pub use entity::Entity;
pub use entity_builder::EntityBuilder;
pub use entity_ref::EntityRef;
pub use query::{QueryDriver, QueryItem};
pub use resources::{ResourceError, Resources};
pub use storage::{SparseSetRef, SparseSetStorage};
pub use system::{HasEntities, HasResources, SysResult, SystemExecutor};
pub use world::{ComponentError, Components, Entities, EntityDead};
