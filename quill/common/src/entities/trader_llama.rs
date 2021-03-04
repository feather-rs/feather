use bytemuck::{Pod, Zeroable};
/// Marker component for trader llama entities.
///
/// # Example
/// A system that queries for all trader llamas:
/// ```no_run
/// use quill::{Game, Position, entities::TraderLlama};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &TraderLlama)>() {
///         println!("Found a trader llama with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct TraderLlama;

pod_component_impl!(TraderLlama);
