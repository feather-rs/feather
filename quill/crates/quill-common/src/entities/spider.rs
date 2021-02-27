use bytemuck::{Pod, Zeroable};
/// Marker component for spider entities.
///
/// # Example
/// A system that queries for all spiders:
/// ```no_run
/// use quill::{Game, Position, entities::Spider};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Spider)>() {
///         println!("Found a spider with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Spider;

pod_component_impl!(Spider);
