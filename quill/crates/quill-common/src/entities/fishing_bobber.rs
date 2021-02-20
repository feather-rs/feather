use bytemuck::{Pod, Zeroable};
/// Marker component for fishing bobber entities.
///
/// # Example
/// A system that queries for all fishing bobbers:
/// ```no_run
/// use quill::{Game, Position, entities::FishingBobber};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &FishingBobber)>() {
///         println!("Found a fishing bobber with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct FishingBobber;

pod_component_impl!(FishingBobber);
