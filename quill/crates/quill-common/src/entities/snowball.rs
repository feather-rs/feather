use bytemuck::{Pod, Zeroable};
/// Marker component for snowball entities.
///
/// # Example
/// A system that queries for all snowballs:
/// ```no_run
/// use quill::{Game, Position, entities::Snowball};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Snowball)>() {
///         println!("Found a snowball with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Snowball;

pod_component_impl!(Snowball);
