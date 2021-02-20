use bytemuck::{Pod, Zeroable};
/// Marker component for firework rocket entities.
///
/// # Example
/// A system that queries for all firework rockets:
/// ```no_run
/// use quill::{Game, Position, entities::FireworkRocket};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &FireworkRocket)>() {
///         println!("Found a firework rocket with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct FireworkRocket;

pod_component_impl!(FireworkRocket);
