use bytemuck::{Pod, Zeroable};
/// Marker component for cod entities.
///
/// # Example
/// A system that queries for all cods:
/// ```no_run
/// use quill::{Game, Position, entities::Cod};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Cod)>() {
///         println!("Found a cod with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Cod;

pod_component_impl!(Cod);
