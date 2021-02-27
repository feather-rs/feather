//! Handles fall damage for entities

use feather_core::util::Position;
use feather_server_types::{
    BlocksFallen, BumpVec, CanTakeDamage, Dead, Game, Health, PreviousPosition,
};
use fecs::{component, Entity, IntoQuery, Read, World, Write};
use std::cell::RefCell;

/// System which updates `BlocksFallen` for all entities.
#[fecs::system]
pub fn update_blocks_fallen(game: &mut Game, world: &mut World) {
    // Entities who went from !on_ground => on_ground

    // (Legion for_each closures are not FnMuts; no idea why.)
    let landed = RefCell::new(BumpVec::<Entity>::new_in(game.bump()));

    // TODO: use parallel iterator (blocked on allocator API)
    // (BumpVec isn't Send.)
    <(Read<Position>, Read<PreviousPosition>, Write<BlocksFallen>)>::query()
        .filter(component::<Health>())
        .filter(component::<CanTakeDamage>())
        .filter(!component::<Dead>())
        .for_each_entities_mut(
            world.inner_mut(),
            |(entity, (pos, prev_pos, mut blocks_fallen))| {
                match (prev_pos.0.map(|pos| pos.on_ground), pos.on_ground) {
                    (Some(false), false) => {
                        // In air: update blocks_fallen
                        blocks_fallen.0 += (prev_pos.0.unwrap().y - pos.y).max(0.0);
                    }
                    (Some(true), false) => {
                        // reset blocks_fallen
                        blocks_fallen.0 = 0.0;
                    }
                    (Some(false), true) => {
                        // landed
                        landed.borrow_mut().push(entity);
                    }
                    _ => (),
                }
            },
        );

    // Damage landed entities
    for entity in landed.into_inner() {
        let blocks_fallen = world.get::<BlocksFallen>(entity).0;

        // https://minecraft.gamepedia.com/Damage#Fall_damage
        let damage = (blocks_fallen - 3.0).max(0.0).round() as u32;

        if damage != 0 {
            game.damage(entity, damage, world);
        }
    }
}
