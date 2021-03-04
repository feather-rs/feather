use bytemuck::{Pod, Zeroable};
/// Marker component for small fireball entities.
///
/// # Example
/// A system that queries for all small fireballs:
/// ```no_run
/// use quill::{Game, Position, entities::SmallFireball};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &SmallFireball)>() {
///         println!("Found a small fireball with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct SmallFireball;

pod_component_impl!(SmallFireball);
