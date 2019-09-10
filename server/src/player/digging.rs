//! This module handles the monolithic Player Digging packet.
//!
//! The packet's name is rather misleading, as it is also sent
//! for completely unrelated actions, including eating, shooting bows,
//! swapping items out the the offhand, and dropping items.

use specs::{Entity, LazyUpdate, Read, ReadStorage, ReaderId, System, World, Write, WriteStorage};

use feather_core::network::cast_packet;
use feather_core::network::packet::implementation::{
    BlockChange, PlayerDigging, PlayerDiggingStatus,
};
use feather_core::network::packet::PacketType;
use feather_core::world::block::{Block, BlockExt};
use feather_core::world::{BlockPosition, ChunkMap};
use feather_core::{Gamemode, Item, Position};

use crate::disconnect_player;
use crate::entity::{PlayerComponent, PositionComponent, ShootArrowEvent};
use crate::network::PacketQueue;
use crate::player::{InventoryComponent, InventoryUpdateEvent};
use crate::util::Util;
use feather_core::inventory::{ItemStack, SlotIndex, SLOT_HOTBAR_OFFSET, SLOT_OFFHAND};
use shrev::EventChannel;
use specs::SystemData;

/// Event triggered when a block is updated.
///
/// This event is triggered *after* the block is updated
/// in the chunk map.
#[derive(Debug, Clone)]
pub struct BlockUpdateEvent {
    /// The cause of this block update event.
    pub cause: BlockUpdateCause,
    /// The location of the block which was broken.
    pub pos: BlockPosition,
    /// The block which was previously at the position.
    pub old_block: Block,
    /// The new block at the position.
    pub new_block: Block,
}

/// The possible causes of a block update event.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BlockUpdateCause {
    /// Indicates that a player updated the block.
    Player(Entity),
    /// A test block update caused, used for unit testing.
    Test,
}

/// Event triggered when a player drops an item.
///
/// Before this event is triggered, the item
/// is removed from the player's inventory.
#[derive(Debug, Clone)]
pub struct PlayerItemDropEvent {
    /// The slot from which the item was dropped,
    /// if known.
    pub slot: Option<SlotIndex>,
    /// The item stack which was dropped.
    pub stack: ItemStack,
    /// The player who dropped the item.
    pub player: Entity,
}

/// System responsible for polling for PlayerDigging
/// packets and writing the corresponding events.
pub struct PlayerDiggingSystem;

impl<'a> System<'a> for PlayerDiggingSystem {
    type SystemData = (
        WriteStorage<'a, InventoryComponent>,
        ReadStorage<'a, PlayerComponent>, // For gamemodes
        ReadStorage<'a, PositionComponent>,
        Write<'a, EventChannel<BlockUpdateEvent>>,
        Write<'a, EventChannel<PlayerItemDropEvent>>,
        Write<'a, EventChannel<InventoryUpdateEvent>>,
        Write<'a, EventChannel<ShootArrowEvent>>,
        Write<'a, ChunkMap>,
        Read<'a, PacketQueue>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use PlayerDiggingStatus::*;

        let (
            mut inventories,
            players,
            positions,
            mut block_breaks,
            mut item_drops,
            mut inventory_updates,
            mut shoot_arrow_events,
            mut chunk_map,
            packet_queue,
            lazy,
        ) = data;

        let packets = packet_queue.for_packet(PacketType::PlayerDigging);

        for (player, packet) in packets {
            let packet = cast_packet::<PlayerDigging>(&*packet);

            match packet.status {
                StartedDigging | FinishedDigging | CancelledDigging => handle_digging(
                    packet,
                    players.get(player).unwrap(),
                    inventories.get(player).unwrap().item_in_main_hand(),
                    player,
                    &mut block_breaks,
                    &mut chunk_map,
                    &lazy,
                ),
                DropItem | DropItemStack => handle_drop_item_stack(
                    packet,
                    player,
                    &mut inventory_updates,
                    &mut item_drops,
                    inventories.get_mut(player).unwrap(),
                ),
                ConsumeItem => handle_consume_item(
                    packet,
                    players.get(player).unwrap(),
                    player,
                    inventories.get_mut(player).unwrap(),
                    &mut inventory_updates,
                    positions.get(player).unwrap().current,
                    &mut shoot_arrow_events,
                ),
                status => warn!("Unhandled Player Digging status {:?}", status),
            }
        }
    }
}

