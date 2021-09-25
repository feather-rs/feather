use bytemuck::{Pod, Zeroable};
/// Marker component for evoker entities.
///
/// # Example
/// A system that queries for all evokers:
/// ```no_run
/// use quill::{Game, Position, entities::Evoker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Evoker)>() {
///         println!("Found a evoker with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Evoker;

pod_component_impl!(Evoker);
