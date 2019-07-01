use crate::io::ServerToWorkerMessage;
use crate::prelude::*;
use crate::{initialhandler as ih, Entity, State, remove_player};
use feather_core::network::packet::{Packet};
use mio_extras::channel::{Receiver, Sender};

//const MAX_KEEP_ALIVE_TIME: u64 = 30;

pub struct NetworkComponent {
    sender: Sender<ServerToWorkerMessage>,
    receiver: Receiver<ServerToWorkerMessage>,

    //last_keep_alive_time: u64,
}

pub fn network_system(state: &mut State) {
    let mut players_to_remove = vec![];

    for player in state.players.clone() {
        while let Ok(msg) = state.network_components[player].receiver.try_recv() {
            match msg {
                ServerToWorkerMessage::NotifyPacketReceived(packet) => {
                    if let Some(_) = state.ih_components.get(player) {
                        if let Err(e) = ih::handle_packet(state, player, packet) {
                            info!("Disconnecting player: {}", e);
                            remove_player(state, player);
                        }
                    } else {
                        // TODO
                    }
                },
                ServerToWorkerMessage::NotifyDisconnect => {
                    players_to_remove.push(player);
                    info!("Player disconnected");
                },
                _ => panic!("Invalid message received from worker thread"),
            }
        }
    }

    for _player in players_to_remove {
        remove_player(state, _player);
    }
}

pub fn send_packet_to_player<P: Packet + 'static>(state: &State, player: Entity, packet: P) {
    let comp = &state.network_components[player];
    let _ = comp.sender.send(ServerToWorkerMessage::SendPacket(Box::new(packet)));
}

pub fn enable_compression_for_player(state: &State, player: Entity, threshold: usize) {
    let comp = &state.network_components[player];
    let _ = comp.sender.send(ServerToWorkerMessage::EnableCompression(threshold));
}

pub fn enable_encryption_for_player(state: &State, player: Entity, key: [u8; 16]) {
    let comp = &state.network_components[player];
    let _ = comp.sender.send(ServerToWorkerMessage::EnableEncryption(key));
}

/// Calculates the relative move fields
/// as used in the EntityRelativeMove packets.
pub fn calculate_relative_move(old: Position, current: Position) -> (u16, u16, u16) {
    let x = ((current.x * 32.0 - old.x * 32.0) * 128.0) as u16;
    let y = ((current.y * 32.0 - old.x * 32.0) * 128.0) as u16;
    let z = ((current.z * 32.0 - old.z * 32.0) * 128.0) as u16;
    (x, y, z)
}
