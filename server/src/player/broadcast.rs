use crate::entity::{EntityComponent, PlayerComponent};
use crate::joinhandler::PlayerJoinEvent;
use crate::network::{send_packet_to_all_players, send_packet_to_player, NetworkComponent};
use feather_core::network::packet::implementation::{PlayerInfo, PlayerInfoAction, SpawnPlayer};
use feather_core::Gamemode;
use shrev::EventChannel;
use specs::SystemData;
use specs::{Entities, Entity, Join, Read, ReadStorage, ReaderId, System, World};
use uuid::Uuid;

/// System for broadcasting when a player joins
/// the game.
///
/// This system only broadcasts the
/// Player Info packet necessary to view to player
/// in the tablist - the `EntityBroadcastSystem` handles
/// the Spawn Player packet.
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
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (join_events, entity_comps, player_comps, net_comps, entities) = data;

        for event in join_events.read(&mut self.reader.as_mut().unwrap()) {
            // Broadcast join
            let entity_comp = entity_comps.get(event.player).unwrap();
            let player_comp = player_comps.get(event.player).unwrap();

            let player_info = get_player_initialization_packet(entity_comp, player_comp);

            send_packet_to_all_players(&net_comps, &entities, player_info, None);

            let net_comp = net_comps.get(event.player).unwrap();

            // Send existing players to new player
            for (entity_comp, player_comp, entity) in
                (&entity_comps, &player_comps, &entities).join()
            {
                if entity != event.player {
                    let player_info = get_player_initialization_packet(entity_comp, player_comp);
                    send_packet_to_player(net_comp, player_info);

                    let spawn_player = SpawnPlayer {
                        entity_id: entity.id() as i32,
                        player_uuid: entity_comp.uuid,
                        x: entity_comp.position.x,
                        y: entity_comp.position.y,
                        z: entity_comp.position.z,
                        yaw: degrees_to_stops(entity_comp.position.yaw),
                        pitch: degrees_to_stops(entity_comp.position.pitch),
                        metadata: Default::default(),
                    };
                    send_packet_to_player(net_comp, spawn_player);
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

/// Returns the player info packet
/// for the given player.
fn get_player_initialization_packet(
    ecomp: &EntityComponent,
    pcomp: &PlayerComponent,
) -> PlayerInfo {
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
    PlayerInfo::new(action, ecomp.uuid)
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
