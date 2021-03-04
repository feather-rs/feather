use bytemuck::{Pod, Zeroable};
/// Marker component for horse entities.
///
/// # Example
/// A system that queries for all horses:
/// ```no_run
/// use quill::{Game, Position, entities::Horse};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Horse)>() {
///         println!("Found a horse with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Horse;

pod_component_impl!(Horse);
