use bytemuck::{Pod, Zeroable};
/// Marker component for pufferfish entities.
///
/// # Example
/// A system that queries for all pufferfishs:
/// ```no_run
/// use quill::{Game, Position, entities::Pufferfish};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Pufferfish)>() {
///         println!("Found a pufferfish with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Pufferfish;

pod_component_impl!(Pufferfish);
