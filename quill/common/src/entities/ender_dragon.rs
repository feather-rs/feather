use bytemuck::{Pod, Zeroable};
/// Marker component for ender dragon entities.
///
/// # Example
/// A system that queries for all ender dragons:
/// ```no_run
/// use quill::{Game, Position, entities::EnderDragon};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &EnderDragon)>() {
///         println!("Found a ender dragon with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct EnderDragon;

pod_component_impl!(EnderDragon);
