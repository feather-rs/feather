use crate::entity::{EntityComponent, PlayerComponent};
use crate::joinhandler::PlayerJoinEvent;
use crate::network::{send_packet_to_player, NetworkComponent};
use crate::player::InventoryComponent;
use feather_core::entitymeta::{EntityMetadata, MetaEntry};
use feather_core::inventory::{SLOT_ENTITY_EQUIPMENT_MAIN_HAND, SLOT_HOTBAR_OFFSET};
use feather_core::network::packet::implementation::{
    EntityEquipment, PlayerInfo, PlayerInfoAction, SpawnPlayer,
};
use feather_core::Gamemode;
use shrev::EventChannel;
use specs::SystemData;
use specs::{Entities, Entity, Join, Read, ReadStorage, ReaderId, System, World};
use uuid::Uuid;

/// System for broadcasting when a player joins
/// the game. Also spawns other players to
/// the player's client.
#[derive(Default)]
pub struct JoinBroadcastSystem {
    reader: Option<ReaderId<PlayerJoinEvent>>,
}

impl JoinBroadcastSystem {
    pub fn new() -> Self {
        Self { reader: None }
    }
}

impl<'a> System<'a> for JoinBroadcastSystem {
    type SystemData = (
        Read<'a, EventChannel<PlayerJoinEvent>>,
        ReadStorage<'a, EntityComponent>,
        ReadStorage<'a, PlayerComponent>,
        ReadStorage<'a, NetworkComponent>,
        ReadStorage<'a, InventoryComponent>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (join_events, entity_comps, player_comps, net_comps, inventories, entities) = data;

        for event in join_events.read(&mut self.reader.as_mut().unwrap()) {
            // Broadcast join
            let entity_comp = entity_comps.get(event.player).unwrap();
            let player_comp = player_comps.get(event.player).unwrap();

            let (player_info, spawn_player) =
                get_player_initialization_packets(entity_comp, player_comp, event.player);

            for (player, net) in (&entities, &net_comps).join() {
                // Send player info to the player who joined
                // so they can see themselves in the tablist,
                // but don't send spawn player.
                send_packet_to_player(net, player_info.clone());
                if player != event.player {
                    send_packet_to_player(net, spawn_player.clone());
                }
            }

            let net_comp = net_comps.get(event.player).unwrap();

            // Send existing players to new player
            for (entity_comp, player_comp, entity, inventory) in
                (&entity_comps, &player_comps, &entities, &inventories).join()
            {
                if entity != event.player {
                    let (player_info, spawn_player) =
                        get_player_initialization_packets(entity_comp, player_comp, entity);
                    send_packet_to_player(net_comp, player_info);
                    send_packet_to_player(net_comp, spawn_player);

                    let slot = inventory.item_at(SLOT_HOTBAR_OFFSET + inventory.held_item);
                    let entity_equipment = EntityEquipment::new(
                        entity.id() as i32,
                        SLOT_ENTITY_EQUIPMENT_MAIN_HAND as i32,
                        slot.cloned(),
                    );
                    send_packet_to_player(net_comp, entity_equipment);
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerJoinEvent>>()
                .register_reader(),
        );
    }
}

/// Returns the player info and spawn player packets
/// for the given player.
fn get_player_initialization_packets(
    ecomp: &EntityComponent,
    pcomp: &PlayerComponent,
    player: Entity,
) -> (PlayerInfo, SpawnPlayer) {
    let display_name = json!({
        "text": ecomp.display_name
    })
    .to_string();

    let mut props = vec![];
    for prop in pcomp.profile_properties.iter() {
        props.push((
            prop.name.clone(),
            prop.value.clone(),
            prop.signature.clone(),
        ));
    }

    let action = PlayerInfoAction::AddPlayer(
        ecomp.display_name.clone(),
        props,
        Gamemode::Creative,
        50,
        display_name,
    );
    let player_info = PlayerInfo::new(action, ecomp.uuid);

    let metadata = EntityMetadata::new().with(&[
        (0, MetaEntry::Byte(0)),
        (1, MetaEntry::VarInt(300)),
        (2, MetaEntry::OptChat(None)),
        (3, MetaEntry::Boolean(false)),
        (4, MetaEntry::Boolean(false)),
        (5, MetaEntry::Boolean(false)),
        (6, MetaEntry::Byte(0)),
        (7, MetaEntry::Float(1.0)),
        (8, MetaEntry::VarInt(0)),
        (9, MetaEntry::Boolean(false)),
        (10, MetaEntry::VarInt(0)),
        (11, MetaEntry::Float(0.0)),
        (12, MetaEntry::VarInt(0)),
        (13, MetaEntry::Byte(0)),
        (14, MetaEntry::Byte(1)),
        // TODO NBT
    ]);

    let spawn_player = SpawnPlayer::new(
        player.id() as i32,
        ecomp.uuid,
        ecomp.position.x,
        ecomp.position.y,
        ecomp.position.z,
        degrees_to_stops(ecomp.position.pitch),
        degrees_to_stops(ecomp.position.yaw),
        metadata,
    );

    (player_info, spawn_player)
}

/// Event which is called when a player disconnected.
pub struct PlayerDisconnectEvent {
    pub player: Entity,
    pub reason: String,
    pub uuid: Uuid,
}

/// System for broadcasting when a player disconnects.
#[derive(Default)]
pub struct DisconnectBroadcastSystem {
    reader: Option<ReaderId<PlayerDisconnectEvent>>,
}

impl DisconnectBroadcastSystem {
    pub fn new() -> Self {
        Self { reader: None }
    }
}

impl<'a> System<'a> for DisconnectBroadcastSystem {
    type SystemData = (
        Read<'a, EventChannel<PlayerDisconnectEvent>>,
        ReadStorage<'a, NetworkComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (events, net_comps) = data;

        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            // Broadcast disconnect.
            // Note that the Destroy Entity packet is sent
            // in a separate system (crate::entity::EntityDestroyBroadcastSystem).
            // This system only updates the tablist for all clients.
            let player_info = PlayerInfo::new(PlayerInfoAction::RemovePlayer, event.uuid);

            for net in net_comps.join() {
                send_packet_to_player(net, player_info.clone());
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerDisconnectEvent>>()
                .register_reader(),
        );
    }
}

fn degrees_to_stops(degs: f32) -> u8 {
    ((degs / 360.0) * 256.0) as u8
}
