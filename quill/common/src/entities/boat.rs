use bytemuck::{Pod, Zeroable};
/// Marker component for boat entities.
///
/// # Example
/// A system that queries for all boats:
/// ```no_run
/// use quill::{Game, Position, entities::Boat};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Boat)>() {
///         println!("Found a boat with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Boat;

pod_component_impl!(Boat);
