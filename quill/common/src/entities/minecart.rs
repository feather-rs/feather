use bytemuck::{Pod, Zeroable};
/// Marker component for minecart entities.
///
/// # Example
/// A system that queries for all minecarts:
/// ```no_run
/// use quill::{Game, Position, entities::Minecart};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Minecart)>() {
///         println!("Found a minecart with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Minecart;

pod_component_impl!(Minecart);
