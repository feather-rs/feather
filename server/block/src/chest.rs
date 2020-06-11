use anyhow::bail;
use feather_core::util::BlockPosition;
use feather_core::{
    anvil::{
        block_entity::{BlockEntityData, BlockEntityKind, BlockEntityVariant},
        player::InventorySlot,
    },
    blocks::{BlockKind, ChestKind, FacingCardinal},
    inventory::{Area, Window},
    items::{Item, ItemStack},
    network::{
        packets::{BlockAction, OpenWindow, WindowItems},
        Packet,
    },
    text::TextRoot,
};
use feather_server_entity::drops::drop_item;
use feather_server_types::{
    BlockEntityLoaderRegistration, BlockSerializer, BlockUpdateCause, BlockUpdateEvent, BumpVec,
    Game, InteractionHandler, Inventory, Network, SpawnPacketCreator, WindowCloseEvent,
    WindowOpenEvent,
};
use fecs::{Entity, EntityBuilder, EntityRef, World};
use num_traits::ToPrimitive;

pub const SLOTS: usize = 27;

inventory::submit!(BlockEntityLoaderRegistration {
    f: &load,
    kind: BlockEntityVariant::Chest,
});

/// Marker component for chests.
pub struct Chest;

/// Stores number of players currently viewing a chest.
/// ("Viewing" seems to mean "has chest open," though the documentation is somewhat vague.)
/// This value is used on the client to render lid animations.
pub struct ChestViewers(u32);

/// Creates a chest.
pub fn create(pos: BlockPosition) -> EntityBuilder {
    create_with_inventory(pos, Inventory::chest())
}

