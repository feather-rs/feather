use bytemuck::{Pod, Zeroable};
/// Marker component for bee entities.
///
/// # Example
/// A system that queries for all bees:
/// ```no_run
/// use quill::{Game, Position, entities::Bee};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Bee)>() {
///         println!("Found a bee with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Bee;

pod_component_impl!(Bee);
