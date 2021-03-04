//! Defines components for all Minecraft entities.
//!
//! # Marker components
//! Each entity has a "marker component":
//! just a struct (often with no fields)
//! that signifies the type of an entity.
//!
//! For example, all horse entities have the [`Horse`]
//! marker component.
//!
//! For certain entities, these components also
//! contain data. For example, the [`Item`] marker
//! component (for item entities) has an `ItemStack`
//! field that indicates the type of the item.

#[doc(inline)]
pub use quill_common::entities::*;