/// Creates a chest with the given inventory.
pub fn create_with_inventory(pos: BlockPosition, inventory: Inventory) -> EntityBuilder {
    crate::base(pos)
        .with(Chest)
        .with(ChestViewers(0))
        .with(inventory)
        .with(SpawnPacketCreator(&create_spawn_packet))
        .with(BlockSerializer(&serialize))
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

#[fecs::event_handler]
pub fn on_chest_create_try_connect(event: &BlockUpdateEvent, game: &mut Game, world: &mut World) {
    if event.new.kind() != BlockKind::Chest {
        return;
    }

    if event.new.chest_kind() != Some(ChestKind::Single) {
        return;
    }

    try_connect_chests(game, world, event.pos);
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

fn serialize(_game: &Game, accessor: &EntityRef) -> BlockEntityData {
    let base = crate::serialize_base(accessor);

    let items = serialize_items(&*accessor.get::<Inventory>());

    BlockEntityData {
        base,
        kind: BlockEntityKind::Chest {
            items,
            loot_table: None,
            loot_table_seed: None,
        },
    }
}

fn serialize_items(inventory: &Inventory) -> Vec<InventorySlot> {
    let mut slots = Vec::new();
    for i in 0..27 {
        let item = inventory.item_at(Area::Chest, i).unwrap();

        if let Some(item) = item {
            slots.push(InventorySlot {
                item: item.ty.identifier().to_owned(),
                count: item.amount as i8,
                slot: i as i8,
            });
        }
    }
    slots
}

fn load(data: BlockEntityData) -> anyhow::Result<EntityBuilder> {
    let pos = crate::load_base(&data.base);
    let slots = match data.kind {
        BlockEntityKind::Chest { items, .. } => items,
        _ => bail!("not a chest"),
    };

    let inventory = load_inventory(&slots);

    Ok(create_with_inventory(pos, inventory))
}

fn load_inventory(slots: &[InventorySlot]) -> Inventory {
    let inv = Inventory::chest();

    for slot in slots {
        if let Some(item) = Item::from_identifier(&slot.item) {
            if let Err(e) = inv.set_item_at(
                Area::Chest,
                slot.slot as usize,
                ItemStack::new(item, slot.count as u8),
            ) {
                log::warn!("Invalid chest slot: {}", e);
            }
        }
    }

    inv
}

/// If the block at the given position is a chest, and it is connected
/// to another chest to form a large chest, returns the position of the connected
/// chest.
pub fn connected_chest(game: &Game, pos: BlockPosition) -> Option<BlockPosition> {
    let block = game.block_at(pos).unwrap_or_default();

    if block.kind() != BlockKind::Chest {
        return None;
    }

    let kind = block
        .chest_kind()
        .expect("chest block always has chest_kind property");
    let facing = block
        .facing_cardinal()
        .expect("chest blcok always has facing_cardinal property");

    // facing_offset is offset along (x, z) axes
    // from the left chest to the right.
    let facing_offset = connected_offset(facing);

    match kind {
        ChestKind::Single => None,
        ChestKind::Left => Some(BlockPosition::new(
            pos.x - facing_offset[0],
            pos.y,
            pos.z - facing_offset[1],
        )),
        ChestKind::Right => Some(BlockPosition::new(
            pos.x + facing_offset[0],
            pos.y,
            pos.z + facing_offset[1],
        )),
    }
}

/// Attempts to connect the chest at `pos` to an adjacent chest
/// facing the same direction.
/// Returns the position of the adjacent chest, or `None` if
/// no chest was found.
pub fn try_connect_chests(
    game: &mut Game,
    world: &mut World,
    pos: BlockPosition,
) -> Option<BlockPosition> {
    let block = game.block_at(pos).unwrap_or_default();
    if block.kind() != BlockKind::Chest {
        return None;
    }
    let facing = block.facing_cardinal().expect("chest has facing_cardinal");

    let offsets = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for (x_offset, z_offset) in offsets.iter().copied() {
        let pos2 = BlockPosition::new(pos.x + x_offset, pos.y, pos.z + z_offset);

        let block2 = game.block_at(pos2).unwrap_or_default();
        if block2.kind() != BlockKind::Chest {
            continue;
        }

        let kind2 = block2.chest_kind().expect("chest has chest_kind");
        if kind2 != ChestKind::Single {
            // Other chest is already connected.
            continue;
        }

        let facing2 = block2.facing_cardinal().expect("chest has facing_cardinal");
        if facing == facing2 {
            // Both chests face same direction. Connect them.
            let (mut left_pos, mut left_block, mut right_pos, mut right_block) =
                if x_offset < 0 || z_offset < 0 {
                    (pos2, block2, pos, block)
                } else {
                    (pos, block, pos2, block2)
                };

            if !matches!(facing, FacingCardinal::South | FacingCardinal::West) {
                // backwards, so swap
                std::mem::swap(&mut left_pos, &mut right_pos);
                std::mem::swap(&mut left_block, &mut right_block);
            }

            left_block = left_block.with_chest_kind(ChestKind::Right);
            right_block = right_block.with_chest_kind(ChestKind::Left);

            game.set_block_at(world, left_pos, left_block, BlockUpdateCause::Unknown);
            game.set_block_at(world, right_pos, right_block, BlockUpdateCause::Unknown);

            return Some(pos2);
        }
    }
    None
}

/// Giving a chest's facing direction, returns the offset
/// along (x, z) axes to a potential connected chest to the right.
fn connected_offset(facing: FacingCardinal) -> [i32; 2] {
    match facing {
        FacingCardinal::North => [-1, 0],
        FacingCardinal::South => [1, 0],
        FacingCardinal::East => [0, -1],
        FacingCardinal::West => [0, 1],
    }
}

/// Handler for player right clicking on chests.
pub struct ChestInteraction;
inventory::submit!(Box::new(ChestInteraction) as Box<dyn InteractionHandler>);

impl InteractionHandler for ChestInteraction {
    fn handle_interaction(
        &self,
        game: &mut Game,
        world: &mut World,
        pos: BlockPosition,
        player: Entity,
        window_id: u8,
    ) {
        // Open chest window and set the player's window
        if let Some(chest_entity) = game.block_entities.get(&pos).copied() {
            let packet = OpenWindow {
                window_id,
                window_type: String::from("minecraft:chest"),
                window_title: TextRoot::from("Chest").into(),
                number_of_slots: 27,
                entity_id: None,
            };
            world.get::<Network>(player).send(packet);

            {
                let mut slots = Vec::new();
                let chest = world.get::<Inventory>(chest_entity);
                for i in 0..27 {
                    slots.push(chest.item_at(Area::Chest, i).unwrap());
                }

                let packet = WindowItems { window_id, slots };
                world.get::<Network>(player).send(packet);
            }

            world
                .add(player, Window::chest(player, chest_entity))
                .unwrap();

            game.handle(
                world,
                WindowOpenEvent {
                    player,
                    opened: chest_entity,
                },
            );
        }
    }

    fn block_kind(&self) -> BlockKind {
        BlockKind::Chest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use feather_core::blocks::BlockId;
    use feather_server_types::BlockUpdateCause;
    use feather_test_framework::Test;

    #[test]
    fn test_connected_chest() {
        let mut test = Test::new();

        let pairs = vec![
            (
                BlockPosition::new(0, 0, 0),
                BlockPosition::new(1, 0, 0),
                BlockId::chest()
                    .with_chest_kind(ChestKind::Right)
                    .with_facing_cardinal(FacingCardinal::South),
                BlockId::chest()
                    .with_chest_kind(ChestKind::Left)
                    .with_facing_cardinal(FacingCardinal::South),
            ),
            (
                BlockPosition::new(0, 0, 0),
                BlockPosition::new(0, 0, -1),
                BlockId::chest()
                    .with_chest_kind(ChestKind::Right)
                    .with_facing_cardinal(FacingCardinal::East),
                BlockId::chest()
                    .with_chest_kind(ChestKind::Left)
                    .with_facing_cardinal(FacingCardinal::East),
            ),
        ];

        for (pos_left, pos_right, block_left, block_right) in pairs {
            assert!(test.game.set_block_at(
                &mut test.world,
                pos_left,
                block_left,
                BlockUpdateCause::Unknown,
            ));
            assert!(test.game.set_block_at(
                &mut test.world,
                pos_right,
                block_right,
                BlockUpdateCause::Unknown,
            ));

            assert_eq!(connected_chest(&test.game, pos_left), Some(pos_right));
        }
    }

    #[test]
    fn test_connected_chest_single() {
        let mut test = Test::new();
        test.game.set_block_at(
            &mut test.world,
            BlockPosition::new(0, 0, 0),
            BlockId::chest().with_chest_kind(ChestKind::Single),
            BlockUpdateCause::Unknown,
        );

        assert_eq!(
            connected_chest(&test.game, BlockPosition::new(0, 0, 0)),
            None
        );
    }

    #[test]
    fn test_try_connect_chests() {
        let mut test = Test::new();
        test.game.set_block_at(
            &mut test.world,
            BlockPosition::new(0, 0, 0),
            BlockId::chest()
                .with_chest_kind(ChestKind::Single)
                .with_facing_cardinal(FacingCardinal::East),
            BlockUpdateCause::Unknown,
        );
        assert_eq!(
            try_connect_chests(&mut test.game, &mut test.world, BlockPosition::new(0, 0, 0)),
            None
        );

        test.game.set_block_at(
            &mut test.world,
            BlockPosition::new(0, 0, 1),
            BlockId::chest()
                .with_chest_kind(ChestKind::Single)
                .with_facing_cardinal(FacingCardinal::East),
            BlockUpdateCause::Unknown,
        );
        assert_eq!(
            try_connect_chests(&mut test.game, &mut test.world, BlockPosition::new(0, 0, 0)),
            Some(BlockPosition::new(0, 0, 1))
        );

        let left = test.game.block_at(BlockPosition::new(0, 0, 0)).unwrap();
        let right = test.game.block_at(BlockPosition::new(0, 0, 1)).unwrap();

        assert_eq!(left.chest_kind(), Some(ChestKind::Left));
        assert_eq!(right.chest_kind(), Some(ChestKind::Right));
        assert_eq!(left.facing_cardinal(), Some(FacingCardinal::East));
        assert_eq!(right.facing_cardinal(), Some(FacingCardinal::East));
    }
}
