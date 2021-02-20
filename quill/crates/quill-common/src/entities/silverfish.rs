use bytemuck::{Pod, Zeroable};
/// Marker component for silverfish entities.
///
/// # Example
/// A system that queries for all silverfishs:
/// ```no_run
/// use quill::{Game, Position, entities::Silverfish};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Silverfish)>() {
///         println!("Found a silverfish with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Silverfish;

pod_component_impl!(Silverfish);
