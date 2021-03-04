use bytemuck::{Pod, Zeroable};
/// Marker component for falling block entities.
///
/// # Example
/// A system that queries for all falling blocks:
/// ```no_run
/// use quill::{Game, Position, entities::FallingBlock};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &FallingBlock)>() {
///         println!("Found a falling block with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct FallingBlock;

pod_component_impl!(FallingBlock);
