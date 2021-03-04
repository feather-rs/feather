use bytemuck::{Pod, Zeroable};
/// Marker component for sheep entities.
///
/// # Example
/// A system that queries for all sheeps:
/// ```no_run
/// use quill::{Game, Position, entities::Sheep};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Sheep)>() {
///         println!("Found a sheep with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Sheep;

pod_component_impl!(Sheep);
