use bytemuck::{Pod, Zeroable};
/// Marker component for skeleton entities.
///
/// # Example
/// A system that queries for all skeletons:
/// ```no_run
/// use quill::{Game, Position, entities::Skeleton};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Skeleton)>() {
///         println!("Found a skeleton with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Skeleton;

pod_component_impl!(Skeleton);
