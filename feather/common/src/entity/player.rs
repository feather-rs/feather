use base::EntityKind;
use ecs::EntityBuilder;

/// Marker component. Indicates that an entity is a player.
pub struct Player;

/// Fills an `EntityBuilder` with components for a player.
pub fn build(builder: &mut EntityBuilder) -> &mut EntityBuilder {
    builder.add(Player).add(EntityKind::Player)
}
