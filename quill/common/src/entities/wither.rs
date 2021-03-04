use bytemuck::{Pod, Zeroable};
/// Marker component for wither entities.
///
/// # Example
/// A system that queries for all withers:
/// ```no_run
/// use quill::{Game, Position, entities::Wither};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Wither)>() {
///         println!("Found a wither with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Wither;

pod_component_impl!(Wither);
