use bytemuck::{Pod, Zeroable};
/// Marker component for strider entities.
///
/// # Example
/// A system that queries for all striders:
/// ```no_run
/// use quill::{Game, Position, entities::Strider};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Strider)>() {
///         println!("Found a strider with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Strider;

pod_component_impl!(Strider);
