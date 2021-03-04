use bytemuck::{Pod, Zeroable};
/// Marker component for painting entities.
///
/// # Example
/// A system that queries for all paintings:
/// ```no_run
/// use quill::{Game, Position, entities::Painting};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Painting)>() {
///         println!("Found a painting with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Painting;

pod_component_impl!(Painting);
