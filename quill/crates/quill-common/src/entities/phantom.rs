use bytemuck::{Pod, Zeroable};
/// Marker component for phantom entities.
///
/// # Example
/// A system that queries for all phantoms:
/// ```no_run
/// use quill::{Game, Position, entities::Phantom};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Phantom)>() {
///         println!("Found a phantom with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Phantom;

pod_component_impl!(Phantom);
