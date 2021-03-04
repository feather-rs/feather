use bytemuck::{Pod, Zeroable};
/// Marker component for lightning bolt entities.
///
/// # Example
/// A system that queries for all lightning bolts:
/// ```no_run
/// use quill::{Game, Position, entities::LightningBolt};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &LightningBolt)>() {
///         println!("Found a lightning bolt with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct LightningBolt;

pod_component_impl!(LightningBolt);
