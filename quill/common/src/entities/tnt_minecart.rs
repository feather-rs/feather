use bytemuck::{Pod, Zeroable};
/// Marker component for tnt minecart entities.
///
/// # Example
/// A system that queries for all tnt minecarts:
/// ```no_run
/// use quill::{Game, Position, entities::TntMinecart};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &TntMinecart)>() {
///         println!("Found a tnt minecart with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct TntMinecart;

pod_component_impl!(TntMinecart);
