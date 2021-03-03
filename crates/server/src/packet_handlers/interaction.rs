use crate::{ClientId, Server};
use common::{entities::player::HotbarSlot, Game};
use ecs::{Entity, EntityRef, SysResult};
use protocol::packets::client::{
    BlockFace, HeldItemChange, PlayerBlockPlacement, PlayerDigging, PlayerDiggingStatus,
};

/// Handles the player block placement packet. Currently just removes the block client side for the player.
pub fn handle_player_block_placement(
    game: &Game,
    server: &mut Server,
    packet: PlayerBlockPlacement,
    player: EntityRef,
) -> SysResult {
    let position = match packet.face {
        BlockFace::Bottom => packet.position.down(),
        BlockFace::Top => packet.position.up(),
        BlockFace::North => packet.position.north(),
        BlockFace::South => packet.position.south(),
        BlockFace::West => packet.position.west(),
        BlockFace::East => packet.position.east(),
    };

    log::trace!("Got player block placement at {:?}", position);

    let client = server.clients.get(*player.get::<ClientId>()?).unwrap();
    let block_id = game.block(position).unwrap_or_default();

    client.send_block_change(position, block_id);
    Ok(())
}

/// Handles the Player Digging packet sent for the following
/// actions:
/// * Breaking blocks.
/// * Dropping items.
/// * Shooting arrows.
/// * Eating.
/// * Swapping items between the main and off hand.
pub fn handle_player_digging(game: &mut Game, packet: PlayerDigging, _player: Entity) -> SysResult {
    log::trace!("Got player digging with status {:?}", packet.status);
    match packet.status {
        PlayerDiggingStatus::StartDigging | PlayerDiggingStatus::CancelDigging => {
            game.break_block(packet.position);
            Ok(())
        }
        _ => Ok(()),
    }
}

pub fn handle_held_item_change(player: EntityRef, packet: HeldItemChange) -> SysResult {
    let new_id = packet.slot as usize;
    let mut slot = player.get_mut::<HotbarSlot>()?;

    log::trace!("Got player slot change from {} to {}", slot.get(), new_id);

    slot.set(new_id)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use common::Game;
    use protocol::packets::client::HeldItemChange;

    use super::*;

    #[test]
    fn held_item_change() {
        let mut game = Game::new();
        let entity = game.ecs.spawn((HotbarSlot::new(0),));
        let player = game.ecs.entity(entity).unwrap();

        let packet = HeldItemChange { slot: 8 };

        handle_held_item_change(player, packet).unwrap();

        assert_eq!(
            *game.ecs.get::<HotbarSlot>(entity).unwrap(),
            HotbarSlot::new(8)
        );
    }
}
