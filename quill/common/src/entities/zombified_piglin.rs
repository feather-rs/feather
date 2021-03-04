use bytemuck::{Pod, Zeroable};
/// Marker component for zombified piglin entities.
///
/// # Example
/// A system that queries for all zombified piglins:
/// ```no_run
/// use quill::{Game, Position, entities::ZombifiedPiglin};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ZombifiedPiglin)>() {
///         println!("Found a zombified piglin with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ZombifiedPiglin;

pod_component_impl!(ZombifiedPiglin);
