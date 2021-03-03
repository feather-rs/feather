use bytemuck::{Pod, Zeroable};
/// Marker component for llama spit entities.
///
/// # Example
/// A system that queries for all llama spits:
/// ```no_run
/// use quill::{Game, Position, entities::LlamaSpit};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &LlamaSpit)>() {
///         println!("Found a llama spit with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct LlamaSpit;

pod_component_impl!(LlamaSpit);
