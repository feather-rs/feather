//! An example plugin that uses BungeeCord's plugin messaging channel to
//! send a player to a server named lobby.
use quill::{entities::Player, BlockPosition, Game, Plugin, Position};

#[quill::plugin]
pub struct PluginMessage;

impl Plugin for PluginMessage {
    fn enable(_game: &mut quill::Game, setup: &mut quill::Setup<Self>) -> Self {
        setup.add_system(plugin_message_system);
        Self
    }

    fn disable(self, _game: &mut quill::Game) {}
}

fn plugin_message_system(_plugin: &mut PluginMessage, game: &mut Game) {
    for (entity, (_, position)) in game.query::<(&Player, &Position)>() {
        if let BlockPosition {
            x: 10..=12,
            y: _,
            z: 10..=12,
        } = position.block()
        {
            let mut data = Vec::new();
            data.extend_from_slice(&u16::to_be_bytes(7));
            data.extend_from_slice(b"Connect");
            data.extend_from_slice(&u16::to_be_bytes(5));
            data.extend_from_slice(b"lobby");
            Game::send_plugin_message(entity.id(), "bungeecord:main", &data);
        }
    }
}
