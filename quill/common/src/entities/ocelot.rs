use bytemuck::{Pod, Zeroable};
/// Marker component for ocelot entities.
///
/// # Example
/// A system that queries for all ocelots:
/// ```no_run
/// use quill::{Game, Position, entities::Ocelot};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Ocelot)>() {
///         println!("Found a ocelot with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Ocelot;

pod_component_impl!(Ocelot);
