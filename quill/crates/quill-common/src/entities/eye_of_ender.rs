use bytemuck::{Pod, Zeroable};
/// Marker component for eye of ender entities.
///
/// # Example
/// A system that queries for all eye of enders:
/// ```no_run
/// use quill::{Game, Position, entities::EyeOfEnder};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &EyeOfEnder)>() {
///         println!("Found a eye of ender with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct EyeOfEnder;

pod_component_impl!(EyeOfEnder);
