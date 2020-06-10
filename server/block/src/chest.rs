use feather_core::util::BlockPosition;
use feather_core::{
    blocks::BlockKind,
    network::{packets::BlockAction, Packet},
};
use feather_server_entity::drops::drop_item;
use feather_server_types::{
    BlockUpdateEvent, BumpVec, Game, Inventory, SpawnPacketCreator, WindowCloseEvent,
    WindowOpenEvent,
};
use fecs::{Entity, EntityBuilder, EntityRef, World};
use num_traits::ToPrimitive;

/// Marker component for chests.
pub struct Chest;

/// Stores number of players currently viewing a chest.
/// ("Viewing" seems to mean "has chest open," though the documentation is somewhat vague.)
/// This value is used on the client to render lid animations.
pub struct ChestViewers(u32);

/// Creates a chest.
pub fn create(_game: &Game, _pos: BlockPosition) -> EntityBuilder {
    EntityBuilder::new()
        .with(Chest)
        .with(ChestViewers(0))
        .with(Inventory::chest(false)) // TODO: handle large chests
        .with(SpawnPacketCreator(&create_spawn_packet))
}

/// When a chest is despawned, drops its contents.
#[fecs::event_handler]
pub fn on_chest_break_drop_contents(event: &BlockUpdateEvent, game: &mut Game, world: &mut World) {
    if let Some(entity) = game.block_entities.get(&event.pos).copied() {
        if !world.has::<Chest>(entity) {
            return;
        }

        let items = BumpVec::from_iter_in(
            world
                .get::<Inventory>(entity)
                .iter_mut()
                .filter_map(|mut guard| guard.take()),
            game.bump(),
        );
        for item in items {
            drop_item(game, world, item, event.pos.position());
        }
    }
}

fn create_spawn_packet(accessor: &EntityRef) -> Box<dyn Packet> {
    Box::new(viewers_packet(accessor))
}

#[fecs::event_handler]
pub fn on_chest_open_increment_viewers(event: &WindowOpenEvent, game: &Game, world: &mut World) {
    let should_resend = if let Some(mut viewers) = world.try_get_mut::<ChestViewers>(event.opened) {
        viewers.0 += 1;
        true
    } else {
        false
    };

    if should_resend {
        resend_viewers(game, world, event.opened);
    }
}

#[fecs::event_handler]
pub fn on_chest_close_decrement_viewers(event: &WindowCloseEvent, game: &Game, world: &mut World) {
    let should_resend = if let Some(mut viewers) = world.try_get_mut::<ChestViewers>(event.closed) {
        viewers.0 = viewers.0.checked_sub(1).unwrap_or_default();
        true
    } else {
        false
    };

    if should_resend {
        resend_viewers(game, world, event.closed);
    }
}

fn resend_viewers(game: &Game, world: &World, chest: Entity) {
    let packet = viewers_packet(&world.entity(chest).unwrap());
    game.broadcast_entity_update(world, packet, chest, None);
}

fn viewers_packet(chest: &EntityRef) -> impl Packet {
    BlockAction {
        location: *chest.get::<BlockPosition>(),
        action_id: 1,
        action_param: chest.get::<ChestViewers>().0 as u8,
        block_type: BlockKind::Chest.to_i32().unwrap(),
    }
}
