use bytemuck::{Pod, Zeroable};
/// Marker component for ghast entities.
///
/// # Example
/// A system that queries for all ghasts:
/// ```no_run
/// use quill::{Game, Position, entities::Ghast};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Ghast)>() {
///         println!("Found a ghast with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Ghast;

pod_component_impl!(Ghast);
