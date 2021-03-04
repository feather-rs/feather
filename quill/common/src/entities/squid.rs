use bytemuck::{Pod, Zeroable};
/// Marker component for squid entities.
///
/// # Example
/// A system that queries for all squids:
/// ```no_run
/// use quill::{Game, Position, entities::Squid};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Squid)>() {
///         println!("Found a squid with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Squid;

pod_component_impl!(Squid);
