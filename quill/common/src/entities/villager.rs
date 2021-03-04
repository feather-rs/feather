use bytemuck::{Pod, Zeroable};
/// Marker component for villager entities.
///
/// # Example
/// A system that queries for all villagers:
/// ```no_run
/// use quill::{Game, Position, entities::Villager};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Villager)>() {
///         println!("Found a villager with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Villager;

pod_component_impl!(Villager);
