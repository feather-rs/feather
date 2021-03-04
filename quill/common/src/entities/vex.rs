use bytemuck::{Pod, Zeroable};
/// Marker component for vex entities.
///
/// # Example
/// A system that queries for all vexs:
/// ```no_run
/// use quill::{Game, Position, entities::Vex};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Vex)>() {
///         println!("Found a vex with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Vex;

pod_component_impl!(Vex);
