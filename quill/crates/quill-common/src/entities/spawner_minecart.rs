use bytemuck::{Pod, Zeroable};
/// Marker component for spawner minecart entities.
///
/// # Example
/// A system that queries for all spawner minecarts:
/// ```no_run
/// use quill::{Game, Position, entities::SpawnerMinecart};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &SpawnerMinecart)>() {
///         println!("Found a spawner minecart with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct SpawnerMinecart;

pod_component_impl!(SpawnerMinecart);
