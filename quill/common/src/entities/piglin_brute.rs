use bytemuck::{Pod, Zeroable};
/// Marker component for piglin brute entities.
///
/// # Example
/// A system that queries for all piglin brutes:
/// ```no_run
/// use quill::{Game, Position, entities::PiglinBrute};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &PiglinBrute)>() {
///         println!("Found a piglin brute with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct PiglinBrute;

pod_component_impl!(PiglinBrute);
