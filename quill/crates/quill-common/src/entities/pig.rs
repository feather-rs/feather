use bytemuck::{Pod, Zeroable};
/// Marker component for pig entities.
///
/// # Example
/// A system that queries for all pigs:
/// ```no_run
/// use quill::{Game, Position, entities::Pig};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Pig)>() {
///         println!("Found a pig with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Pig;

pod_component_impl!(Pig);