fn handle_digging(
    packet: &PlayerDigging,
    player: &PlayerComponent,
    item_in_main_hand: Option<&ItemStack>,
    entity: Entity,
    events: &mut EventChannel<BlockUpdateEvent>,
    chunk_map: &mut ChunkMap,
    lazy: &LazyUpdate,
) {
    // Return early if needed
    match packet.status {
        PlayerDiggingStatus::StartedDigging => {
            if player.gamemode != Gamemode::Creative {
                return;
            }
        }
        PlayerDiggingStatus::CancelledDigging => return,
        _ => (),
    }

    // Don't break block if player is holding a sword in creative mode.
    if player.gamemode == Gamemode::Creative {
        if let Some(item_in_main_hand) = item_in_main_hand {
            match item_in_main_hand.ty {
                Item::WoodenSword
                | Item::StoneSword
                | Item::GoldenSword
                | Item::IronSword
                | Item::DiamondSword => return,
                _ => (),
            }
        }
    }

    let old = chunk_map.block_at(packet.location);

    if chunk_map.set_block_at(packet.location, Block::Air).is_err() {
        disconnect_player(
            entity,
            "Attempted to break block in unloaded chunk".to_string(),
            lazy,
        );
        return;
    }

    let event = BlockUpdateEvent {
        cause: BlockUpdateCause::Player(entity),
        pos: packet.location,
        old_block: old.unwrap(), // We checked that the location was valid above
        new_block: Block::Air,
    };

    events.single_write(event);
}

fn handle_drop_item_stack(
    packet: &PlayerDigging,
    entity: Entity,
    inventory_updates: &mut EventChannel<InventoryUpdateEvent>,
    item_drops: &mut EventChannel<PlayerItemDropEvent>,
    inventory: &mut InventoryComponent,
) {
    assert!(
        packet.status == PlayerDiggingStatus::DropItem
            || packet.status == PlayerDiggingStatus::DropItemStack
    );

    let slot = inventory.held_item + SLOT_HOTBAR_OFFSET;

    let stack = {
        if let Some(item) = inventory.item_at(slot) {
            item.clone()
        } else {
            // Silently fail - no item stack to drop
            return;
        }
    };

    let amnt = match packet.status {
        PlayerDiggingStatus::DropItem => {
            if stack.amount == 0 {
                inventory.clear_item_at(slot);
                0
            } else if stack.amount == 1 {
                inventory.clear_item_at(slot);
                1
            } else {
                inventory.set_item_at(slot, ItemStack::new(stack.ty, stack.amount - 1));
                1
            }
        }
        PlayerDiggingStatus::DropItemStack => {
            inventory.clear_item_at(slot);
            stack.amount
        }
        _ => unreachable!(), // Assertion above
    };

    let inv_update = InventoryUpdateEvent {
        slots: smallvec![slot],
        player: entity,
    };
    inventory_updates.single_write(inv_update);

    if amnt != 0 {
        let item_drop = PlayerItemDropEvent {
            slot: Some(slot),
            stack: ItemStack::new(stack.ty, amnt),
            player: entity,
        };
        item_drops.single_write(item_drop);
    }
}

/// Handles food consumption and shooting arrows.
fn handle_consume_item(
    packet: &PlayerDigging,
    player: &PlayerComponent,
    entity: Entity,
    inventory: &mut InventoryComponent,
    inventory_updates: &mut EventChannel<InventoryUpdateEvent>,
    position: Position,
    shoot_arrow_events: &mut EventChannel<ShootArrowEvent>,
) {
    assert_eq!(packet.status, PlayerDiggingStatus::ConsumeItem);

    // TODO: Fallback to off-hand if main-hand is not a consumable
    let used_item = inventory.item_in_main_hand();

    if let Some(item) = used_item {
        if item.ty == Item::Bow {
            handle_shoot_bow(
                player,
                entity,
                inventory,
                inventory_updates,
                position,
                shoot_arrow_events,
            );
        }
        // TODO: Food, potions
    }
}

