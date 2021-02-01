use common::Game;
use ecs::{Entity, SysResult};
use protocol::ClientPlayPacket;

use crate::Server;

mod movement;

/// Handles a packet received from a client.
pub fn handle_packet(
    game: &mut Game,
    _server: &mut Server,
    player: Entity,
    packet: ClientPlayPacket,
) -> SysResult {
    let player = game.ecs.entity(player)?;
    match packet {
        ClientPlayPacket::PlayerPosition(packet) => {
            movement::handle_player_position(player, packet)
        }
        ClientPlayPacket::PlayerPositionAndRotation(packet) => {
            movement::handle_player_position_and_rotation(player, packet)
        }
        ClientPlayPacket::PlayerRotation(packet) => {
            movement::handle_player_rotation(player, packet)
        }
        ClientPlayPacket::PlayerMovement(packet) => {
            movement::handle_player_movement(player, packet)
        }

        ClientPlayPacket::TeleportConfirm(_)
        | ClientPlayPacket::QueryBlockNbt(_)
        | ClientPlayPacket::SetDifficulty(_)
        | ClientPlayPacket::ChatMessage(_)
        | ClientPlayPacket::ClientStatus(_)
        | ClientPlayPacket::ClientSettings(_)
        | ClientPlayPacket::TabComplete(_)
        | ClientPlayPacket::WindowConfirmation(_)
        | ClientPlayPacket::ClickWindowButton(_)
        | ClientPlayPacket::ClickWindow(_)
        | ClientPlayPacket::CloseWindow(_)
        | ClientPlayPacket::PluginMessage(_)
        | ClientPlayPacket::EditBook(_)
        | ClientPlayPacket::QueryEntityNbt(_)
        | ClientPlayPacket::InteractEntity(_)
        | ClientPlayPacket::GenerateStructure(_)
        | ClientPlayPacket::KeepAlive(_)
        | ClientPlayPacket::LockDifficulty(_)
        | ClientPlayPacket::VehicleMove(_)
        | ClientPlayPacket::SteerBoat(_)
        | ClientPlayPacket::PickItem(_)
        | ClientPlayPacket::CraftRecipeRequest(_)
        | ClientPlayPacket::PlayerAbilities(_)
        | ClientPlayPacket::PlayerDigging(_)
        | ClientPlayPacket::EntityAction(_)
        | ClientPlayPacket::SteerVehicle(_)
        | ClientPlayPacket::SetDisplayedRecipe(_)
        | ClientPlayPacket::SetRecipeBookState(_)
        | ClientPlayPacket::NameItem(_)
        | ClientPlayPacket::ResourcePackStatus(_)
        | ClientPlayPacket::AdvancementTab(_)
        | ClientPlayPacket::SelectTrade(_)
        | ClientPlayPacket::SetBeaconEffect(_)
        | ClientPlayPacket::HeldItemChange(_)
        | ClientPlayPacket::UpdateCommandBlock(_)
        | ClientPlayPacket::UpdateCommandBlockMinecart(_)
        | ClientPlayPacket::CreativeInventoryAction(_)
        | ClientPlayPacket::UpdateJigsawBlock(_)
        | ClientPlayPacket::UpdateStructureBlock(_)
        | ClientPlayPacket::UpdateSign(_)
        | ClientPlayPacket::Animation(_)
        | ClientPlayPacket::Spectate(_)
        | ClientPlayPacket::PlayerBlockPlacement(_)
        | ClientPlayPacket::UseItem(_) => Ok(()),
    }
}
