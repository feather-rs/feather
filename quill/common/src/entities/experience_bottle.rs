use bytemuck::{Pod, Zeroable};
/// Marker component for experience bottle entities.
///
/// # Example
/// A system that queries for all experience bottles:
/// ```no_run
/// use quill::{Game, Position, entities::ExperienceBottle};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ExperienceBottle)>() {
///         println!("Found a experience bottle with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ExperienceBottle;

pod_component_impl!(ExperienceBottle);
