use bytemuck::{Pod, Zeroable};
/// Marker component for polar bear entities.
///
/// # Example
/// A system that queries for all polar bears:
/// ```no_run
/// use quill::{Game, Position, entities::PolarBear};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &PolarBear)>() {
///         println!("Found a polar bear with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct PolarBear;

pod_component_impl!(PolarBear);
