use bytemuck::{Pod, Zeroable};
/// Marker component for wandering trader entities.
///
/// # Example
/// A system that queries for all wandering traders:
/// ```no_run
/// use quill::{Game, Position, entities::WanderingTrader};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &WanderingTrader)>() {
///         println!("Found a wandering trader with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct WanderingTrader;

pod_component_impl!(WanderingTrader);
