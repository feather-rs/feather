use bytemuck::{Pod, Zeroable};
/// Marker component for experience orb entities.
///
/// # Example
/// A system that queries for all experience orbs:
/// ```no_run
/// use quill::{Game, Position, entities::ExperienceOrb};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ExperienceOrb)>() {
///         println!("Found a experience orb with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ExperienceOrb;

pod_component_impl!(ExperienceOrb);
