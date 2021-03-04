use bytemuck::{Pod, Zeroable};
/// Marker component for cow entities.
///
/// # Example
/// A system that queries for all cows:
/// ```no_run
/// use quill::{Game, Position, entities::Cow};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Cow)>() {
///         println!("Found a cow with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Cow;

pod_component_impl!(Cow);
