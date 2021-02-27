use bytemuck::{Pod, Zeroable};
/// Marker component for zoglin entities.
///
/// # Example
/// A system that queries for all zoglins:
/// ```no_run
/// use quill::{Game, Position, entities::Zoglin};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Zoglin)>() {
///         println!("Found a zoglin with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Zoglin;

pod_component_impl!(Zoglin);
