use bytemuck::{Pod, Zeroable};
/// Marker component for spectral arrow entities.
///
/// # Example
/// A system that queries for all spectral arrows:
/// ```no_run
/// use quill::{Game, Position, entities::SpectralArrow};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &SpectralArrow)>() {
///         println!("Found a spectral arrow with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct SpectralArrow;

pod_component_impl!(SpectralArrow);
