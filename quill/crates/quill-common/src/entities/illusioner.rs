use bytemuck::{Pod, Zeroable};
/// Marker component for illusioner entities.
///
/// # Example
/// A system that queries for all illusioners:
/// ```no_run
/// use quill::{Game, Position, entities::Illusioner};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Illusioner)>() {
///         println!("Found a illusioner with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Illusioner;

pod_component_impl!(Illusioner);
