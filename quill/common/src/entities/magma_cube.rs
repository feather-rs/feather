use bytemuck::{Pod, Zeroable};
/// Marker component for magma cube entities.
///
/// # Example
/// A system that queries for all magma cubes:
/// ```no_run
/// use quill::{Game, Position, entities::MagmaCube};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &MagmaCube)>() {
///         println!("Found a magma cube with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct MagmaCube;

pod_component_impl!(MagmaCube);
