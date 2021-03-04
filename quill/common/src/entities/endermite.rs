use bytemuck::{Pod, Zeroable};
/// Marker component for endermite entities.
///
/// # Example
/// A system that queries for all endermites:
/// ```no_run
/// use quill::{Game, Position, entities::Endermite};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Endermite)>() {
///         println!("Found a endermite with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Endermite;

pod_component_impl!(Endermite);
