use bytemuck::{Pod, Zeroable};
/// Marker component for pillager entities.
///
/// # Example
/// A system that queries for all pillagers:
/// ```no_run
/// use quill::{Game, Position, entities::Pillager};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Pillager)>() {
///         println!("Found a pillager with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Pillager;

pod_component_impl!(Pillager);
