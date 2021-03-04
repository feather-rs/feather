use bytemuck::{Pod, Zeroable};
/// Marker component for zombie horse entities.
///
/// # Example
/// A system that queries for all zombie horses:
/// ```no_run
/// use quill::{Game, Position, entities::ZombieHorse};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ZombieHorse)>() {
///         println!("Found a zombie horse with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ZombieHorse;

pod_component_impl!(ZombieHorse);
