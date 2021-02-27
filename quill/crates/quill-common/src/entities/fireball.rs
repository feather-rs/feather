use bytemuck::{Pod, Zeroable};
/// Marker component for fireball entities.
///
/// # Example
/// A system that queries for all fireballs:
/// ```no_run
/// use quill::{Game, Position, entities::Fireball};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Fireball)>() {
///         println!("Found a fireball with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Fireball;

pod_component_impl!(Fireball);
