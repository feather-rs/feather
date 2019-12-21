//! Systems and components specific to player entities.

use crate::entity;
use crate::entity::NameComponent;
use crate::io::NewClientInfo;
use crate::network::Network;
use crate::state::State;
use legion::entity::Entity;
use mojang_api::ProfileProperty;

/// Profile properties of a player.
#[derive(Debug, Clone)]
pub struct ProfileProperties(pub Vec<ProfileProperty>);

/// Event triggered when a player joins.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerJoinEvent {
    pub player: Entity,
}

/// Tag used to mark a player.
///
/// Note that this is a _tag_, not a component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player;

/// Creates a new player from the given `NewClientInfo`.
///
/// This function also triggers the `PlayerJoinEvent` for this player.
pub fn create(state: &State, info: NewClientInfo) {
    entity::base(state, info.position)
        .with_tag(Player)
        .with_component(info.uuid)
        .with_component(Network {
            sender: info.sender,
            receiver: info.receiver,
        })
        .with_component(info.ip)
        .with_component(ProfileProperties(info.profile))
        .with_component(NameComponent(info.username))
        .with_exec(|world, scheduler, player| {
            scheduler.trigger(PlayerJoinEvent { player }, world);
        })
        .build();
}
