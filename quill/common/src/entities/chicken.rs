use bytemuck::{Pod, Zeroable};
/// Marker component for chicken entities.
///
/// # Example
/// A system that queries for all chickens:
/// ```no_run
/// use quill::{Game, Position, entities::Chicken};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Chicken)>() {
///         println!("Found a chicken with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Chicken;

pod_component_impl!(Chicken);
