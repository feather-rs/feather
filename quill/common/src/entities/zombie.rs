use bytemuck::{Pod, Zeroable};
/// Marker component for zombie entities.
///
/// # Example
/// A system that queries for all zombies:
/// ```no_run
/// use quill::{Game, Position, entities::Zombie};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Zombie)>() {
///         println!("Found a zombie with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Zombie;

pod_component_impl!(Zombie);
