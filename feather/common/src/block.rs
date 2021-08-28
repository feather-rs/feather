use base::{Area, BlockId, Gamemode, Inventory, Item, ItemStack};
use ecs::{SysResult, SystemExecutor};
use quill_common::events::BlockPlacementEvent;
use crate::chunk::entities::ChunkEntities;
use crate::entities::player::HotbarSlot;
use crate::events::BlockChangeEvent;
use crate::{Game, World};

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.add_system(block_placement);
}

pub fn block_placement(game: &mut Game) -> SysResult {
    let mut events = vec![];
    for (player, event) in game.ecs.query::<&BlockPlacementEvent>().iter() {
        let inv = game.ecs.get_mut::<Inventory>(player)?;
        let gamemode = game.ecs.get::<Gamemode>(player)?;
        let hotbar = game.ecs.get::<HotbarSlot>(player)?;
        let mut slot = match event.hand {
            libcraft_core::Hand::Main => inv.item(Area::Hotbar, hotbar.get()),
            libcraft_core::Hand::Offhand => inv.item(Area::Offhand, 0),
        }.unwrap();
        if slot.is_none() { continue; }
        let block = match item_to_block(slot.as_ref().unwrap().item()) {
            Some(s) => s,
            None => continue,
        };
        match place_block(&mut game.world, &game.chunk_entities, block, event) {
            Some(s) =>  {
                match *gamemode {
                    Gamemode::Survival | Gamemode::Adventure => decrease_slot(&mut slot),
                    Gamemode::Creative | Gamemode::Spectator => {},
                }
                events.push(s)
            },
            None => {},
        }
    }
    for e in events {
        game.ecs.insert_event(e);
    }
    
    Ok(())
}

fn place_block(world: &mut World, chunk_entities: &ChunkEntities, block: BlockId, placement: &BlockPlacementEvent) -> Option<BlockChangeEvent> {
    let placement_pos = placement.location.adjacent(placement.face);
    if !world.block_at(placement_pos).map(|e| e.is_replaceable()).unwrap_or(false) { // Not loaded or solid
        return None;
    }
    if chunk_entities.entities_in_chunk(placement_pos.chunk()).iter().any(|_entity| { false }) { // FIXME: Somehow check if block would collide with any entities
        return None;
    }
    if world.set_block_at(placement_pos, block) {
        Some(BlockChangeEvent::single(placement_pos))
    } else {
        None
    }
}

fn item_to_block(item: Item) -> Option<BlockId> {
    let mut name = "minecraft:".to_owned();
    name.push_str(item.name());
    BlockId::from_identifier(&name)
}

fn decrease_slot(slot: &mut Option<ItemStack>) {
    match slot  {
        Some(s) => {
            s.remove(1);
            if s.count() == 0 {
                *slot = None;
            }
        },
        None => {},
    }
}