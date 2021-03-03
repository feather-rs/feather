use bytemuck::{Pod, Zeroable};
/// Marker component for skeleton horse entities.
///
/// # Example
/// A system that queries for all skeleton horses:
/// ```no_run
/// use quill::{Game, Position, entities::SkeletonHorse};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &SkeletonHorse)>() {
///         println!("Found a skeleton horse with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct SkeletonHorse;

pod_component_impl!(SkeletonHorse);