fn handle_shoot_bow(
    player: &PlayerComponent,
    entity: Entity,
    inventory: &mut InventoryComponent,
    inventory_updates: &mut EventChannel<InventoryUpdateEvent>,
    position: Position,
    shoot_arrow_events: &mut EventChannel<ShootArrowEvent>,
) {
    let arrow_to_consume: Option<(SlotIndex, ItemStack)> = find_arrow(&inventory);
    if player.gamemode == Gamemode::Survival || player.gamemode == Gamemode::Adventure {
        // If no arrow was found, don't shoot
        let arrow_to_consume = arrow_to_consume.clone();
        if arrow_to_consume.is_none() {
            debug!("Tried to shoot bow with no arrows.");
            return;
        }

        // Consume arrow
        let (arrow_slot, arrow_stack) = arrow_to_consume.unwrap();
        let mut arrow_stack: ItemStack = arrow_stack.clone();
        arrow_stack.amount -= 1;

        inventory.set_item_at(arrow_slot, arrow_stack);
        inventory_updates.single_write(InventoryUpdateEvent {
            slots: smallvec![arrow_slot],
            player: entity,
        });
    }

    let arrow_type: Item = match arrow_to_consume {
        None => Item::Arrow, // Default to generic arrow in creative mode with none in inventory
        Some((_, arrow_stack)) => arrow_stack.ty,
    };

    shoot_arrow_events.single_write(ShootArrowEvent {
        shooter: Some(entity),
        position,
        arrow_type,
        critical: false, // TODO: Determine critical based on how long bow was pulled back
    });
}

fn find_arrow(inventory: &InventoryComponent) -> Option<(SlotIndex, ItemStack)> {
    // Order of priority is: off-hand, hotbar (0 to 8), rest of inventory

    if let Some(offhand) = inventory.item_at(SLOT_OFFHAND) {
        if is_arrow_item(offhand.ty) {
            return Some((SLOT_OFFHAND, offhand.clone()));
        }
    }

    for hotbar_slot in 0..9 {
        if let Some(hotbar_stack) = inventory.item_at(SLOT_HOTBAR_OFFSET + hotbar_slot) {
            if is_arrow_item(hotbar_stack.ty) {
                return Some((SLOT_HOTBAR_OFFSET + hotbar_slot, hotbar_stack.clone()));
            }
        }
    }

    for inv_slot in 9..=35 {
        if let Some(inv_stack) = inventory.item_at(inv_slot) {
            if is_arrow_item(inv_stack.ty) {
                return Some((inv_slot, inv_stack.clone()));
            }
        }
    }
    None
}

fn is_arrow_item(item: Item) -> bool {
    match item {
        Item::Arrow | Item::SpectralArrow | Item::TippedArrow => true,
        _ => false,
    }
}

/// System for broadcasting block update
/// events to all clients.
///
/// This system listens to `BlockUpdateEvent`s.
#[derive(Default)]
pub struct BlockUpdateBroadcastSystem {
    reader: Option<ReaderId<BlockUpdateEvent>>,
}

impl<'a> System<'a> for BlockUpdateBroadcastSystem {
    type SystemData = (Read<'a, EventChannel<BlockUpdateEvent>>, Read<'a, Util>);

