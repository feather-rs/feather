use bytemuck::{Pod, Zeroable};
/// Marker component for zombie villager entities.
///
/// # Example
/// A system that queries for all zombie villagers:
/// ```no_run
/// use quill::{Game, Position, entities::ZombieVillager};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ZombieVillager)>() {
///         println!("Found a zombie villager with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ZombieVillager;

pod_component_impl!(ZombieVillager);
