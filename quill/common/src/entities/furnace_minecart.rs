use bytemuck::{Pod, Zeroable};
/// Marker component for furnace minecart entities.
///
/// # Example
/// A system that queries for all furnace minecarts:
/// ```no_run
/// use quill::{Game, Position, entities::FurnaceMinecart};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &FurnaceMinecart)>() {
///         println!("Found a furnace minecart with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct FurnaceMinecart;

pod_component_impl!(FurnaceMinecart);
