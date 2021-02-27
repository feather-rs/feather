use bytemuck::{Pod, Zeroable};
/// Marker component for potion entities.
///
/// # Example
/// A system that queries for all potions:
/// ```no_run
/// use quill::{Game, Position, entities::Potion};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Potion)>() {
///         println!("Found a potion with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Potion;

pod_component_impl!(Potion);
