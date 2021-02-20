use bytemuck::{Pod, Zeroable};
/// Marker component for trident entities.
///
/// # Example
/// A system that queries for all tridents:
/// ```no_run
/// use quill::{Game, Position, entities::Trident};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Trident)>() {
///         println!("Found a trident with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Trident;

pod_component_impl!(Trident);
