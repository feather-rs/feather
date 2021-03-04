use bytemuck::{Pod, Zeroable};
/// Marker component for snow golem entities.
///
/// # Example
/// A system that queries for all snow golems:
/// ```no_run
/// use quill::{Game, Position, entities::SnowGolem};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &SnowGolem)>() {
///         println!("Found a snow golem with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct SnowGolem;

pod_component_impl!(SnowGolem);
