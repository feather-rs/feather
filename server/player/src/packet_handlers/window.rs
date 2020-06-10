use crate::IteratorExt;
use feather_core::{inventory::Window, network::packets::CloseWindowServerbound};
use feather_server_types::PacketBuffers;
use fecs::World;
use std::sync::Arc;

/// When a client sends Close Window, resets their `Window`
/// to the normal player window.
#[fecs::system]
pub fn handle_close_window(world: &mut World, packet_buffers: &Arc<PacketBuffers>) {
    packet_buffers
        .received::<CloseWindowServerbound>()
        .for_each_valid(world, |world, (player, _packet)| {
            // TODO: at some point, there should be a more rigorous window ID/window handling system
            *world.get_mut::<Window>(player) = Window::player(player);
        });
}
