use bytemuck::{Pod, Zeroable};
/// Marker component for bat entities.
///
/// # Example
/// A system that queries for all bats:
/// ```no_run
/// use quill::{Game, Position, entities::Bat};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Bat)>() {
///         println!("Found a bat with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Bat;

pod_component_impl!(Bat);
