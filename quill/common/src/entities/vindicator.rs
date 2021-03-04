use bytemuck::{Pod, Zeroable};
/// Marker component for vindicator entities.
///
/// # Example
/// A system that queries for all vindicators:
/// ```no_run
/// use quill::{Game, Position, entities::Vindicator};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Vindicator)>() {
///         println!("Found a vindicator with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Vindicator;

pod_component_impl!(Vindicator);