    fn run(&mut self, data: Self::SystemData) {
        let (events, util) = data;

        // Process events
        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            // Send Block Change packet to every player,
            // except for the one that performed the update
            // (if any)
            let neq = if let BlockUpdateCause::Player(player) = event.cause {
                Some(player)
            } else {
                None
            };

            let packet = BlockChange::new(event.pos, i32::from(event.new_block.native_state_id()));
            util.broadcast_chunk_update(event.pos.chunk_pos(), packet, neq);
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(world.fetch_mut::<EventChannel<_>>().register_reader());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testframework as t;
    use feather_core::item::Item;
    use feather_core::world::chunk::Chunk;
    use feather_core::world::ChunkPosition;
    use specs::WorldExt;

    #[test]
    fn test_started_digging() {
        let (mut w, mut d) = t::init_world();

        let cpos = ChunkPosition::new(0, 0);
        let bpos = BlockPosition::new(0, 0, 0);
        let mut chunk = Chunk::new(cpos);
        chunk.set_block_at(0, 0, 0, Block::Stone);
        w.fetch_mut::<ChunkMap>().set_chunk_at(cpos, chunk);

        let mut event_reader = t::reader(&w);

        // Creative mode

        let player = t::add_player(&mut w);

        let packet = PlayerDigging::new(PlayerDiggingStatus::StartedDigging, bpos, 0);

        t::receive_packet(&player, &w, packet.clone());

        d.dispatch(&w);
        w.maintain();

        t::assert_not_disconnected(&player);

        {
            let mut chunk_map = w.fetch_mut::<ChunkMap>();

            assert_eq!(chunk_map.block_at(bpos).unwrap(), Block::Air);

            chunk_map.set_block_at(bpos, Block::Stone).unwrap();

            let channel = w.fetch_mut::<EventChannel<BlockUpdateEvent>>();
            let events = channel.read(&mut event_reader).collect::<Vec<_>>();
            assert_eq!(events.len(), 1);

            let first = events.first().unwrap();
            assert_eq!(first.old_block, Block::Stone);
            assert_eq!(first.new_block, Block::Air);
            assert_eq!(first.cause, BlockUpdateCause::Player(player.entity));
            assert_eq!(first.pos, bpos);
        }

        // Survival mode
        let player = t::add_player(&mut w);
        w.write_component::<PlayerComponent>()
            .get_mut(player.entity)
            .unwrap()
            .gamemode = Gamemode::Survival;

        t::receive_packet(&player, &w, packet.clone());

        d.dispatch(&w);
        w.maintain();

        t::assert_not_disconnected(&player);

        let chunk_map = w.fetch::<ChunkMap>();
        assert_eq!(chunk_map.block_at(bpos).unwrap(), Block::Stone);

        let channel = w.fetch_mut::<EventChannel<BlockUpdateEvent>>();
        let events = channel.read(&mut event_reader).collect::<Vec<_>>();
        assert_eq!(events.len(), 0);
    }

    // This should be a no-op.
    #[test]
    fn test_cancelled_digging() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        let mut event_reader = t::reader(&w);

        let packet = PlayerDigging::new(
            PlayerDiggingStatus::CancelledDigging,
            BlockPosition::default(),
            0,
        );
        t::receive_packet(&player, &w, packet);

        d.dispatch(&w);
        w.maintain();

        t::assert_not_disconnected(&player);

        let channel = w.fetch::<EventChannel<BlockUpdateEvent>>();
        let events = channel.read(&mut event_reader).collect::<Vec<_>>();
        assert!(events.is_empty());
    }

