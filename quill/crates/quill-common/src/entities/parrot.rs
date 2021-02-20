use bytemuck::{Pod, Zeroable};
/// Marker component for parrot entities.
///
/// # Example
/// A system that queries for all parrots:
/// ```no_run
/// use quill::{Game, Position, entities::Parrot};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Parrot)>() {
///         println!("Found a parrot with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Parrot;

pod_component_impl!(Parrot);
