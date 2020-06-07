use crate::{item, InventoryExt};
use feather_core::items::ItemStack;
use feather_core::loot::{loot_table, Conditions};
use feather_core::util::Position;
use feather_server_types::{
    BlockUpdateEvent, CanInstaBreak, EntitySpawnEvent, Game, Inventory, Velocity, TPS,
};
use fecs::{Entity, World};
use rand::Rng;

/// When a block is broken with valid conditions,
/// yields items from the block's loot table.
#[fecs::event_handler]
pub fn on_block_break_drop_loot(event: &BlockUpdateEvent, game: &mut Game, world: &mut World) {
    if event.old.is_air() || !event.new.is_air() {
        return;
    }

    let item = match event.cause {
        feather_server_types::BlockUpdateCause::Entity(entity) => {
            // If broken by a player who can insta-break, don't drop loot.
            if world.has::<CanInstaBreak>(entity) {
                return;
            }

            let item = world
                .try_get::<Inventory>(entity)
                .map(|inv| inv.item_in_main_hand(entity, world))
                .flatten();

            // If the block was not broken with the correct tool, don't drop loot.
            if event.old.kind().best_tool_required() {
                let tool_used = item.map(|item| item.ty.tool()).flatten();

                let best_tool = event.old.kind().best_tool();

                if tool_used != best_tool {
                    return;
                }
            }

            item
        }
        feather_server_types::BlockUpdateCause::Unsupported => None,
        _ => return,
    };

    if let Some(loot_table) = loot_table(&format!("blocks/{}", &event.old.identifier()[10..])) {
        let conditions = Conditions { item };
        let items = loot_table
            .sample(&mut *game.rng(), &conditions)
            .unwrap_or_else(|e| {
                log::error!(
                    "Error sampling from loot table `{}`: {:?}",
                    event.old.identifier(),
                    e
                );
                Default::default()
            });

        for item in items {
            drop_item(game, world, item, event.pos.position());
        }
    }
}

/// "Naturally" drops an item caused by e.g. a broken block or a dead entity.
pub fn drop_item(game: &mut Game, world: &mut World, item: ItemStack, pos: Position) -> Entity {
    // Compute velocity. Based on Glowstone's implementation of `World#dropItemNaturally()`.
    let mut rng = game.rng();

    let radius = 0.05f64;
    let offset_x = rng.gen_range(0.0f64, radius * 2.0) - radius;
    let offset_y = 0.15;
    let mut offset_z = (radius.powi(2) - offset_x.powi(2)).sqrt();

    if rng.gen() {
        offset_z *= -1.0;
    }

    let entity = item::create(item, game.tick_count + TPS)
        .with(pos)
        .with(Velocity(glm::vec3(offset_x, offset_y, offset_z)))
        .build()
        .spawn_in(world);
    drop(rng);
    game.handle(world, EntitySpawnEvent { entity });

    entity
}
