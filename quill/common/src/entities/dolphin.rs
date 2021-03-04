use bytemuck::{Pod, Zeroable};
/// Marker component for dolphin entities.
///
/// # Example
/// A system that queries for all dolphins:
/// ```no_run
/// use quill::{Game, Position, entities::Dolphin};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Dolphin)>() {
///         println!("Found a dolphin with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Dolphin;

pod_component_impl!(Dolphin);
