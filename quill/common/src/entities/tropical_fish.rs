use bytemuck::{Pod, Zeroable};
/// Marker component for tropical fish entities.
///
/// # Example
/// A system that queries for all tropical fishs:
/// ```no_run
/// use quill::{Game, Position, entities::TropicalFish};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &TropicalFish)>() {
///         println!("Found a tropical fish with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct TropicalFish;

pod_component_impl!(TropicalFish);
