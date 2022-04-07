use anyhow::Context;
use common::Game;
use quill::{Plugin, SysResult};

struct Setup<'a> {
    game: &'a mut Game,
}

impl<'a> quill::Setup for Setup<'a> {
    fn register_system(
        &mut self,
        system: fn(&mut dyn quill::Game) -> quill::SysResult,
        name: &str,
    ) {
        self.game
            .system_executor
            .borrow_mut()
            .add_system_with_name(move |game| system(game), name);
    }

    fn game(&self) -> &dyn quill::Game {
        self.game
    }

    fn game_mut(&mut self) -> &mut dyn quill::Game {
         self.game
    }
}

pub fn initialize_plugin<P: Plugin>(game: &mut Game) -> SysResult {
    log::info!("Initializing plugin {}", P::name());
    let mut setup = Setup { game };
    P::initialize(&mut setup)
        .with_context(|| format!("failed to initialize plugin {}", P::name()))?;
    Ok(())
}
