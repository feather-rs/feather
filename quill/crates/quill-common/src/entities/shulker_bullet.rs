use bytemuck::{Pod, Zeroable};
/// Marker component for shulker bullet entities.
///
/// # Example
/// A system that queries for all shulker bullets:
/// ```no_run
/// use quill::{Game, Position, entities::ShulkerBullet};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ShulkerBullet)>() {
///         println!("Found a shulker bullet with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ShulkerBullet;

pod_component_impl!(ShulkerBullet);
