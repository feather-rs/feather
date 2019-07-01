use crate::io::ServerToWorkerMessage;
use crate::prelude::*;
use crate::{initialhandler as ih, Entity, State};
use feather_core::network::packet::{implementation::*, Packet, PacketType};
use mio_extras::channel::{Receiver, Sender};
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_KEEP_ALIVE_TIME: u64 = 30;

pub struct NetworkComponent {
    sender: Sender<ServerToWorkerMessage>,
    receiver: Receiver<ServerToWorkerMessage>,

    last_keep_alive_time: u64,
}

pub fn network_system(state: &mut State) {
    let mut players_to_remove = vec![];

    for player in &state.players {
        while let Ok(msg) = state.network_components[*player].receiver.try_recv() {
            match msg {
                ServerToWorkerMessage::NotifyPacketReceived(packet) => {

                },
                ServerToWorkerMessage::NotifyDisconnect => {
                    players_to_remove.push(*player);
                    info!("Player disconnected");
                },
                _ => panic!("Invalid message received from worker thread"),
            }
        }
    }

    for _player in players_to_remove {
        state.remove_entity(_player);
        state.players.retain(|p| *p != _player);
    }
}

fn current_time_in_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Calculates the relative move fields
/// as used in the EntityRelativeMove packets.
pub fn calculate_relative_move(old: Position, current: Position) -> (u16, u16, u16) {
    let x = ((current.x * 32.0 - old.x * 32.0) * 128.0) as u16;
    let y = ((current.y * 32.0 - old.x * 32.0) * 128.0) as u16;
    let z = ((current.z * 32.0 - old.z * 32.0) * 128.0) as u16;
    (x, y, z)
}
