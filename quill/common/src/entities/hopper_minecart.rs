use bytemuck::{Pod, Zeroable};
/// Marker component for hopper minecart entities.
///
/// # Example
/// A system that queries for all hopper minecarts:
/// ```no_run
/// use quill::{Game, Position, entities::HopperMinecart};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &HopperMinecart)>() {
///         println!("Found a hopper minecart with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct HopperMinecart;

pod_component_impl!(HopperMinecart);
