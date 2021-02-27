use bytemuck::{Pod, Zeroable};
/// Marker component for elder guardian entities.
///
/// # Example
/// A system that queries for all elder guardians:
/// ```no_run
/// use quill::{Game, Position, entities::ElderGuardian};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ElderGuardian)>() {
///         println!("Found a elder guardian with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ElderGuardian;

pod_component_impl!(ElderGuardian);
