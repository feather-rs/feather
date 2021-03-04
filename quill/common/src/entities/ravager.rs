use bytemuck::{Pod, Zeroable};
/// Marker component for ravager entities.
///
/// # Example
/// A system that queries for all ravagers:
/// ```no_run
/// use quill::{Game, Position, entities::Ravager};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Ravager)>() {
///         println!("Found a ravager with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Ravager;

pod_component_impl!(Ravager);
