use bytemuck::{Pod, Zeroable};
/// Marker component for evoker fangs entities.
///
/// # Example
/// A system that queries for all evoker fangss:
/// ```no_run
/// use quill::{Game, Position, entities::EvokerFangs};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &EvokerFangs)>() {
///         println!("Found a evoker fangs with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct EvokerFangs;

pod_component_impl!(EvokerFangs);
