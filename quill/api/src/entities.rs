//! Defines components for all Minecraft entities.
//!
//! # Entity wrappers
//! In an ECS, components provide access to raw _data_,
//! like an entity's position, health, inventory, etc.
//! But components don't have to abstract high-level
//! methods that need access to multiple components. For example,
//! "deal damage" requires access to `Health` and `Inventory` (to handle armor),
//! and possibly potion effects.
//!
//! To solve this problem, Quill provides entity _wrappers_ that provide
//! these high-level methods. Each entity has a wrapper struct
//! that implements a series of traits. For example, the `Player` wrapper
//! struct provides the `send_message` function via the `SendMessage` trait.
//! Similarly, the `DealDamage` trait provides the `damage()` method
//! for entities with health.
//!
//! # Marker components
//! Each entity has a "marker component":
//! just a struct (often with no fields)
//! that signifies the type of an entity.
//!
//! For example, all horse entities have the [`HorseMarker`]
//! marker component.
//!
//! For certain entities, these components also
//! contain data. For example, the [`Item`] marker
//! component (for item entities) has an `ItemStack`
//! field that indicates the type of the item.

#[doc(inline)]
pub use quill_common::entities::*;
