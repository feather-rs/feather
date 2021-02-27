use bytemuck::{Pod, Zeroable};
/// Marker component for donkey entities.
///
/// # Example
/// A system that queries for all donkeys:
/// ```no_run
/// use quill::{Game, Position, entities::Donkey};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Donkey)>() {
///         println!("Found a donkey with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Donkey;

pod_component_impl!(Donkey);
