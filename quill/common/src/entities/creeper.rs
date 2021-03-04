use bytemuck::{Pod, Zeroable};
/// Marker component for creeper entities.
///
/// # Example
/// A system that queries for all creepers:
/// ```no_run
/// use quill::{Game, Position, entities::Creeper};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Creeper)>() {
///         println!("Found a creeper with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Creeper;

pod_component_impl!(Creeper);
