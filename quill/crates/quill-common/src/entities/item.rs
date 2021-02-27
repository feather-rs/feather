use bytemuck::{Pod, Zeroable};
/// Marker component for item entities.
///
/// # Example
/// A system that queries for all items:
/// ```no_run
/// use quill::{Game, Position, entities::Item};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Item)>() {
///         println!("Found a item with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Item;

pod_component_impl!(Item);
