use bytemuck::{Pod, Zeroable};
/// Marker component for dragon fireball entities.
///
/// # Example
/// A system that queries for all dragon fireballs:
/// ```no_run
/// use quill::{Game, Position, entities::DragonFireball};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &DragonFireball)>() {
///         println!("Found a dragon fireball with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct DragonFireball;

pod_component_impl!(DragonFireball);
