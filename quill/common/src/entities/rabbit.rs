use bytemuck::{Pod, Zeroable};
/// Marker component for rabbit entities.
///
/// # Example
/// A system that queries for all rabbits:
/// ```no_run
/// use quill::{Game, Position, entities::Rabbit};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Rabbit)>() {
///         println!("Found a rabbit with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Rabbit;

pod_component_impl!(Rabbit);
