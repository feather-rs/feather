use crate::IteratorExt;
use feather_core::{inventory::Window, network::packets::CloseWindowServerbound};
use feather_server_types::{Game, PacketBuffers, WindowCloseEvent};
use fecs::{Entity, World};
use smallvec::SmallVec;
use std::sync::Arc;

/// When a client sends Close Window, resets their `Window`
/// to the normal player window.
#[fecs::system]
pub fn handle_close_window(
    game: &mut Game,
    world: &mut World,
    packet_buffers: &Arc<PacketBuffers>,
) {
    packet_buffers
        .received::<CloseWindowServerbound>()
        .for_each_valid(world, |world, (player, _packet)| {
            // TODO: at some point, there should be a more rigorous window ID/window handling system

            let windows_closed: SmallVec<[Entity; 2]> = {
                let mut window = world.get_mut::<Window>(player);
                let windows_closed = window.wrapped_entities().into();
                *window = Window::player(player);
                windows_closed
            };

            for closed in windows_closed {
                game.handle(world, WindowCloseEvent { player, closed });
            }
        });
}
