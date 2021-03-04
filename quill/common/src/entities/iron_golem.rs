use bytemuck::{Pod, Zeroable};
/// Marker component for iron golem entities.
///
/// # Example
/// A system that queries for all iron golems:
/// ```no_run
/// use quill::{Game, Position, entities::IronGolem};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &IronGolem)>() {
///         println!("Found a iron golem with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct IronGolem;

pod_component_impl!(IronGolem);
