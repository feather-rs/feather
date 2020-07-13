//! Systems which handle packets.

mod animation;
mod chat;
mod client_status;
mod digging;
mod inventory;
mod movement;
mod placement;
mod use_item;
mod window;

pub use self::inventory::*;
pub use animation::handle_animation;
pub use chat::handle_chat;
pub use client_status::handle_client_status;
pub use digging::*;
use fecs::{Entity, World};
pub use movement::handle_movement_packets;
pub use placement::handle_player_block_placement;
pub use use_item::handle_player_use_item;
pub use window::handle_close_window;

/// Iterator filter to ensure players have not been removed from the world.
pub trait IteratorExt: Iterator {
    fn for_each_valid(self, world: &mut World, f: impl FnMut(&mut World, Self::Item));
}

impl<T, P> IteratorExt for T
where
    T: Iterator<Item = (Entity, P)>,
{
    fn for_each_valid(self, world: &mut World, mut f: impl FnMut(&mut World, Self::Item)) {
        self.for_each(move |(entity, packet)| {
            if !world.is_alive(entity) {
                return;
            }

            f(world, (entity, packet));
        })
    }
}
