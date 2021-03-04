use bytemuck::{Pod, Zeroable};
/// Marker component for witch entities.
///
/// # Example
/// A system that queries for all witchs:
/// ```no_run
/// use quill::{Game, Position, entities::Witch};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Witch)>() {
///         println!("Found a witch with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Witch;

pod_component_impl!(Witch);