    #[test]
    fn test_finished_digging() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);
        w.write_component::<PlayerComponent>()
            .get_mut(player.entity)
            .unwrap()
            .gamemode = Gamemode::Survival;

        let mut event_reader = t::reader(&w);

        let bpos = BlockPosition::new(0, 0, 0);
        let cpos = bpos.chunk_pos();

        let mut chunk = Chunk::new(cpos);
        chunk.set_block_at(0, 0, 0, Block::Stone);

        w.fetch_mut::<ChunkMap>().set_chunk_at(cpos, chunk);

        let packet = PlayerDigging::new(PlayerDiggingStatus::FinishedDigging, bpos, 0);
        t::receive_packet(&player, &w, packet);

        d.dispatch(&w);
        w.maintain();

        t::assert_not_disconnected(&player);

        let chunk_map = w.fetch::<ChunkMap>();

        assert_eq!(chunk_map.block_at(bpos).unwrap(), Block::Air);

        let channel = w.fetch_mut::<EventChannel<BlockUpdateEvent>>();
        let events = channel.read(&mut event_reader).collect::<Vec<_>>();
        assert_eq!(events.len(), 1);

        let first = events.first().unwrap();
        assert_eq!(first.old_block, Block::Stone);
        assert_eq!(first.new_block, Block::Air);
        assert_eq!(first.cause, BlockUpdateCause::Player(player.entity));
        assert_eq!(first.pos, bpos);
    }

    #[test]
    fn test_block_break_in_unloaded_chunk() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        let mut event_reader = t::reader(&w);

        let bpos = BlockPosition::new(1000, 25, 1000);

        let packet = PlayerDigging::new(PlayerDiggingStatus::FinishedDigging, bpos, 0);
        t::receive_packet(&player, &w, packet);

        d.dispatch(&w);
        w.maintain();

        t::assert_disconnected(&player);

        let channel = w.fetch::<EventChannel<BlockUpdateEvent>>();
        let events = channel.read(&mut event_reader).collect::<Vec<_>>();
        assert!(events.is_empty());
    }

    #[test]
    fn test_drop_item() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        let slot = SLOT_HOTBAR_OFFSET;
        {
            let mut invs = w.write_component::<InventoryComponent>();
            let inv = invs.get_mut(player.entity).unwrap();
            inv.held_item = 0;
            inv.set_item_at(slot, ItemStack::new(Item::CookedBeef, 4));
        }

        let mut drop_reader = t::reader(&w);
        let mut update_reader = t::reader(&w);

        let packet = PlayerDigging::new(PlayerDiggingStatus::DropItem, BlockPosition::default(), 0);
        t::receive_packet(&player, &w, packet);

        d.dispatch(&w);
        w.maintain();

        let drop_channel = w.fetch::<EventChannel<PlayerItemDropEvent>>();
        let update_channel = w.fetch::<EventChannel<InventoryUpdateEvent>>();

        // Check that events are correct
        let drop_events = drop_channel.read(&mut drop_reader).collect::<Vec<_>>();
        assert_eq!(drop_events.len(), 1);
        let first = drop_events.first().unwrap();
        assert_eq!(first.player, player.entity);
        assert_eq!(first.slot, Some(slot));
        assert_eq!(first.stack, ItemStack::new(Item::CookedBeef, 1)); // 1 beef was dropped

        let update_events = update_channel.read(&mut update_reader).collect::<Vec<_>>();
        assert_eq!(update_events.len(), 1);
        let first = update_events.first().unwrap();
        assert_eq!(first.player, player.entity);
        assert_eq!(first.slots.as_slice(), &[slot]);

        // Check that inventory was updated correctly
        let invs = w.read_component::<InventoryComponent>();
        let inv = invs.get(player.entity).unwrap();
        assert_eq!(
            inv.item_at(slot).unwrap(),
            &ItemStack::new(Item::CookedBeef, 3)
        ); // 1 was removed
    }

    #[test]
    fn test_drop_item_no_stack() {
        // This should be a no-op.
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        let mut drop_reader = t::reader(&w);
        let mut update_reader = t::reader(&w);

        let packet = PlayerDigging::new(PlayerDiggingStatus::DropItem, BlockPosition::default(), 0);
        t::receive_packet(&player, &w, packet);

        d.dispatch(&w);
        w.maintain();

        t::assert_not_disconnected(&player);

        let drop_channel = w.fetch::<EventChannel<PlayerItemDropEvent>>();
        let update_channel = w.fetch::<EventChannel<InventoryUpdateEvent>>();

        let drop_events = drop_channel.read(&mut drop_reader).collect::<Vec<_>>();
        assert!(drop_events.is_empty());
        let update_events = update_channel.read(&mut update_reader).collect::<Vec<_>>();
        assert!(update_events.is_empty());
    }

    #[test]
    fn test_drop_item_stack() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        let mut drop_reader = t::reader(&w);
        let mut update_reader = t::reader(&w);

        let slot = SLOT_HOTBAR_OFFSET;
        let amnt = 32;
        {
            let mut invs = w.write_component::<InventoryComponent>();
            let inv = invs.get_mut(player.entity).unwrap();
            inv.set_item_at(slot, ItemStack::new(Item::CookedBeef, amnt));
        }

        let packet = PlayerDigging::new(
            PlayerDiggingStatus::DropItemStack,
            BlockPosition::default(),
            0,
        );
        t::receive_packet(&player, &w, packet);

        d.dispatch(&w);
        w.maintain();

        let drop_channel = w.fetch::<EventChannel<PlayerItemDropEvent>>();
        let update_channel = w.fetch::<EventChannel<InventoryUpdateEvent>>();

        let update_events = update_channel.read(&mut update_reader).collect::<Vec<_>>();
        assert_eq!(update_events.len(), 1);
        let first = update_events.first().unwrap();
        assert_eq!(first.player, player.entity);
        assert_eq!(first.slots.as_slice(), &[slot]);

        let drop_events = drop_channel.read(&mut drop_reader).collect::<Vec<_>>();
        assert_eq!(drop_events.len(), 1);
        let first = drop_events.first().unwrap();
        assert_eq!(first.player, player.entity);
        assert_eq!(first.slot, Some(slot));
        assert_eq!(first.stack, ItemStack::new(Item::CookedBeef, amnt));

        let invs = w.read_component::<InventoryComponent>();
        let inv = invs.get(player.entity).unwrap();
        assert_eq!(inv.item_at(slot), None);
    }

    #[test]
    fn test_block_update_broadcast_system() {
        let (mut w, mut d) = t::init_world();

        let player1 = t::add_player(&mut w);
        let player2 = t::add_player(&mut w);

        let pos = BlockPosition::default();
        let block = Block::Sand;

        let event = BlockUpdateEvent {
            cause: BlockUpdateCause::Player(player1.entity),
            pos,
            old_block: block,
            new_block: Block::Air,
        };
        w.fetch_mut::<EventChannel<_>>().single_write(event);

        d.dispatch(&w);
        w.maintain();

        let block_change = t::assert_packet_received(&player2, PacketType::BlockChange);
        let block_change = cast_packet::<BlockChange>(&*block_change);
        assert_eq!(block_change.location, pos);
        assert_eq!(
            block_change.block_id,
            i32::from(Block::Air.native_state_id())
        );

        t::assert_packet_not_received(&player1, PacketType::BlockChange); // Don't send update to own player

        // Now handle an event not caused by a player
        let event = BlockUpdateEvent {
            cause: BlockUpdateCause::Test,
            pos,
            old_block: block,
            new_block: Block::Air,
        };
        w.fetch_mut::<EventChannel<_>>().single_write(event);

        d.dispatch(&w);
        w.maintain();

        // Packet should be sent to both players
        t::assert_packet_received(&player1, PacketType::BlockChange);
        t::assert_packet_received(&player2, PacketType::BlockChange);
    }

    #[test]
    pub fn test_find_arrow() {
        let mut inv = InventoryComponent::new();
        inv.set_item_at(
            SLOT_OFFHAND,
            ItemStack {
                ty: Item::Arrow,
                amount: 1,
            },
        );
        inv.set_item_at(
            SLOT_HOTBAR_OFFSET,
            ItemStack {
                ty: Item::Arrow,
                amount: 1,
            },
        );
        inv.set_item_at(
            9,
            ItemStack {
                ty: Item::Arrow,
                amount: 1,
            },
        );

        // 1. Off-hand
        let (slot, stack) = find_arrow(&inv).unwrap();
        assert_eq!(slot, SLOT_OFFHAND);
        assert_eq!(stack.ty, Item::Arrow);
        inv.clear_item_at(SLOT_OFFHAND);

        // 2. Hot-bar
        let (slot, stack) = find_arrow(&inv).unwrap();
        assert_eq!(slot, SLOT_HOTBAR_OFFSET);
        assert_eq!(stack.ty, Item::Arrow);
        inv.clear_item_at(SLOT_HOTBAR_OFFSET);

        // 3. Rest of inventory
        let (slot, stack) = find_arrow(&inv).unwrap();
        assert_eq!(slot, 9);
        assert_eq!(stack.ty, Item::Arrow);
        inv.clear_item_at(9);

        // 4. No arrow found
        assert!(find_arrow(&inv).is_none());
    }

    #[test]
    pub fn test_shoot_arrow() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        let mut shoot_reader = t::reader(&w);
        let mut update_reader = t::reader(&w);

        let slot = SLOT_HOTBAR_OFFSET;
        let amnt = 32;
        {
            let mut invs = w.write_component::<InventoryComponent>();
            let inv = invs.get_mut(player.entity).unwrap();
            inv.set_item_at(slot, ItemStack::new(Item::Bow, 1));
            inv.set_item_at(slot + 1, ItemStack::new(Item::Arrow, amnt));
        }

        // Change to survival
        w.write_component::<PlayerComponent>()
            .insert(
                player.entity,
                PlayerComponent {
                    gamemode: Gamemode::Survival,
                    profile_properties: vec![],
                },
            )
            .unwrap();

        let packet = PlayerDigging::new(
            PlayerDiggingStatus::ConsumeItem,
            BlockPosition::default(),
            0,
        );
        t::receive_packet(&player, &w, packet);

        d.dispatch(&w);
        w.maintain();

        let shoot_channel = w.fetch::<EventChannel<ShootArrowEvent>>();
        let update_channel = w.fetch::<EventChannel<InventoryUpdateEvent>>();

        let update_events = update_channel.read(&mut update_reader).collect::<Vec<_>>();
        assert_eq!(update_events.len(), 1);
        let first = update_events.first().unwrap();
        assert_eq!(first.player, player.entity);
        assert_eq!(first.slots.as_slice(), &[slot + 1]);

        let shoot_events = shoot_channel.read(&mut shoot_reader).collect::<Vec<_>>();
        assert_eq!(shoot_events.len(), 1);
        let first = shoot_events.first().unwrap();
        assert_eq!(first.shooter.unwrap(), player.entity);
        assert_eq!(first.arrow_type, Item::Arrow);

        // In survival, check if amount of arrow stack decreased.
        let invs = w.read_component::<InventoryComponent>();
        let inv = invs.get(player.entity).unwrap();
        assert_eq!(inv.item_at(slot + 1).unwrap().amount, amnt - 1);
    }
}
