use bytemuck::{Pod, Zeroable};
/// Marker component for item frame entities.
///
/// # Example
/// A system that queries for all item frames:
/// ```no_run
/// use quill::{Game, Position, entities::ItemFrame};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ItemFrame)>() {
///         println!("Found a item frame with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ItemFrame;

pod_component_impl!(ItemFrame);
