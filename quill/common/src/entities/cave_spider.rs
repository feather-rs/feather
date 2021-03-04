use bytemuck::{Pod, Zeroable};
/// Marker component for cave spider entities.
///
/// # Example
/// A system that queries for all cave spiders:
/// ```no_run
/// use quill::{Game, Position, entities::CaveSpider};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &CaveSpider)>() {
///         println!("Found a cave spider with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct CaveSpider;

pod_component_impl!(CaveSpider);
