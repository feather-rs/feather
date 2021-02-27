use bytemuck::{Pod, Zeroable};
/// Marker component for llama entities.
///
/// # Example
/// A system that queries for all llamas:
/// ```no_run
/// use quill::{Game, Position, entities::Llama};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Llama)>() {
///         println!("Found a llama with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Llama;

pod_component_impl!(Llama);
