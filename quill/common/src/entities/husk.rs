use bytemuck::{Pod, Zeroable};
/// Marker component for husk entities.
///
/// # Example
/// A system that queries for all husks:
/// ```no_run
/// use quill::{Game, Position, entities::Husk};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Husk)>() {
///         println!("Found a husk with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Husk;

pod_component_impl!(Husk);
