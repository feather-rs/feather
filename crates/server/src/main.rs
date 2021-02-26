use std::{cell::RefCell, rc::Rc};

use common::{
    world_source::{flat::FlatWorldSource, region::RegionWorldSource, WorldSource},
    Game, TickLoop, World,
};
use ecs::SystemExecutor;
use feather_server::{Options, Server};
use plugin_host::PluginManager;

mod logging;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logging::init();

    let server = Server::bind(Options::default()).await?;
    let mut game = Game::new();

    let world_source = RegionWorldSource::new("world").with_fallback(FlatWorldSource::new());
    game.world = World::with_source(world_source);

    let mut systems = SystemExecutor::new();
    common::register(&mut game, &mut systems);
    server.link_with_game(&mut game, &mut systems);

    game.system_executor = Rc::new(RefCell::new(systems));

    let mut plugin_manager = PluginManager::new();
    plugin_manager.load_dir(&mut game, "plugins")?;
    game.insert_resource(Rc::new(RefCell::new(plugin_manager)));

    print_systems(&mut *game.system_executor.borrow_mut());

    let tick_loop = TickLoop::new(move || {
        let systems = Rc::clone(&game.system_executor);
        systems.borrow_mut().run(&mut game);

        false
    });

    tick_loop.run();

    Ok(())
}

fn print_systems(systems: &mut SystemExecutor<Game>) {
    let systems: Vec<&str> = systems.system_names().collect();
    log::debug!("---SYSTEMS---\n{:#?}\n", systems);
}
