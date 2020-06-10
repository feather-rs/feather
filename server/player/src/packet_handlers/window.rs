use crate::IteratorExt;
use feather_core::{inventory::Window, network::packets::CloseWindowServerbound};
use feather_server_types::{Game, PacketBuffers, WindowCloseEvent};
use fecs::{Entity, World};
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

            let entity_closed = {
                let mut window = world.get_mut::<Window>(player);
                let entity_closed = entity_closed(&*window);
                *window = Window::player(player);
                entity_closed
            };

            if let Some(closed) = entity_closed {
                game.handle(world, WindowCloseEvent { player, closed });
            }
        });
}

fn entity_closed(window: &Window) -> Option<Entity> {
    window.wrapped_entities().first().copied()
}
