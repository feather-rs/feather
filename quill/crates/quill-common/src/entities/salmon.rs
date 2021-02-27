use bytemuck::{Pod, Zeroable};
/// Marker component for salmon entities.
///
/// # Example
/// A system that queries for all salmons:
/// ```no_run
/// use quill::{Game, Position, entities::Salmon};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Salmon)>() {
///         println!("Found a salmon with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Salmon;

pod_component_impl!(Salmon);
