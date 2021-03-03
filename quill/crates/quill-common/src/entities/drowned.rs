use bytemuck::{Pod, Zeroable};
/// Marker component for drowned entities.
///
/// # Example
/// A system that queries for all drowneds:
/// ```no_run
/// use quill::{Game, Position, entities::Drowned};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Drowned)>() {
///         println!("Found a drowned with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Drowned;

pod_component_impl!(Drowned);
