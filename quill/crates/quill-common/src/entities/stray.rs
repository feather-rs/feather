use bytemuck::{Pod, Zeroable};
/// Marker component for stray entities.
///
/// # Example
/// A system that queries for all strays:
/// ```no_run
/// use quill::{Game, Position, entities::Stray};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Stray)>() {
///         println!("Found a stray with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Stray;

pod_component_impl!(Stray);
