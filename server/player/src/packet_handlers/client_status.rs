use crate::packet_handlers::IteratorExt;
use feather_core::network::packets::ClientStatus;
use feather_core::network::packets::Respawn;
use feather_core::util::{Gamemode, Position};
use feather_server_types::{Dead, Health, Network, PacketBuffers, Teleported};
use fecs::World;
use std::sync::Arc;

/// Handles the Client Status packet, which is sent
/// when the user clicks the respawn button.
#[fecs::system]
pub fn handle_client_status(world: &mut World, packet_buffers: &Arc<PacketBuffers>) {
    packet_buffers
        .received::<ClientStatus>()
        .for_each_valid(world, |world, (player, packet)| {
            match packet.action_id {
                0 => {
                    // Perform respawn
                    let _ = world.remove::<Dead>(player);

                    // TODO: support spawn positons
                    *world.get_mut::<Position>(player) = Position::default();

                    world.get_mut::<Health>(player).0 = 20;

                    world.add(player, Teleported).unwrap();

                    let gamemode = *world.get::<Gamemode>(player);

                    // Send Respawn packet
                    let packet = Respawn {
                        dimension: 0,
                        difficulty: 1,
                        gamemode: gamemode.id() as u8,
                        level_type: String::from("default"),
                    };
                    world.get::<Network>(player).send(packet);
                }
                x => log::debug!("Unimplemented Client Status action ID {}", x),
            }
        });
}
